use log::debug;
use std::cmp::{max, min};
use serde::{Deserialize, Serialize};  // TODO Check if we could get Deserialized in dev dependancies

use crate::{fission, metric_prefix};


// type blocks = i32;

const GENERAL_DISPERSER_GAS_FLOW: i32 = 1280; // mB/t
const GENERAL_VENT_GAS_FLOW: i32 = 32000; // mB/t
const GENERAL_CONDENSER_RATE: i32 = 64000; // mB/t
const MAX_ENERGY_PER_STEAM: i32 = 10; // Joules/mB of steam
const TURBINE_BLADES_PER_COIL: i32 = 4;
const GAS_PER_TANK: i32 = 64000; // mB

/// Struct Turbine
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Turbine {
    pub x_z: i32,
    pub y: i32,
    pub vents: i32,
    pub dispersers: i32,
    pub condensers: i32,
    pub shaft_height: i32,
    pub blades: i32,
    pub coils: i32,
    pub capacity: i32,
    pub max_flow: i32,
    pub tank_volume: i32,
    pub max_production: f32,
    pub max_water_output: i32,
    //TODO Might need to break every water or energy value into type and value
    pub energy_si_prefix: metric_prefix::Prefix
}

impl Default for Turbine {
    fn default() -> Turbine {
        Turbine {
            x_z: 0,
            y: 0,
            vents: 0,
            dispersers: 0,
            condensers: 0,
            shaft_height: 0,
            blades: 0,
            coils: 0,
            capacity: 0,
            max_flow: 0,
            tank_volume: 0,
            max_production: 0.0,
            max_water_output: 0,
            energy_si_prefix: metric_prefix::Prefix::Base,
        }
    }
}

impl PartialEq for Turbine {
    fn eq(&self, other: &Self) -> bool {
        let converted_max_production = metric_prefix::convert_to_prefix(self.max_production, &self.energy_si_prefix, &other.energy_si_prefix);
        self.x_z == other.x_z && 
        self.y == other.y &&
        self.vents == other.vents &&
        self.dispersers == other.dispersers &&
        self.condensers == other.condensers &&
        self.shaft_height == other.shaft_height &&
        self.blades == other.blades &&
        self.coils == other.coils &&
        self.capacity == other.capacity &&
        self.max_flow == other.max_flow &&
        self.tank_volume == other.tank_volume &&
        // 5x5x5 turbine in game has 182.93 kJ where formula returns 182.95.  Drop Accuracy for now.
        // 9x9x17 turbine in game has 44,79, where fomular returned 44.8, dropping all decimals for now
        metric_prefix::drop_decimals(converted_max_production) == metric_prefix::drop_decimals(other.max_production) &&
        self.max_water_output == other.max_water_output
    }
}

impl Turbine {
    pub fn print(&self){
        println!("A {}x{}x{} Turbine", self.x_z, self.x_z, self.y);
        println!("- Shaft {}, Blades {}, and Coils {}", self.shaft_height, self.blades, self.coils);
        println!("- Vents: {}",self.vents);
        println!("- Dispersers: {}", self.dispersers);
        println!("- Condensers: {}", self.condensers);
        println!("- Max Flow Rate {} mB/t, Max Water Output {} mB /t", self.max_flow, self.max_water_output);
        println!("- Capacity {} mJ, Max Energy Production {} mJ\n", self.capacity, self.max_production);
    }

    pub fn summarize(&self) -> String{
        format!("A {}x{}x{} Turbine", self.x_z, self.x_z, self.y)
    }
}

#[derive(Deserialize, Debug)]
struct TurbineFlow {
    shaft_height: i32,
    vents: i32,
    condensers: i32,
    max_flow: i32,
    max_water_ouput: i32,
    max_energy_production: f32,
}

///  Create turbine based on all blocks/parts added.  Mostly for calculating formulas
#[allow(dead_code)]
pub fn turbine_factory(
    x_z: i32,
    y: i32,
    condensers: i32,
    dispersers: i32,
    vents: i32,
    shaft_height: i32,
    blades: i32,
    coils: i32,
) -> Turbine {
    Turbine {
        x_z,
        y,
        vents,
        dispersers,
        condensers,
        shaft_height,
        blades,
        coils,
        capacity: energy_capacity(x_z, y),  // TODO Wrong capacity is buckets not power
        max_flow: calc_max_flow_rate(x_z, shaft_height, vents),
        tank_volume: calc_lower_volume(x_z, shaft_height),
        max_water_output: max_water_output(condensers),
        ..Default::default()
    }
}

