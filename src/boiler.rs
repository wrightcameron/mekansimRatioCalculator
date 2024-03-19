
pub struct Boiler {
    pub x: i32,
    pub z: i32,
    pub y: i32,
    pub heating_element: i32,
    pub dispersers: i32,
}

impl Boiler {
    /// Pretty print for Boiler struct
    #[allow(dead_code)]
    pub fn print(&self){
        println!("A {}x{}x{} Fission Reactor", self.x, self.z, self.y);
        println!("- Super Heating Elements {}", self.heating_element);
    }
}

impl Default for Boiler {
    fn default() -> Boiler {
        Boiler {
            x: 0,
            z: 0,
            y: 0,
            heating_element: 0,
            dispersers: 0,
        }
    }
}

#[allow(dead_code)]
pub fn optimal_boiler_with_dimensions(x: i32, z: i32, y: i32) -> Boiler {
    // TODO Need to throw an error, return nothing
    // Check if reactor's dimensions fall within an acceptable size
    if x < 3 {
        println!("Reactor length too small, min 3 blocks.");
    } else if 18 < x {
        println!("Reactor length too large, max 18 blocks.");
    }
    if z < 3 {
        println!("Reactor width too small, min 3 blocks.");
    } else if 18 < z {
        println!("Reactor width too large, max 18 blocks.");
    }
    if y < 4 {
        println!("Reactor height too small, min 4 blocks.");
    } else if 18 < y {
        println!("Reactor height too large, max 18 blocks.");
    }
    Boiler { ..Default::default()}
}