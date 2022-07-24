// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html

pub fn production_success_rate(speed: u8) -> f64 {
    match speed {
        1..=4 => return 1.0,
        5..=8 => return 0.9,
        9..=10 => return 0.77,
        _ => return 0.0,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    return speed as f64 * 221.0 * production_success_rate(speed);
}
