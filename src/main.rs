mod turbine;
mod fission;
mod metricPrefix;
mod utils;
mod setups;
mod boiler;

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
    // let mut turbines_vec: Vec<turbine::Turbine> = Vec::new();
    //TODO Does user want to build a turbine or reactor
    // Root level loop
    println!("Command (m: for help):");
    loop {
        let mut user_input = read_user_input();
        match user_input.as_ref() {
            "t" => {
                // Turbines
                println!("Turbines.\n\n Options:\n o: optimal - optimal based on dimension.\nm: manual - get calculations based on already existing turbine.");
                user_input = read_user_input();
                match user_input.as_ref() {
                    "o" => {
                        println!("Input turbine length & depth.");
                        let x_z = read_user_input().parse::<i32>().unwrap();
                        println!("Input turbine height.");
                        let y = read_user_input().parse::<i32>().unwrap();

                        //Pass the dimensions, get the most optimal turbine.
                        let turbine = turbine::optimal_turbine_with_dimensions(x_z,y);
                        turbine.print();
                        // Ask user if they want to make an encompanting fission reactor
                        println!("Create an optimal fission reactor for this turbine? (y/n)");
                        user_input = read_user_input();
                        if user_input.eq("y") {
                            //Recommend Fission Reactor based on Turbine
                            let fission_reactor = fission::turbine_based_fission_reactor(&turbine);
                            fission_reactor.print();
                        }
                    },
                    "m" => println!("TODO: Need to add"),
                    _ => println!("Unrecognized input: '{}'",user_input),
                }
            },
            "r" => {
                println!("Fission Reactor.\n\n Options:\n o: optimal - optimal based on dimension.\nm: manual - get calculations based on already existing reactor.");
                user_input = read_user_input();
                match user_input.as_ref() {
                    "o" => {
                        println!("Input reactor length.");
                        let x = read_user_input().parse::<i32>().unwrap();
                        println!("Input reactor width.");
                        let z = read_user_input().parse::<i32>().unwrap();
                        println!("Input reactor height.");
                        let y = read_user_input().parse::<i32>().unwrap();

                        //Pass the dimensions, get the most optimal turbine.
                        let fission_reactor = fission::optimal_fission_with_dimensions(x, z, y);
                        fission_reactor.print();
                        // Ask user if they want to make an encompanting turbine
                        println!("Create an optimal turbine for this turbine? (y/n)");
                        user_input = read_user_input();
                        if user_input.eq("y") {
                            //Recommend Turbine based Fission Reactor
                            let turbine = turbine::turbine_based_on_fission_reactor(fission_reactor.water_burn_rate);
                            // turbine.print();
                        }
                    },
                    "m" => println!("TODO: Need to add"),
                    _ => println!("Unrecognized input: '{}'",user_input),
                }
            },
            "m" => print_interactive_help(),
            "q" => std::process::exit(0),
            _ => {
                println!("Unrecognized input: '{}'",user_input);
                print_interactive_help();
            },
        }
    }
}

/// Print help message for interactive commands at top level
fn print_interactive_help() {
    println!("Options:\n t: turbine\nr: fission reactor\nQuit: q");
}
