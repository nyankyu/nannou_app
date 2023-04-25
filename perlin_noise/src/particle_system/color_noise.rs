use nannou::{
    noise::{Perlin, Seedable, NoiseFn},
    prelude::*,
};

pub struct ColorNoise {
    noise: Perlin,
    param: f64,
}

impl ColorNoise {
    pub fn new() -> ColorNoise {
        ColorNoise {
            noise: Perlin::new().set_seed(random()),
            param: -10000.0,
        }
    }

    pub fn get_hue(&mut self, x: f32) -> f32 {
        self.param += 0.0001;
        self.noise.get([0.001 * x as f64, self.param]) as f32
    }
}
