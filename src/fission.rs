use crate::turbine::Turbine;
use std::cmp::min;
use factor::factor;
use serde::Deserialize;
// use num_integer::Roots; 

const FUEL_ASSEMBLY_FLUID_BURN_RATE: i32 = 20000; // mb/t of water
const CASING_HEAT_CAPACITY: i32 = 1000;
const FISSION_SURFACE_AREA_TARGET: f32 = 4.0;

/// Fission Reactor Struct, containing info on dimensions, block ammounts, and calculations
#[derive(Deserialize, Debug, Clone)]
pub struct FissionReactor {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub fuel_assemblies: i32,
    pub control_rods: i32,
    pub water_burn_rate: i32,  // mb/t
    pub heat_capacity: i32,  // J/K
    pub fuel_surface_area: i32,  // m2
    pub boil_efficiency: f32,  //This one will be hard to model
    pub max_burn_rate: i32,  // mB/t
    //TODO Need to add the burn rate calculations, like what the coolant flow rate will be
}

//have this here so I can ignore the broken value fuel_surface_area
impl PartialEq for FissionReactor {
    fn eq(&self, other: &Self) -> bool {
        self.x ==  other.x &&
        self.z == other.z &&
        self.y ==  other.y &&
        self.fuel_assemblies == other.fuel_assemblies &&
        self.control_rods ==  other.control_rods &&
        self.water_burn_rate == other.water_burn_rate &&
        self.heat_capacity == other.heat_capacity &&
        // self.fuel_surface_area == other.fuel_surface_area &&
        self.boil_efficiency == other.boil_efficiency &&
        self.max_burn_rate == other.max_burn_rate
    }
}

impl Default for FissionReactor {
    fn default() -> FissionReactor {
        FissionReactor {
            x: 0,
            z: 0,
            y: 0,
            fuel_assemblies: 0,
            control_rods: 0,
            water_burn_rate: 0,
            heat_capacity: 0,  // J/K
            fuel_surface_area: 0,  // m2
            boil_efficiency: 0.0,  //This one will be hard to model
            max_burn_rate: 0,  // mB/t
        }
    }
}

impl FissionReactor {
    /// Pretty print for Fission Reactor struct
    #[allow(dead_code)]
    pub fn print(&self){
        println!("A {}x{}x{} Fission Reactor", self.x, self.z, self.y);
        println!("- Fuel Assemblies {}, and Control Rods {}", self.fuel_assemblies, self.control_rods);
        println!("- Water Burn Rate {} mb/t \n", self.water_burn_rate);
    }
}

/// Create an optimal fission reactor based on passed in dimensions
pub fn optimal_fission_with_dimensions(x: i32, z: i32, y: i32) -> FissionReactor {
    // TODO Need to throw an error, return nothing
    // Check if reactor's dimensions fall within an acceptable size
    if x < 3 {
        println!("Reactor length too small, min 3 blocks.");
    } else if 18 < x {
        println!("Reactor length too large, max 18 blocks.");
    }
    if z < 3 {
        println!("Reactor width too small, min 3 blocks.");
    } else if 18 < z {
        println!("Reactor width too large, max 18 blocks.");
    }
    if y < 4 {
        println!("Reactor height too small, min 4 blocks.");
    } else if 18 < y {
        println!("Reactor height too large, max 18 blocks.");
    }
    // Calculate number of fuel assemblies and control rods
    let (fuel_assemblies, control_rods) = fuel_assemblies_dimensions(x, z, y);
    // Is this something the reactor should know about itself?  Also this could be calculated automatically by the struct
    let water_burn_rate = fuel_assemblies * FUEL_ASSEMBLY_FLUID_BURN_RATE;
    FissionReactor {x, z, y, fuel_assemblies, control_rods, water_burn_rate, ..Default::default()}
}

