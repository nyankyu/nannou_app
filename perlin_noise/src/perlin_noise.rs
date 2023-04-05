use nannou::prelude::*;

pub type Rgb = Srgb<u8>;

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub color: Rgb,
    pub origin: Point2,
    pub radius: f32,
    pub max_radius: f32,
    pub growth_rate: f32,
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

    pub fn update(&mut self,) {
        if self.radius > self.max_radius {
            self.radius = 10.0;
        } else {
            self.radius += self.growth_rate;
        }
    }
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            color: STEELBLUE,
            origin: Point2::new(0.0, 0.0),
            radius: 10.0,
            max_radius: 200.0,
            growth_rate: 1.0
        }
    }
}