// Return most optimal turbine based on number of fuel assemblies of existing fission reactor
// pub fn turbine_based_on_fission_reactor(water_burn_rate: i32) -> Result<Turbine, &'static str> {
//     let mut turbine: Turbine = Turbine { ..Default::default() };
//     // Get Max Water Output
//     turbine.condensers = (water_burn_rate as f32 / GENERAL_CONDENSER_RATE as f32 ).ceil() as i32;
//     turbine.max_water_output = max_water_output(turbine.condensers);

//     // Get Max Water Flow, which is more effort
//     turbine.vents = (water_burn_rate as f32 / GENERAL_VENT_GAS_FLOW as f32).ceil() as i32;
//     let vent_flow = turbine.vents * GENERAL_VENT_GAS_FLOW;
//     let mut power_production = 0.0;
//     //Tank Flow needs to be calculated to get as close to vent_flow as possible
//     'outer: for length in (5..18).step_by(2) {
//         let dispersers = calc_pressure_dispersers(length);
//         // Maximum total height = min(2xLENGTH-1,18)
//         // let max_height = min(2 * length - 1, 18);
//         // Maximum shaft height = min(2xLENGTH-5,14) [so blades don't touch sides]
//         for shaft_height in 1..min(2 * length - 5, 14) {
//             let tank_flow = dispersers * GENERAL_DISPERSER_GAS_FLOW * calc_lower_volume(length, shaft_height);
//             let blades = shaft_height * 2;
//             let coils = calc_coils_needed(blades);
//             let current_power = max_energy_prod(blades, coils, length, shaft_height, turbine.vents);
//             debug!("Tank Flow {tank_flow}, Vent Flow {vent_flow}");
//             debug!("Length: {length}/{shaft_height}, Old Power {power_production}, current power {current_power}");
//             // TODO This doesn't optimize power production
//             if tank_flow >= vent_flow {
//                 turbine.x_z = length;
//                 turbine.dispersers = dispersers;
//                 turbine.shaft_height = shaft_height;
//                 break 'outer;
//             } else {
//                 power_production = current_power;
//             }
//         }
//     }

// Return most optimal turbine based on number of fuel assemblies of existing fission reactor
pub fn turbine_based_on_fission_reactor(water_burn_rate: i32) -> Result<Turbine, &'static str> {
    let mut turbine: Turbine = Turbine { ..Default::default() };
    // Get Max Water Output
    let required_condensers = (water_burn_rate as f32 / GENERAL_CONDENSER_RATE as f32 ).ceil() as i32;
    // Get Max Water Flow, which is more effort
    turbine.vents = (water_burn_rate as f32 / GENERAL_VENT_GAS_FLOW as f32).ceil() as i32;
    let mut all_shaft_heights: Vec<Turbine> = Vec::new();
    //Tank Flow needs to be calculated to get as close to vent_flow as possible
    for length in (5..18).step_by(2) {
        let dispersers = calc_pressure_dispersers(length);
        // Maximum total height = min(2xLENGTH-1,18)
        // let max_height = min(2 * length - 1, 18);
        // Maximum shaft height = min(2xLENGTH-5,14) [so blades don't touch sides]
        let mut shaft_heights: Vec<Turbine> = (1..min(2 * length - 5, 14)).map(| shaft_height | {
            let mut temp_turbine: Turbine = Turbine { ..Default::default() };
            // let tank_flow = calc_tank_flow_rate(length, shaft_height);
            temp_turbine.x_z = length;
            temp_turbine.shaft_height = shaft_height;
            temp_turbine.dispersers = dispersers;
            temp_turbine.blades = shaft_height * 2;
            temp_turbine.coils = calc_coils_needed(temp_turbine.blades);
            temp_turbine.vents = turbine.vents;
            return temp_turbine;
            
        }).collect();
        if shaft_heights.len() == 0 {
            println!("all shaft heights is empty.");
            continue;
        }
        all_shaft_heights.append(& mut shaft_heights);
    }
    // TODO if the vec is empty should error of function
    if all_shaft_heights.len() == 0 {
        panic!("No turbines to return!")
    }
    // For debugging
    for t in all_shaft_heights.iter_mut() {
        t.y = min_height(t.shaft_height, t.coils, t.condensers, t.x_z, t.vents);
        t.max_production = max_energy_prod(t.blades, t.coils, t.x_z, t.shaft_height, t.vents);
        t.max_flow = calc_max_flow_rate(t.x_z, t.shaft_height, t.vents);
        t.condensers = calc_optimal_condensers(t.x_z, t.y, t.shaft_height, t.coils, t.max_flow);
        t.max_water_output = max_water_output(t.condensers);
        t.capacity = steam_capacity(t.x_z, t.shaft_height);
        t.tank_volume = calc_lower_volume(t.x_z, t.shaft_height);
    }
    for t in all_shaft_heights.iter() {
        println!("Turbines {:?}", t);
    }
    // Remove turbines that don't meet min max flow, output, or condensers
    all_shaft_heights.retain( | t | {
        let (best_vent_count, _best_energy_count) = best_vent_count(&t);

        min(t.max_flow, t.max_water_output) >= water_burn_rate &&
        t.condensers >= required_condensers &&
        best_vent_count <= t.vents
    });
    // Remove turbines that make less power and are larger than smaller more efficient turbines.
    let mut index_to_remove = Vec::new();
    'outer: for i in 1..all_shaft_heights.len() {
        let turbine = &all_shaft_heights[i];
        'inner: for j in 1..all_shaft_heights.len() {
            if i == j {
                continue 'inner;
            }
            let other_turbine = &all_shaft_heights[j];
            if turbine.x_z > other_turbine.x_z {
                if turbine.max_production < other_turbine.max_production {
                    index_to_remove.push(i);
                    continue 'outer;
                }
            }
        }
    }
    for i in index_to_remove.iter().rev() {
        all_shaft_heights.remove(*i);
    }
    
    let best_turbine = all_shaft_heights.iter().max_by_key( | turbine | turbine.max_production.round() as i32).unwrap().clone();
    debug!("Best turbine:\n{:?}\n-------------------------------------", best_turbine);
    Ok(best_turbine)
}


