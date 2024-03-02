#[path = "../src/turbine.rs"]
mod turbine;
#[path = "../src/fission.rs"]
mod fission;
#[path = "../src/utils.rs"]
mod utils;

#[test]
fn test_optimal_turbine_and_fission() {
    // 5x5x5 Turbine
    // TODO Remove variables from turbine we don't need to test, those can be defaulted to 0
    let starting_turbine = turbine::Turbine {
        x_z: 5,
        y: 5,
        ..Default::default()
    };
    let second_expected_turbine = turbine::optimal_turbine_with_dimensions(starting_turbine.x_z, starting_turbine.y);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(&second_expected_turbine);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate);
    assert_eq!(actual_turbine.max_water_output, second_expected_turbine.max_water_output, "Max Water Output mb/t didn't match.");
    assert_eq!(actual_turbine.max_flow, second_expected_turbine.max_flow, "Max Flows mb/t didn't match.");
}