
// This file will be more generic and I should use coposition when can

#[allow(dead_code)]
pub struct BinarySetup {
    pub reactor: crate::fission::FissionReactor,
    pub turbine: crate::turbine::Turbine
}

#[allow(dead_code)]
pub struct TrinarySetup {
    pub reactor: crate::fission::FissionReactor,
    pub turbine: crate::turbine::Turbine,
    pub boiler: crate::boiler::Boiler
}