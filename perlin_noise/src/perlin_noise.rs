use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

pub type Rgb = Srgb<u8>;

pub const WINDOW_W: f32 = 512.0;
pub const WINDOW_H: f32 = 512.0;

const WINDOW_RIGHT: f32 = WINDOW_W / 2.0;
const WINDOW_LEFT: f32 = -WINDOW_RIGHT;

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    color: Rgb,
    origin: Point2,
    radius: f32,
    noise: Perlin,
    noise_parameter: f64,
}

impl Dot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .color(self.color)
            .w(self.radius)
            .h(self.radius)
            .x_y(self.origin.x, self.origin.y);
    }

    pub fn update(&mut self) {
        self.origin.x += 2.0;

        if self.origin.x > WINDOW_RIGHT {
            self.origin.x = WINDOW_LEFT;
            self.noise_parameter = random_f64();
        }

        self.origin.y = 240.0 * self.get_noise();
    }

    fn get_noise(&self) -> f32 {
        self.noise
            .get([0.01 * self.origin.x as f64, self.noise_parameter]) as f32
    }
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            color: STEELBLUE,
            origin: Point2::new(0.0, 200.0 * random_f32()),
            radius: 3.0,
            noise: Perlin::new().set_seed(random_range(0, 10000)),
            noise_parameter: 0.0,
        }
    }
}
