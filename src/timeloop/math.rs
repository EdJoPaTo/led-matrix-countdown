pub fn calc_relative_position(start: i64, end: i64, position: i64) -> f64 {
    let relative_max = end - start;
    let relative_position = position - start;
    relative_position as f64 / relative_max as f64
}

pub fn interpolate(start: i64, end: i64, position: f64) -> i64 {
    let relative_max = end - start;
    let relative_position = relative_max as f64 * position;
    start + relative_position as i64
}
