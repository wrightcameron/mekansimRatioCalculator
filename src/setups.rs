
// This file will be more generic and I should use composition when can

pub struct BinarySetup {
    pub reactor: crate::fission::FissionReactor,
    pub turbine: crate::turbine::Turbine
}

pub struct TrinarySetup {
    pub reactor: crate::fission::FissionReactor,
    pub turbine: crate::turbine::Turbine,
    pub boiler: crate::boiler::Boiler
}

// TODO  Change this to a trait so Binary and Trinary share it
impl BinarySetup {
    pub fn print(&self){
        self.reactor.print();
        self.turbine.print();
    }
}