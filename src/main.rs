#![allow(dead_code)]
mod turbine;
mod fission;
mod metric_prefix;
mod utils;
mod setups;
mod boiler;

use std::io;

use fission::FissionReactor;

use crate::{setups::BinarySetup, turbine::Turbine};

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

/// Handle interactive "REPL" use of tool, Root level loop
fn interactive() {
    println!("Welcome to Mekanism Ratio Calculator, interactive mode.");
    //TODO Does user want to build a turbine or reactor
    println!("Command (m: for help):");
    loop {
        let user_input = read_user_input();
        match user_input.as_ref() {
            "t" => {
                interactive_turbine();
            },
            "r" => {
                interactive_fission();
            },
            "c" => {
                interactive_create_setup();
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

fn interactive_create_setup() {
    let prompt = "Create new Mutliblock group setup:
    b: binary - Two multiblocks attached together
    t: trinary - Three multiblock attached setup
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
            let binary_setup = BinarySetup {reactor, turbine};
            binary_setup.print();
            // TODO Need to figure out how to represent the type of pipe between the two
        },
        "t" => {
            println!("TODO: Need to add");
        },
        "q" => println!("Returning"),
        _ => println!("Unrecognized input: '{}'",user_input),
    }
}

    // Turbines
fn interactive_turbine() -> Turbine {
    let prompt = "Turbines Options:
    o: optimal - optimal based on dimension.
    f: flow - optimal based on max flow of water
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
            let turbine = turbine::optimal_turbine_with_dimensions(x_z,y).unwrap_or_else(| error | {
                panic!("Problem creating turbine: {error}");
            });
            turbine.print();
            return turbine;
        },
        "f" => {
            println!("What is the max flow (mb/t) of water?");
            let max_flow_rate = read_user_input().parse::<i32>().unwrap();
            let turbine = turbine::turbine_based_on_fission_reactor(max_flow_rate).unwrap_or_else(| error | {
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
        },
        "m" => println!("TODO: Need to add"),
        _ => println!("Unrecognized input: '{}'",user_input),
    }
    // TODO Change this to an option
    return Turbine { ..Default::default() }
}

fn interactive_fission() -> FissionReactor {
    let prompt = "Fission Reactor. Options:
    o: optimal - optimal based on dimension.
    f: fuel assemblies - optimal based on number of fuel assemblies.
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
        },
        "f" => {
            println!("Input number of fuel assemblies.");
            let fuel_assemblies = read_user_input().parse::<i32>().unwrap();
            return fission::turbine_based_fission_reactor(fuel_assemblies);
        },
        "m" => println!("TODO: Need to add"),
        _ => println!("Unrecognized input: '{}'",user_input),
    }
    // TODO Change this to an option
    return FissionReactor { ..Default::default() }
}

/// Print help message for interactive commands at top level
fn print_interactive_help() {
    let prompt = "Options:
    t: turbine
    r: fission reactor
    c: binary setup
    Quit: q";
    println!("{prompt}");
}
