use serde_json;
use std::fs;
use crate::turbine::Turbine;
use crate::fission::FissionReactor;

pub fn get_optimal_turbine(x_z: i32, y: i32) -> Turbine {
    let json_string =
        fs::read_to_string("data/optimal_turbines.json").expect("JSON file doesn't exist!");
    let json: Vec<Turbine> = serde_json::from_str(&json_string).expect("JSON was not well-formatted");
    json.iter().find(|x| x.x_z == x_z && x.y == y).unwrap().clone()
}

pub fn get_optimal_reactor(x: i32, z: i32, y: i32) -> FissionReactor {
    let json_string =
        fs::read_to_string("data/optimal_reactors.json").expect("JSON file doesn't exist!");
    let json: Vec<FissionReactor> = serde_json::from_str(&json_string).expect("JSON was not well-formatted");
    json.iter().find(|reactor| reactor.x == x && reactor.y == y && reactor.z == z).unwrap().clone()
}