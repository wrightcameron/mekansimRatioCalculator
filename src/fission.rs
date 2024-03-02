use crate::turbine::Turbine;
use std::cmp::min;
use factor::factor;

// use num_integer::Roots; 

const FUEL_ASSEMBLY_FLUID_BURN_RATE: i32 = 20000; // mb/t of water

/// Fission Reactor Struct, containing info on dimensions, block ammounts, and calculations
#[derive(Debug)]
pub struct FissionReactor {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub fuel_assemblies: i32,
    pub control_rods: i32,
    pub water_burn_rate: i32,  // mb/t
    pub heat_capacity: i32,  // J/K
    pub fuel_surface_area: i32,  // m2
    pub boil_efficiency: i32,  //This one will be hard to model
    pub max_burn_rate: i32,  // mB/t
    //TODO Need to add the burn rate calculations, like what the coolant flow rate will be
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
            boil_efficiency: 0,  //This one will be hard to model
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
#[allow(dead_code)]
pub fn turbine_based_fission_reactor(turbine: &Turbine) -> FissionReactor {
    let fuel_assemblies = optimal_fuel_assemblies(turbine);
    //12 = aread of the inside = (x - 2) * (z-2) * (y - 2)
    let reactor = FissionReactor {
        fuel_assemblies: fuel_assemblies,
        ..Default::default()
    };
    // Get the largest factor of fuel assemblies, for now assume composite/not prime
    let soma = factor::factor(fuel_assemblies as i64);
    println!("Soma is {:?}", soma);
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
#[allow(dead_code)]
fn optimal_fuel_assemblies(turbine: &Turbine) -> i32 {
    min(turbine.max_flow, turbine.max_water_output) / FUEL_ASSEMBLY_FLUID_BURN_RATE
}

#[cfg(test)]
mod tests {
    use super::*;

    //Arrange
    //Act
    //Assert

    // #[test]
    // fn test_calc_coils_needed() {
    //     let blades = 10;
    //     let expected_coils = 3;
    //     assert_eq!(calc_coils_needed(blades), expected_coils);
    // }

    // fn test_optimal_fission_with_dimensions() {
    //     let actual = FissionReactor {
    //         x: 5,
    //         z: 5,
    //         y: 5,
    //         fuel_assemblies: 10,
    //         control_rods: 5,
    //         water_burn_rate: fuel_assemblies * FUEL_ASSEMBLY_FLUID_BURN_RATE,
    //     };
    //     let expected = optimal_fission_with_dimensions(actual.x, actual.z, actual.y);
    //     assert_eq!(actual.fuel_assemblies, expected.fuel_assemblies);
    //     assert_eq!(actual.control_rods, expected.control_rods);
    // }

    #[test]
    fn test_turbine_based_fission_reactor() {
        let actual_turbine = Turbine {
            max_flow: 256000,
            max_water_output: 256000,
            ..Default::default()
        };
        let reactor = turbine_based_fission_reactor(&actual_turbine);
        assert_eq!(reactor.fuel_assemblies, 12);
        assert_eq!(reactor.control_rods, 6);
    }
}