fn min_height(shaft_height: i32, coils: i32, condensers: i32, x_z: i32, vents: i32)  -> i32 {
    for y in (shaft_height + 3)..18 {
        let upper_y = y - shaft_height - 2;
        let internal_volume = (upper_y - 1) * (x_z - 2).pow(2);
        // Check if internal area big enough for all vents
        if internal_volume < (coils + condensers) {
            continue;
        }
        // Check if internal area big enough for both coils and condensers
        let side_area = upper_y * (x_z - 2) * 4;
        let top_area = (x_z - 2).pow(2);
        if (side_area + top_area) >= vents {
            return y;
        }
    }
    return 0;
}

//FLOW = min(1, TURBINE_STORED_AMOUNT / MAX_RATE) *
//          (TURBINE_STORED_AMOUNT/TURBINE_MAX_STORED_AMOUNT) * MAX_RATE

///  Return most optimal turbine only based on user inputing dimensions
pub fn optimal_turbine_with_dimensions(x_z: i32, y: i32) -> Result<Turbine, &'static str> {
    let mut turbine = Turbine { ..Default::default() };
    // Check if turbine's dimensions fall within an acceptable size
    if x_z < 5 {
        return Err("Turbine length and width too small, min 5 by 5 blocks.");
    } else if 17 < x_z {
        return Err("Turbine length and width too large, max 17 by 17 blocks.");
    }

    if y < 5 {
        return Err("Turbine height too small, min 5 blocks.");
    } else if 18 < y {
        return Err("Turbine height too large, max 18 blocks.");
    }
    // Length can't be even
    if x_z % 2 == 0 {
        return Err("Turbine Length can't be even, otherwise shaft can't be in center.");
    }
    // Calculate the max flow, and max water output for each shaft_height of the turbine.
    let info: Vec<TurbineFlow> = (1..min(2 * y - 5, 14))
        .map(|shaft_height: i32| {
            // Block constants
            let blades = shaft_height * 2;
            let coils = calc_coils_needed(blades);
            // Calculated Rates
            // let tank_flow = calc_tank_flow_rate(x_z, shaft_height);  // TODO Not used
            let temp_turbine = Turbine {
                x_z,
                y,
                shaft_height,
                blades,
                coils,
                ..Default::default()
            };
            let (best_vent_count, best_energy_production) = best_vent_count(&temp_turbine);
            //TODO Now figure out what the best number of condensors would be
            let max_flow = calc_max_flow_rate(x_z, shaft_height, best_vent_count);
            let condensers =
                calc_optimal_condensers(x_z, y, shaft_height, shaft_height * 2, max_flow);
            let water_output = max_water_output(condensers);
            let max_energy_production = best_energy_production;
            let this = TurbineFlow {
                shaft_height,
                vents: best_vent_count,
                condensers,
                max_flow,
                max_water_ouput: water_output,
                max_energy_production,
            };
            // println!("{:?}", this);
            return this;
        })
        .filter(|x| x.condensers > 0)
        .collect::<Vec<TurbineFlow>>();

    // TODO Man I hate this calculation, it should be done better.
    // Find the ideal ratio of vents vs volume
    // Maximum shaft height = min(2xLENGTH-5,14) [so blades don't touch sides]
    // maxrate, vent_count, shaft_height

    let best_turbine = info.iter().max_by_key(|x| x.max_energy_production.round() as i32).unwrap();
    turbine.shaft_height = best_turbine.shaft_height;
    turbine.max_flow = best_turbine.max_flow;
    turbine.vents = best_turbine.vents;
    turbine.coils = calc_coils_needed(turbine.shaft_height * 2);
    turbine.x_z = x_z;
    turbine.y = y;
    turbine.blades = turbine.shaft_height * 2;
    turbine.dispersers = calc_pressure_dispersers(x_z);
    turbine.condensers = calc_optimal_condensers(x_z, y, turbine.shaft_height, turbine.coils, turbine.max_flow);
    turbine.tank_volume = calc_lower_volume(x_z, turbine.shaft_height);
    turbine.max_production = max_energy_prod(turbine.shaft_height * 2, turbine.coils, x_z, turbine.shaft_height, turbine.vents);
    turbine.max_water_output = max_water_output(turbine.condensers);
    turbine.capacity = steam_capacity(turbine.x_z, turbine.shaft_height);
    Ok(turbine)
}