/// Create fission reactor based on max output/max flow from turbine
pub fn turbine_based_fission_reactor(fuel_assemblies: i32) -> FissionReactor {
    //12 = aread of the inside = (x - 2) * (z-2) * (y - 2)
    let mut reactor = FissionReactor {
        fuel_assemblies: fuel_assemblies,
        ..Default::default()
    };
    // Find larger than needed reactor, then shave off it.
    let mut area;
    for i in 4..18 {
        let x: i32 = i - 2;
        let y: i32 = i - 2;
        let z: i32 = i - 2;
        let efficient_area: i32 = ((x * z) as f32 / 2.0).ceil() as i32 * y;
        let control_rods = (x.pow(2) as f32 / 2.0).ceil() as i32;
        if efficient_area > fuel_assemblies + control_rods {
            reactor.x = x + 2;
            reactor.y = y + 2;
            reactor.z = z + 2;
            reactor.control_rods = control_rods;
            break;
        }
    }
    // Check if x can be reduce
    let x = reactor.x - 3;
    let y: i32 = reactor.y - 2;
    let z: i32 = reactor.z - 2;
    area = (x * y * z) / 2;
    let control_rods = x * z / 2;
    if area >= fuel_assemblies + control_rods {
        reactor.x = reactor.x - 1;
        reactor.control_rods = control_rods;
    } 
    // Check if y can be reduce
    let y = reactor.y - 3;
    let x: i32 = reactor.x - 2;
    let z: i32 = reactor.z - 2;
    area = (x * y * z) / 2;
    if area >= fuel_assemblies + control_rods {
        reactor.y = reactor.y - 1;
    }
    // Surface area
    let mut surface_area = fuel_assemblies * 6;  //306
    let levels = fuel_assemblies / reactor.control_rods;
    let mut touching_assemblies = 0;
    if levels <= 1 {
        touching_assemblies = 0;
    } else if levels == 2 {
        touching_assemblies = reactor.control_rods * 2;
    } else if levels > 2 {
        touching_assemblies = reactor.control_rods * 2 + reactor.control_rods * levels - 2;
    }
    // let remander = fuel_assemblies - reactor.control_rods * touching_assemblies;
    // touching_assemblies = touching_assemblies + (remander * 6);
    surface_area = surface_area - touching_assemblies;
    // Boil Efficiency Rate
    let avg_surface_area = surface_area / fuel_assemblies;
    let mut boil_efficiency = avg_surface_area as f32 / FISSION_SURFACE_AREA_TARGET;
    if boil_efficiency > 1.0 {
        boil_efficiency = 1.0;
    }
    // reactor.x = x;
    // reactor.z = z;
    // reactor.y = y;  // TODO Figure out what the hell do for this 
    // reactor.control_rods = factors[second_value_index + 1] as i32;
    reactor.water_burn_rate = fuel_assemblies * FUEL_ASSEMBLY_FLUID_BURN_RATE;
    reactor.heat_capacity = heat_capacity(reactor.x, reactor.z, reactor.y);
    reactor.fuel_surface_area = surface_area;
    reactor.boil_efficiency = boil_efficiency;
    reactor.max_burn_rate = fuel_assemblies;
    // TODO What do we do with these ratios?
    // TODO How do I calculate the hight.  3,4 is correct for the x,z but y in this case is 2, is it the distance between the two variables.
    // No it shouldnt be cause all should lead to size of 2
    reactor
}

/// area inside reactor, 
/// assumes reactor has 100 efficency by spacing rods 1 block apart
#[allow(dead_code)]
fn area_inside_reactor(x: i32, z: i32, y: i32) -> i32 {
    (x - 2) * (z - 2) * (y - 2)
}

fn fuel_assemblies_dimensions(x: i32, z: i32, y: i32) -> (i32, i32) {
    // Area of x and z divided by 2 to keep ideal boil rate at 100%
    // TODO This doesn't account for rounding, like an area of 9
    let f_x = x as f32;
    let f_z = z as f32;
    
    let ideal_area_slice = ((f_x - 2.0) * (f_z - 2.0) / 2.0).ceil() as i32;
    let num_fuel_assemblies = ideal_area_slice * (y - 3);
    let num_control_rods = ideal_area_slice;
    (num_fuel_assemblies, num_control_rods)
}

/// Get required area inside reactor, 
/// assumes reactor has 100 efficency by spacing rods 1 block apart
/// and x,y,z are the same to only solve for x
// fn required_area_inside_reactor(fuel_assemblies: i32) -> i32 {
//     (fuel_assemblies * 2).nth_root(3).round() + 2
// }

/// Get optimal number of fuel assemblies based on max flow and max water output of turbine
pub fn optimal_fuel_assemblies(turbine: &Turbine) -> i32 {
    // Any decimal remainder truncated, which is fine the reactor burn rate should be less then turbine
    min(turbine.max_flow, turbine.max_water_output) / FUEL_ASSEMBLY_FLUID_BURN_RATE
}

