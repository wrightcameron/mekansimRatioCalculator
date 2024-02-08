mod turbine;
mod fission;
mod utils;

use std::io;

fn main() -> std::io::Result<()> {
    // Add some basic arguemtns to pass in
    interactive();
    Ok(())
}

/// Read User input from stdin, return it as trimed string
fn read_user_input() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

/// Handle interactive "REPL" use of tool
fn interactive() {
    println!("Welcome to Mekanism Ratio Calculator, interactive mode.");
    //TODO Does user want to build a turbine or reactor
    println!("Options:\n t: turbine\nr: fission reactor");
    let mut user_input = read_user_input();
    if user_input.eq("t") {
        println!("Turbines.\n\n Options:\n o: optimal - optimal based on dimension.\nm: manual - get calculations based on already existing turbine.");
        user_input = read_user_input();
        if user_input.eq("o") {
            println!("Input turbine length.");
            let x_z = read_user_input().parse::<i32>().unwrap();

            println!("Input turbine height.");
            let y = read_user_input().parse::<i32>().unwrap();

            //Pass the dimensions, get the most optimal turbine.
            let turbine = turbine::optimal_turbine_with_dimensions(x_z,y);
            println!("{:?}",turbine);

            //Recommend Fission Reactor based on Turbine
            let fission_reactor = fission::turbine_based_fission_reactor(turbine);
            println!("{:?}",fission_reactor);

        }else if user_input.eq("m"){
            println!("Manual Turbine entry -- TODO");
        } else {
            println!("Unrecognized input: '{}'",user_input);
        }
    }else if user_input.eq("r"){
        println!("Fission Reactor -- TODO");
    } else {
        println!("Unrecognized input: '{}'",user_input);
    }
}