fn best_vent_count(turbine: &Turbine) -> (i32, f32) {
    let mut best_vent_count = 0;
    let mut best_energy_production = 0.0;
    // Find the vent_count/vent flow closest to the tank flow.
    for vent_count in 1..calc_max_vents(  turbine.x_z, turbine.y, turbine.shaft_height) + 1 {
        let max_flow = calc_max_flow_rate(turbine.x_z, turbine.shaft_height, vent_count);
        let condensers = calc_optimal_condensers(turbine.x_z, turbine.y, turbine.shaft_height, turbine.coils, max_flow);
        if condensers < 0 {
            continue;
        }
        let max_energy_prod = max_energy_prod(turbine.blades, turbine.coils, turbine.x_z, turbine.shaft_height, vent_count);
        if max_energy_prod > best_energy_production {
            best_energy_production = max_energy_prod;
            best_vent_count = vent_count;
        }
    }
    (best_vent_count, best_energy_production)
}

// Max Flow Rate
//MAX_RATE = min(TURBINE_DISPERSER_COUNT * GENERAL_DISPERSER_GAS_FLOW * structure.lowerVolume,
//               TURBINE_VENT_COUNT * GENERAL_VENT_GAS_FLOW)
#[allow(dead_code)]
fn calc_max_flow_rate(x_z: i32, shaft_height: i32, vent_count: i32) -> i32 {
    let tank_flow: i32 = calc_pressure_dispersers(x_z)
        * GENERAL_DISPERSER_GAS_FLOW
        * calc_lower_volume(x_z, shaft_height);
    let vent_flow: i32 = vent_count * GENERAL_VENT_GAS_FLOW;
    debug!("Tank flow: {tank_flow}");
    debug!("Vent flow: {vent_flow}");
    min(tank_flow, vent_flow)
}

/// Calculate the lower tank's flow rate
fn calc_tank_flow_rate(x_z: i32, shaft_height: i32) -> i32 {
    calc_pressure_dispersers(x_z)
        * GENERAL_DISPERSER_GAS_FLOW
        * calc_lower_volume(x_z, shaft_height)
}

/// Calculate the flow rate of the vents
#[allow(dead_code)]
fn calc_vent_flow_rate(vent_count: i32) -> i32 {
    vent_count * GENERAL_VENT_GAS_FLOW
}