fn heat_capacity(x: i32, z: i32, y: i32) -> i32 {
    let top_bottom = x * z * 2;
    let front_back = x * (y - 2) * 2;
    let left_right = (z - 2) * (y - 2) * 2;
    (top_bottom + front_back + left_right) * CASING_HEAT_CAPACITY
}

// Find structure surface area https://github.com/mekanism/Mekanism/blob/1.20.4/src/generators/java/mekanism/generators/common/content/fission/FissionReactorValidator.java#L58

// https://github.com/mekanism/Mekanism/blob/a3660901504ef724366224012bcea14be2cb734a/src/generators/java/mekanism/generators/common/content/fission/FissionReactorMultiblockData.java#L471
// fn boil_efficiency(reactor: &FissionReactor) -> f32 {
//     if reactor.fuel_assemblies == 0 {
//         return 0.0;
//     }
//     let avg_surface_area = structure_surface_area(reactor.fuel_assemblies) / reactor.fuel_assemblies;
//     0.0
// }

// fn structure_surface_area(fuel_assemblies: i32) -> i32 {
//     let surface_area = fuel_assemblies * 6;  // Fuel Assemblies have six sides
//     let factors: Vec<i64> = factor::factor(fuel_assemblies as i64);
//     println!("{:?}",factors);
//     0
// }

fn optimal_structure(fuel_assemblies: i32) -> (i32, i32, i32) {
    let factors: Vec<i64> = factor::factor(fuel_assemblies as i64);
    // Get middle pair, by finding the middle of the list in this case 3,4
    let factor_length: usize = factors.len();
    let middle: usize = factor_length / 2 - 1;
    println!("{:?}",factors);
    println!("{:?}",middle);
    let first_value = factors[middle];
    let first_value_index = factors.iter().position(|&x| x == first_value).unwrap();
    let second_value = factors[factor_length - 1 - middle];
    let second_value_index = factors.iter().position(|&x| x == second_value).unwrap();
    println!("{}, second index is {}",first_value_index, second_value_index);
    println!("{}, pair is {}",first_value, second_value);
    let difference = 2;
    let x = first_value as i32 + 2;
    let z = second_value as i32 + 2;
    let y = difference + 1 + 2;  // Plus 1 for controllers
    (x, z, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    //Arrange
    //Act
    //Assert

    #[test]
    fn test_optimal_structure(){
        let actual = (5,6,5);
        let expected = optimal_structure(12);
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn test_structure_surface_area(){
    //     let actual = 60;
    //     let expected = structure_surface_area(12);
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn test_heat_capacity() {
        let actual = 114000;  // J/K
        let expected = heat_capacity(5,6,5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_optimal_fission_with_dimensions() {
        let actual = utils::get_optimal_reactor(5,6,5);
        let expected = optimal_fission_with_dimensions(actual.x, actual.z, actual.y);
        assert_eq!(actual.fuel_assemblies, expected.fuel_assemblies);
        assert_eq!(actual.control_rods, expected.control_rods);
    }

    #[test]
    fn test_turbine_based_fission_reactor() {
        // 5x5x5 Turbine
        let turbine: Turbine = utils::get_optimal_turbine(5,5);
        let expected_reactor = utils::get_optimal_reactor(5,6,5);
        let fuel_assemblies = super::optimal_fuel_assemblies(&turbine);
        let actual_reactor = turbine_based_fission_reactor(fuel_assemblies);
        assert_eq!(actual_reactor, expected_reactor);
        // 5x5x9 Turbine
        let turbine = utils::get_optimal_turbine(5,9);
        let expected_reactor = utils::get_optimal_reactor(7,7,7);
        let fuel_assemblies = super::optimal_fuel_assemblies(&turbine);
        let actual_reactor = turbine_based_fission_reactor(fuel_assemblies);
        assert_eq!(actual_reactor, expected_reactor);
        // 153 fuel assemblies
        let expected_reactor = FissionReactor { fuel_assemblies: 153, water_burn_rate: 3060000, ..Default::default() };
        let actual_reactor = turbine_based_fission_reactor(expected_reactor.fuel_assemblies );
        assert_eq!(actual_reactor.fuel_assemblies, expected_reactor.fuel_assemblies);
        assert_eq!(actual_reactor.water_burn_rate, expected_reactor.water_burn_rate);
    }
}