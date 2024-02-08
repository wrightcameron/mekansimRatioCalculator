mod turbine;
mod fission;
mod utils;

use std::io;

fn main() {
    // TODO Move this to an interactive RPEL
    println!("Welcome to Mekanism Ratio Calculator, interactive mode.");

    println!("Input turbine length.");
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input).unwrap();
    let x_z = user_input.trim().parse::<i32>().unwrap();
    user_input.clear();

    println!("Input turbine height.");
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input).unwrap();
    let y = user_input.trim().parse::<i32>().unwrap();
    user_input.clear();
    
    //Pass the dimensions, get the most optimal turbine.
    let turbine = turbine::optimal_turbine_with_dimensions(5,5);
    println!("{:?}",turbine);

    //Recommend Fission Reactor based on Turbine
    let fission_reactor = fission::turbine_based_fission_reactor(turbine);
    println!("{:?}",fission_reactor);
}