/// Calculate the max number of vents
fn calc_max_vents(x_z: i32, y: i32, shaft_height: i32) -> i32 {
    let remaining_height = y - 2 - shaft_height;
    // TODO Maybe better to handle this with exceptions and only use unsigned ints.
    if remaining_height <= 0 {
        return 0
    }
    let top_vents = (x_z - 2).pow(2);
    let side_vents = max((remaining_height * (x_z - 2)) * 4, 0);
    top_vents + side_vents
}
#[allow(dead_code)]
fn calc_lower_volume(x_z: i32, shaft_height: i32) -> i32 {
    (x_z).pow(2) * shaft_height
}

///
#[allow(dead_code)]
fn calc_coils_needed(num_blades: i32) -> i32 {
    max((num_blades as f32 / 4.0).ceil() as i32, 2)
}

///
#[allow(dead_code)]
fn calc_pressure_dispersers(x_z: i32) -> i32 {
    (x_z - 2).pow(2) - 1
}

/// Storing(J) = TurbineWidth² × TurbineHeight × 16,000
#[allow(dead_code)] 
fn energy_capacity(x_z: i32, shaft_height: i32) -> i32 {
    // TODO Move magic number 16000 into const
    x_z.pow(2) * shaft_height * 16000
}

/// Capacity(mB) = TurbineWidth² × TurbineRotorNumber x GAS_PER_TANK(64,000)
fn steam_capacity(x_z: i32, shaft_height: i32) -> i32 {
    x_z.pow(2) * shaft_height * GAS_PER_TANK
}

// fn get_production_rate() {
//     let mut energy_multiplier: f32 = (MAX_ENERGY_PER_STEAM as f32 / blades as f32);
//     energy_multiplier = energy_multiplier * min(blades, coils * TURBINE_BLADES_PER_COIL) as f32;
//     return energy_multiplier
// }

// https://github.com/mekanism/Mekanism/blob/d22f6e2028009ed043f8b40c4ea1f7912be3002c/src/generators/java/mekanism/generators/common/content/turbine/TurbineMultiblockData.java#L244
///  Production(J) = maxEnergyPerSteam x BladeRate x SteamFlow
#[allow(dead_code)]
fn max_energy_prod(blades: i32, coils: i32, x_z: i32, shaft_height: i32, vents: i32) -> f32 {
    MAX_ENERGY_PER_STEAM as f32
        * blade_rate(blades, coils) as f32
        * calc_max_flow_rate(x_z, shaft_height, vents) as f32
}

#[allow(dead_code)]
fn blade_rate(blades: i32, coils: i32) -> f32 {
    let blade_rate_1 = blades as f32 / 28.0;
    let blade_rate_2 = (coils * TURBINE_BLADES_PER_COIL) as f32 / 28.0;
    if blade_rate_1 < blade_rate_2 {
        return blade_rate_1;
    } else {
        return blade_rate_2;
    }
}

fn calc_optimal_condensers(x_z: i32, y: i32, shaft_height: i32, coils: i32, max_flow: i32) -> i32 {
    debug!("y: {y}, shaft_height: {shaft_height}");
    let remaining_y = (y - 3) - shaft_height;
    let avaliable_space = remaining_y * (x_z - 2).pow(2) - coils;
    debug!("{remaining_y}");
    debug!("{avaliable_space}");
    min((max_flow as f32 / GENERAL_CONDENSER_RATE as f32).ceil() as i32, avaliable_space)
}

