// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base_rate = 221;
    let raw_rate = base_rate * speed as u32;

    let success_rate = match speed {
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 1.0,
    };

    raw_rate as f64 * success_rate

    // unimplemented!("calculate hourly production rate at speed: {}", speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32

    // unimplemented!("calculate the amount of working items at speed: {}", speed)
}
