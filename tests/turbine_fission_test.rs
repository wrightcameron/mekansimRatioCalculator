#[path = "../src/turbine.rs"]
mod turbine;
#[path = "../src/fission.rs"]
mod fission;
#[path = "../src/metricPrefix.rs"]
mod metricPrefix;

use serde_json;
use std::fs;

fn get_optimal_turbine(x_z: i32, y: i32) -> turbine::Turbine {
    let json_string =
        fs::read_to_string("data/optimal_turbines.json").expect("JSON file doesn't exist!");
    let json: Vec<turbine::Turbine> = serde_json::from_str(&json_string).expect("JSON was not well-formatted");
    json.iter().find(|x| x.x_z == x_z && x.y == y).unwrap().clone()
}

#[test]
fn test_optimal_turbine_and_fission() {
    // 5x5x5 Turbine
    let expected_turbine = get_optimal_turbine(5,5);
    // Get the optimal turbine
    let expected_turbine = turbine::optimal_turbine_with_dimensions(expected_turbine.x_z, expected_turbine.y);
    assert_eq!(expected_turbine.x_z, 5, "Optimal Turbine is not 5 x_z");
    assert_eq!(expected_turbine.y, 5, "Optimal Turbine is not 5 y");
    println!("Condensors: {}", expected_turbine.condensers);
    let actual_reactor: fission::FissionReactor = fission::turbine_based_fission_reactor(&expected_turbine);
    println!("Reactor Water Burn Rate: {}", actual_reactor.water_burn_rate);
    let actual_turbine = turbine::turbine_based_on_fission_reactor(actual_reactor.water_burn_rate);
    assert_eq!(actual_turbine.max_water_output, expected_turbine.max_water_output, "Max Water Output mb/t didn't match.");
    assert_eq!(actual_turbine.max_flow, expected_turbine.max_flow, "Max Flows mb/t didn't match.");
}