#[allow(dead_code)]
fn max_water_output(condensers: i32) -> i32 {
    debug!("Condensers: {condensers}");
    condensers * GENERAL_CONDENSER_RATE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fission, utils};

    //Arrange
    //Act
    //Assert

    #[test]
    fn test_calc_coils_needed() {
        let blades = 10;
        let expected_coils = 3;
        assert_eq!(calc_coils_needed(blades), expected_coils);
    }

    #[test]
    fn test_calc_pressure_dispersers() {
        let x_z = 5;
        let expected = 8;
        assert_eq!(calc_pressure_dispersers(x_z), expected);
    }

    #[test]
    fn test_calc_max_vents() {
        // A 5x5x5 with shaft of 1 tall could have a max vent of 33
        let x_z = 5;
        let y = 5;
        let shaft_height = 1;
        let expected = 33;
        let actual = calc_max_vents(x_z, y, shaft_height);
        assert_eq!(actual, expected);
        // A 9x9x11 with shaft 5 tall could have max vents of 161
        let x_z = 7;
        let y = 13;
        let shaft_height = 6;
        let expected = 125;
        let actual = calc_max_vents(x_z, y, shaft_height);
        assert_eq!(actual, expected);
        // A 9x9x11 with shaft 5 tall could have max vents of 161
        let x_z = 9;
        let y = 11;
        let shaft_height = 5;
        let expected = 161;
        let actual = calc_max_vents(x_z, y, shaft_height);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max_energy_production() {
        // 5x5x9
        let expected = Turbine {
            x_z: 5,
            vents: 32,
            shaft_height: 4,
            blades: 8,
            coils: 2,
            max_production: 2.92,      //MJ
            ..Default::default()
        };
        let actual = max_energy_prod(
            expected.blades,
            expected.coils,
            expected.x_z,
            expected.shaft_height,
            expected.vents,
        );
        assert_eq!(metric_prefix::convert_to_mega(actual), expected.max_production);
        //9x9x17
        let expected = Turbine {
            x_z: 9,
            vents: 245,
            shaft_height: 8,
            blades: 16,
            coils: 4,
            max_production: 44.80, //MJ
            ..Default::default()
        };
        let actual = max_energy_prod(
            expected.blades,
            expected.coils,
            expected.x_z,
            expected.shaft_height,
            expected.vents,
        );
        assert_eq!(metric_prefix::convert_to_mega(actual), expected.max_production);
        //17x17x18
        let expected = Turbine {
            x_z: 17,
            vents: 585,
            shaft_height: 10,
            blades: 20,
            coils: 5,
            max_production: 133.71, //MJ
            ..Default::default()
        };
        let actual = max_energy_prod(
            expected.blades,
            expected.coils,
            expected.x_z,
            expected.shaft_height,
            expected.vents,
        );
        assert_eq!(metric_prefix::convert_to_mega(actual), expected.max_production);
    }

    #[test]
    fn test_turbine_factory() {
        let actual = turbine_factory(9, 11, 48, 48, 105, 5, 10, 2);
        // assert_eq!(actual.capacity, 25920000);
        assert_eq!(actual.max_flow, 3360000);
        assert_eq!(actual.tank_volume, 405);
        assert_eq!(actual.dispersers, 48);
        assert_eq!(actual.vents, 105);
        assert_eq!(actual.coils, 2);
        // assert_eq!(actual.max_production, 3.83);
        assert_eq!(actual.max_water_output, 3072000);
    }

    #[test]
    fn test_optimal_turbine_with_dimensions() {
        // 5x5x5 Turbine
        let expected = utils::get_optimal_turbine(5,5);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        // 5x5x9 Turbine
        let expected = utils::get_optimal_turbine(5,9);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        // 7x7x13 Turbine
        let expected = utils::get_optimal_turbine(7,13);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        //9x9x17
        let expected = utils::get_optimal_turbine(9,17);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        //11x11x18
        let expected = utils::get_optimal_turbine(11,18);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        //13x13x18
        let expected = utils::get_optimal_turbine(13,18);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        //15x15x18
        let expected = utils::get_optimal_turbine(15,18);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
        //17x17x18
        let expected = utils::get_optimal_turbine(17,18);
        let actual = optimal_turbine_with_dimensions(expected.x_z, expected.y).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_turbine_based_on_fission_reactor() {
        // 153 fuel assembly reactor
        let expected_fuel_assemblies = 153;
        let expected_water_burn = 3060000;
        let actual: Turbine = turbine_based_on_fission_reactor(expected_water_burn).unwrap();
        let actual_fuel_assemblies = fission::optimal_fuel_assemblies(&actual);
        assert_eq!(actual_fuel_assemblies, expected_fuel_assemblies);
        //5x5x5
        let expected = utils::get_optimal_turbine(5,5);
        let water_burn_rate = 240000; //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        // 5x5x9 Turbine
        let expected = utils::get_optimal_turbine(5,5);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        // 7x7x13 Turbine
        let expected = utils::get_optimal_turbine(7,13);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        //9x9x17
        let expected = utils::get_optimal_turbine(9,17);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        //11x11x18
        let expected = utils::get_optimal_turbine(11,18);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        //13x13x18
        let expected = utils::get_optimal_turbine(13,18);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        //15x15x18
        let expected = utils::get_optimal_turbine(15,18);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
        //17x17x18
        let expected = utils::get_optimal_turbine(17,18);
        let water_burn_rate = min(expected.max_flow, expected.max_water_output); //mb/t
        let actual: Turbine = turbine_based_on_fission_reactor(water_burn_rate).unwrap();
        assert_eq!(actual.max_water_output, expected.max_water_output);
    }
}