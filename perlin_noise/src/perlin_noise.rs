use nannou::prelude::*;

pub type Rgb = Srgb<u8>;

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    color: Rgb,
    origin: Point2,
    radius: f32,
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
        self.origin.x += 10.0;
        if self.origin.x > 500.0 {
            self.origin.x = -500.0;
        }

        self.origin.y = random_range(-100.0, 100.0);
    }
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            color: STEELBLUE,
            origin: Point2::new(0.0, 0.0),
            radius: 10.0,
        }
    }
}
