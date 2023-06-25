const infinity: f64 = f64::INFINITY;
const pi: f64 = 3.141_592_653_589_793;

use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
pub fn random_f64_1(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
