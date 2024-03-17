#[path = "../src/turbine.rs"]
mod turbine;
#[path = "../src/fission.rs"]
mod fission;
#[path = "../src/metricPrefix.rs"]
mod metricPrefix;
#[path = "../src/utils.rs"]
mod utils;

#[test]
fn test_optimal_turbine_and_fission() {
    // 5x5x5 Turbine
    let expected_turbine = utils::get_optimal_turbine(5,5);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(&expected_turbine);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate);
    assert_eq!(actual_turbine, expected_turbine);
    // 5x5x9 Turbine
    let expected_turbine = utils::get_optimal_turbine(5,9);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(&expected_turbine);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate);
    assert_eq!(actual_turbine, expected_turbine);
    // 7x7x13 Turbine
    let expected_turbine = utils::get_optimal_turbine(7,13);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(&expected_turbine);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate);
    assert_eq!(actual_turbine, expected_turbine);
    //9x9x17 Turbine
    let expected_turbine = utils::get_optimal_turbine(9,17);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(&expected_turbine);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate);
    assert_eq!(actual_turbine, expected_turbine);
}