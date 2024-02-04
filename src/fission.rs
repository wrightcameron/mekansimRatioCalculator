use crate::turbine::Turbine;
use std::cmp::min;

const FUEL_ASSEMBLY_FLUID_BURN_RATE: i32 = 20000;

///
#[derive(Debug)]
pub struct FissionReactor {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub fuel_assemblies: i32,
    pub control_rods: i32,
}

///
pub fn turbine_based_fission_reactor(turbine: Turbine) -> FissionReactor {
    let fuel_assemblies = optimal_fuel_assemblies(turbine);
    FissionReactor {
        x: 0,
        z: 0,
        y: 0,
        fuel_assemblies: fuel_assemblies,
        control_rods: 0,
    }
}

///
fn optimal_fuel_assemblies(turbine: Turbine) -> i32 {
    min(turbine.max_flow, turbine.max_water_output) / FUEL_ASSEMBLY_FLUID_BURN_RATE
}
