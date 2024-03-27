#![allow(dead_code)]
mod boiler;
mod fission;
mod metric_prefix;
mod setups;
mod turbine;
mod utils;
mod recipe;
mod lookup_table;

use std::env;
use std::io;
use crate::setups::Setup;
use fission::FissionReactor;

fn main() -> std::io::Result<()> {
    // TODO  Have some flags to cause other interactions, like just generate lookup tables, don't run interactive
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "lookup" {
        lookup_table::create_turbine_lookup_table().unwrap();
    } else {
        interactive();
    }
    Ok(())
}

/// Read User input from stdin, return it as trimed string
fn read_user_input() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

/// Handle interactive "REPL" use of tool, Root level loop
fn interactive() {
    println!("Welcome to Mekanism Ratio Calculator, interactive mode.");
    println!("Command (m: for help):");
    let prompt = "Options:\n\
                        c: create setup\n\
                        l: List created setups\n\
                        p: print created setups\n\
                        r: Get all materials needed for select setup\n\
                        q: Quit";
    let mut setups: Vec<setups::SetupType> = Vec::new();
    loop {
        let user_input = read_user_input();
        match user_input.as_ref() {
            "c" => {
                // TODO Figure out better error handling
                // let setup = match interactive_create_setup() {
                //     Some(s) => s,
                //     None => println!("Couldn't create setup, can't add it to collection."),
                // };
                let setup = interactive_create().unwrap();
                setups.push(setup);
            },
            "l" => {
                if setups.len() == 0 {
                    println!("No Setups to print yet, try creating one first.")
                }
                for (index, setup) in setups.iter().enumerate() {
                    println!("{index}: {}",setup.summarize());
                }
            },
            "p" => {
                if setups.len() == 0 {
                    println!("No Setups created yet, try creating one first.")
                }
                for setup in setups.iter() {
                    setup.print();
                }
            },
            "r" => {
                if setups.len() == 0 {
                    println!("No Setups created yet, try creating one first.")
                }
                println!("Which created setup do you wish to get parts for?  Choose the index.");
                let index = read_user_input().parse::<usize>().unwrap();
                let setup = &setups[index];
            },
            "m" => println!("{prompt}"),
            "q" => std::process::exit(0),
            _ => {
                println!("Unrecognized input: '{}' Avaliable commands\n{}", user_input, prompt);
            }
        }
    }
}

// TODO Make a new function for creating multi setups, Turbines, Reactors
fn interactive_create() -> Option<setups::SetupType> {
    let prompt = "Create. Options:\n\
                        m: multi - multi block setup.";
    println!("{prompt}");
    let user_input = read_user_input();
    match user_input.as_ref() {
        "m" => {
            // TODO Figure out better error handling
            // let setup = match interactive_create_setup() {
            //     Some(s) => s,
            //     None => println!("Couldn't create setup, can't add it to collection."),
            // };
            let multi_setup = interactive_multi_setup().unwrap();
            println!("Succesfully created the binary multi block setup.");
            return Some(multi_setup);
        }
        // TODO Need to implement
        // "t" => {
        //     println!("Input number of fuel assemblies.");
        //     let fuel_assemblies = read_user_input().parse::<i32>().unwrap();
        //     return fission::turbine_based_fission_reactor(fuel_assemblies);
        // },
        // "t" => {
        //     interactive_turbine();
        // },
        // "r" => {
        //     interactive_fission();
        // },
        _ => println!("Unrecognized input: '{}'", user_input),
    }
    // TODO Change this to an option
    None
}

