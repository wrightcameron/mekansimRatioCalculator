
//TODO Change file name to multi_setup, better describes what this is.

use crate::{boiler, fission, turbine};

pub trait Setup {
    fn print(&self);
    fn summarize(&self) -> String;
}

// This file will be more generic and I should use composition when can
pub enum SetupType {
    BinarySetup(BinarySetup),
    TrinarySetup(TrinarySetup),
}

impl Setup for SetupType {
    fn print(&self){
        match self {
            SetupType::BinarySetup(binary_setup) => binary_setup.print(),
            SetupType::TrinarySetup(trinary_setup) => trinary_setup.print(),
        }
    }

    fn summarize(&self) -> String{
        match self {
            SetupType::BinarySetup(binary_setup) => binary_setup.summarize(),
            SetupType::TrinarySetup(trinary_setup) => trinary_setup.summarize(),
        }
    }
}

pub struct BinarySetup {
    pub reactor: crate::fission::FissionReactor,
    pub turbine: crate::turbine::Turbine
}

impl Setup for BinarySetup {
    fn print(&self){
        self.reactor.print();
        self.turbine.print();
    }

    fn summarize(&self) -> String {
        format!(" {} / {} ",self.reactor.summarize(), self.turbine.summarize())
    } 
}

impl BinarySetup {
    pub fn print(&self){
        self.reactor.print();
        self.turbine.print();
    }
}

pub struct TrinarySetup {
    pub reactor: crate::fission::FissionReactor,
    pub turbine: crate::turbine::Turbine,
    pub boiler: crate::boiler::Boiler
}

impl Setup for TrinarySetup {
    fn print(&self){
        self.reactor.print();
        self.boiler.print();
        self.turbine.print();
    }

    fn summarize(&self){
        self.reactor.summarize();
        self.boiler.summarize();
        self.turbine.summarize();
    }
}

impl TrinarySetup {
    pub fn print(&self){
        self.reactor.print();
        self.boiler.print();
        self.turbine.print();
    }
}

// TODO THis is just for debugging right now
impl Default for TrinarySetup {
    fn default() -> TrinarySetup {
        TrinarySetup {
            reactor: fission::FissionReactor { ..Default::default() },
            boiler: boiler::Boiler { ..Default::default() },
            turbine: turbine::Turbine { ..Default::default() }
        }
    }
}
