pub fn production_rate_per_hour(speed: u8) -> f64 {
    let group = (speed as f64 /4.0).ceil() - 1 as f64;
    let sr=[1.0, 0.9, 0.77];
    let success_rate=sr[group as usize];
    221 as f64 * speed as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let minutely = production_rate_per_hour(speed) / 60 as f64;
    minutely as u32
}