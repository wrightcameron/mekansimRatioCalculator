// TODO Create CSV writer to build giant table of all combinations for turbines, reactors, and boilers
use std::{error::Error, io};

use crate::turbine;

pub fn create_turbine_lookup_table() -> Result<(), Box<dyn Error>> {
    let mut wtr: csv::Writer<io::Stdout> = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["Length", "Height", "Vents", "Dispersers", "Condensers", "Shaft Height", "Blades", "Coils", "Capacity", "Max Flow", "Tank Volume", "Max Production", "Max Water Output", "Energy SI Prefix"])?;
    for x in (5..17).step_by(2) {
        for y in 5..18 {
            let turbine = turbine::optimal_turbine_with_dimensions(x, y);
            wtr.serialize(turbine)?;
        }
    }
    wtr.flush()?;
    Ok(())
}