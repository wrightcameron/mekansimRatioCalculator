use num::pow::Pow;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Prefix {
    Base,
    Kilo,
    Mega,
    Giga,
    Tera
}

/// Metric 1 to million conversion
pub fn convert_to_mega(n: f32) -> f32 {
    let mega = n / 1000000.0;
    // Appears that Mekanism calc just removes the 3rd decimcal instead of rounding up.
    (mega * 100.0).floor() / 100.0
}

/// Metric 1 to thousand conversion
pub fn convert_to_kilo(n: f32) -> f32 {
    let kilo = n / 1000.0;
    // Appears that Mekanism calc just removes the 3rd decimcal instead of rounding up.
    (kilo * 100.0).floor() / 100.0
}

pub fn drop_tenth_decimal(n: f32) -> f32 {
    (n * 10.0).floor() / 10.0
}

pub fn drop_decimals(n: f32) -> f32 {
    n.floor()
}

pub fn convert_to_prefix(n: f32, start_prefix: &Prefix, end_prefix: &Prefix) -> f32 {
    let starting_power = match start_prefix {
        self::Prefix::Base => 0,
        self::Prefix::Kilo => 3,
        self::Prefix::Mega => 6,
        self::Prefix::Giga => 9,
        self::Prefix::Tera => 12
    };
    let end_power = match end_prefix {
        self::Prefix::Base => 0,
        self::Prefix::Kilo => 3,
        self::Prefix::Mega => 6,
        self::Prefix::Giga => 9,
        self::Prefix::Tera => 12
    };
    let difference: i32 = starting_power - end_power;
    let is_negative = difference < 0;
    if is_negative {
        return n / 10.pow(difference.abs() as u32) as f32;
    }
    return n * 10.pow(difference.abs() as u32) as f32;
} 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metric_prefix::Prefix::*;
    
    #[test]
    fn test_convert_to_prefix() {
        // Sanity Base to Base
        let starting = 123.0;
        let expected = 123.0;
        let actual = convert_to_prefix(starting, &Base, &Base);
        assert_eq!(expected, actual);
        // Base to Kilo
        let starting = 123000.0;
        let expected = 123.0;
        let actual = convert_to_prefix(starting, &Base, &Kilo);
        assert_eq!(expected, actual);
        // Mega to Kilo
        let starting = 123.0;
        let expected = 123000.0;
        let actual = convert_to_prefix(starting, &Mega, &Kilo);
        assert_eq!(expected, actual);
    } 
    

}