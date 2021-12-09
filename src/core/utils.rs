use rand::Rng;

pub use std::f32::consts::PI as PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees.to_radians();
}

pub fn random_double() -> f32 {
    return rand::thread_rng().gen_range(0.0..1.0);
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    return min + (max-min)*random_double();
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}