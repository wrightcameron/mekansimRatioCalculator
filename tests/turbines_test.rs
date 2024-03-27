#[path = "../src/turbine.rs"]
mod turbine;
#[path = "../src/fission.rs"]
mod fission;
#[path = "../src/metric_prefix.rs"]
mod metric_prefix;
#[path = "../src/utils.rs"]
mod utils;

// TODO Put this in a common place
const FUEL_ASSEMBLY_FLUID_BURN_RATE: i32 = 20000; // mb/t of water

#[test]
fn dimensional_turbine_compared_fuel_assembly() {
    // 5x5x5 Turbine
    let expected_turbine = utils::get_optimal_turbine(5,5);
    let fuel_assemblies = fission::optimal_fuel_assemblies(&expected_turbine);
    let water_burn_rate = fuel_assemblies * FUEL_ASSEMBLY_FLUID_BURN_RATE;
    let actual_turbine = turbine::turbine_based_on_fission_reactor(water_burn_rate).unwrap();
    assert_eq!(actual_turbine, expected_turbine);
    // 5x5x9 Turbine
    let expected_turbine = utils::get_optimal_turbine(5,9);
    let fuel_assemblies = fission::optimal_fuel_assemblies(&expected_turbine);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(fuel_assemblies);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate).unwrap();
    assert_eq!(actual_turbine, expected_turbine);
    // 7x7x13 Turbine
    let expected_turbine = utils::get_optimal_turbine(7,13);
    let fuel_assemblies = fission::optimal_fuel_assemblies(&expected_turbine);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(fuel_assemblies);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate).unwrap();
    assert_eq!(actual_turbine, expected_turbine);
}