fn interactive_multi_setup() -> Option<setups::SetupType> {
    let prompt = "Create new Mutliblock group setup:\n\
                        b: binary - Two multiblocks attached together\n\
                        t: trinary - Three multiblock attached setup\n\
                        q: quit - Return to top menu";
    println!("{prompt}");
    let mut user_input = read_user_input();
    match user_input.as_ref() {
        "b" => {
            // Declare to keep them in scope
            let turbine;
            let reactor;
            println!("Create turbine first (y/n)");
            user_input = read_user_input();
            if user_input.eq("y") {
                turbine = interactive_turbine();
                // Ask user if they want to make an encompanting fission reactor
                println!("Create an optimal fission reactor for this turbine? (y/n)");
                user_input = read_user_input();
                reactor = if user_input.eq("y") {
                    //Recommend Fission Reactor based on Turbine
                    let fuel_assemblies = fission::optimal_fuel_assemblies(&turbine);
                    fission::turbine_based_fission_reactor(fuel_assemblies)
                } else {
                    interactive_fission()
                };
            } else {
                reactor = interactive_fission();
                println!("Create an optimal turbine for this reactor? (y/n)");
                user_input = read_user_input();
                turbine = if user_input.eq("y") {
                    //Recommend Fission Reactor based on Turbine
                    turbine::turbine_based_on_fission_reactor(reactor.water_burn_rate).unwrap()
                } else {
                    interactive_turbine()
                };
            }
            let binary_setup = setups::BinarySetup { reactor, turbine };
            return Some(setups::SetupType::BinarySetup(binary_setup));
            // TODO Need to figure out how to represent the type of pipe between the two
        }
        "t" => {
            let trinary_setup = setups::TrinarySetup {
                ..Default::default()
            };
            return Some(setups::SetupType::TrinarySetup(trinary_setup));
        }
        "q" => println!("Returning"),
        _ => println!("Unrecognized input: '{}'", user_input),
    }
    return None;
}

// Turbines
fn interactive_turbine() -> turbine::Turbine {
    let prompt = "Turbines Options:\n\
                        o: optimal - optimal based on dimension.\n\
                        f: flow - optimal based on max flow of water\n\
                        m: manual - get calculations based on already existing turbine.";
    println!("{prompt}");
    let mut user_input = read_user_input();
    match user_input.as_ref() {
        "o" => {
            println!("Input turbine length & depth.");
            let x_z = read_user_input().parse::<i32>().unwrap();
            println!("Input turbine height.");
            let y = read_user_input().parse::<i32>().unwrap();
            //Pass the dimensions, get the most optimal turbine.
            let turbine =
                turbine::optimal_turbine_with_dimensions(x_z, y).unwrap_or_else(|error| {
                    panic!("Problem creating turbine: {error}");
                });
            turbine.print();
            return turbine;
        }
        "f" => {
            println!("What is the max flow (mb/t) of water?");
            let max_flow_rate = read_user_input().parse::<i32>().unwrap();
            let turbine =
                turbine::turbine_based_on_fission_reactor(max_flow_rate).unwrap_or_else(|error| {
                    panic!("Problem creating turbine: {error}");
                });
            turbine.print();
            println!("Create an optimal fission reactor for this turbine? (y/n)");
            user_input = read_user_input();
            if user_input.eq("y") {
                let fuel_assemblies = fission::optimal_fuel_assemblies(&turbine);
                let fission_reactor = fission::turbine_based_fission_reactor(fuel_assemblies);
                fission_reactor.print();
            }
            return turbine;
        }
        "m" => println!("TODO: Need to add"),
        _ => println!("Unrecognized input: '{}'", user_input),
    }
    // TODO Change this to an option
    return turbine::Turbine {
        ..Default::default()
    };
}

fn interactive_fission() -> FissionReactor {
    let prompt = "Fission Reactor. Options:\n\
                        o: optimal - optimal based on dimension.\n\
                        f: fuel assemblies - optimal based on number of fuel assemblies.\n\
                        m: manual - get calculations based on already existing reactor.";
    println!("{prompt}");
    let user_input = read_user_input();
    match user_input.as_ref() {
        "o" => {
            println!("Input reactor length.");
            let x = read_user_input().parse::<i32>().unwrap();
            println!("Input reactor width.");
            let z = read_user_input().parse::<i32>().unwrap();
            println!("Input reactor height.");
            let y = read_user_input().parse::<i32>().unwrap();
            //Pass the dimensions, get the most optimal turbine.
            return fission::optimal_fission_with_dimensions(x, z, y);
        }
        "f" => {
            println!("Input number of fuel assemblies.");
            let fuel_assemblies = read_user_input().parse::<i32>().unwrap();
            return fission::turbine_based_fission_reactor(fuel_assemblies);
        }
        "m" => println!("TODO: Need to add"),
        _ => println!("Unrecognized input: '{}'", user_input),
    }
    // TODO Change this to an option
    return FissionReactor {
        ..Default::default()
    };
}
