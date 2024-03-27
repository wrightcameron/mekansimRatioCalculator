//TODO module for calculating amount of resources needed to build any setup
use crate::boiler;
use crate::turbine;
use crate::fission;
use crate::setups;


type Block = u16;

enum TurbineBlocks {
    TurbineCasing(Block),
    StructuralGlass(Block),
    TurbineBlade(Block),
    TurbineRotor(Block),
    RotationalComplex(Block),
    ElectromagneticCoil(Block),
    TurbineVent(Block),
    SaturatingCondenser(Block)
}

enum FissionReactorBlocks {
    FissionReactorCasing,
    ReactorGlass,
    FissionFuelAssembly,
    ControlRodAssembly
}

enum BoilerBocks {

}

pub fn turbine_recipe(turbine: &turbine::Turbine) {
    println!("Recipe for {}x{}x{} Turbine", turbine.x_z, turbine.x_z, turbine.y);
    let saturating_condenser = turbine.condensers as Block;
    let turbine_vent = turbine.condensers as Block;
    let electromagnetic_coil = turbine.condensers as Block;
    let rotational_complex = turbine.condensers as Block;
    let turbine_rotor = turbine.condensers as Block;
    let turbine_blade = turbine.condensers as Block;
    // Saturating Condensers
    println!("{} Saturating Condenser", saturating_condenser);
    // Turbine Vent
    println!("{} Turbine Vent", turbine_vent);
    // Electromagnetic Coil
    println!("{} Electromagnetic Coil", electromagnetic_coil);
    // Rotational Complex
    println!("{} Rotational Complex", rotational_complex);
    // Turbine Rotor
    println!("{} Turbine Rotor", turbine_rotor);
    // Turbine Blade
    println!("{} Turbine Blade", turbine_blade);
    // Structural Glass
    let structural_glass = turbine_blade * (turbine.x_z - 2) as Block * 4;
    println!("{} Structural Glass", structural_glass);
    // Turbine Casng
    let remaining_y = (turbine.y - turbine.shaft_height) as Block;
    let side_area = remaining_y * turbine.x_z as Block * 4;
    let top_bottom_area = ((turbine.x_z - 2).pow(2) * 2) as Block;
    let turbine_casing = top_bottom_area + side_area - turbine_vent;
    println!("{} Turbine Casng", turbine_casing);

}

pub fn fission_reactor_recipe(reactor: &fission::FissionReactor) {

}

pub fn boiler_recipe(boiler: &boiler::Boiler) {

}

pub fn handle_binary_type(binary_type: &setups::BinarySetup) {

}

pub fn handle_trinary_type(trinary_type: &setups::TrinarySetup){

}