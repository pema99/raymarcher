pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

pub fn difference(a: f64, b: f64) -> f64 {
    a.max(-b)
}

pub fn min_smooth(a: f64, b: f64, k: f64) -> f64 {
    let h = (0.5 + 0.5 * (a - b) / k).max(0.0).min(1.0);
    lerp(a, b, h) - k * h * (1.0 - h)
}

pub fn max_smooth(a: f64, b: f64, k: f64) -> f64 {
    min_smooth(a, b, -k)
}

pub fn difference_smooth(a: f64, b: f64, k: f64) -> f64 {
    min_smooth(a, -b, -k)
}
