
pub fn convert_to_mega(n: f32) -> f32 {
    let mega = n / 1000000.0;
    // Appears that Mekanism calc just removes the 3rd decimcal instead of rounding up.
    (mega * 100.0).floor() / 100.0
}