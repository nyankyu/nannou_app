use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

pub struct ForceField {
    velocity_x: Perlin,
    velocity_y: Perlin,
    noise_param: f64,
    noise_frequency_x: f64,
    noise_frequency_y: f64,
    noise_scale_x: f32,
    noise_scale_y: f32,
}

impl ForceField {
    pub fn new() -> Self {
        Self {
            velocity_x: Perlin::new().set_seed(random()),
            velocity_y: Perlin::new().set_seed(random()),
            noise_param: -100000.0,
            noise_frequency_x: 0.005,
            noise_frequency_y: 0.003,
            noise_scale_x: 1.2,
            noise_scale_y: 2.0,
        }
    }

    #[allow(unused)]
    pub fn display(&self, draw: &Draw) {
        for x in -10..=10 {
            for y in -10..=10 {
                let px = x as f32 * 25.6;
                let py = y as f32 * 25.6;
                let vx = self.get_x(px, py);
                let vy = self.get_y(px, py) + 20.0;

                draw.line()
                    .color(GRAY)
                    .weight(3.0)
                    .start(vec2(px, py))
                    .end(vec2(px + vx, py + vy));
            }
        }
    }

    pub fn update(&mut self) {
        self.noise_param += 0.007;
    }

    pub fn get_x(&self, x: f32, y: f32) -> f32 {
        self.velocity_x.get([
            self.noise_frequency_x * x as f64,
            self.noise_frequency_y * y as f64,
            self.noise_param,
        ]) as f32
            * self.noise_scale_x
    }

    pub fn get_y(&self, x: f32, y: f32) -> f32 {
        self.velocity_y.get([
            self.noise_frequency_x * x as f64,
            self.noise_frequency_y * y as f64,
            self.noise_param,
        ]) as f32
            * self.noise_scale_y + 1.5
    }
}
