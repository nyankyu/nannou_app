use super::{
    color_noise::ColorNoise, force_field::ForceField,
};
use nannou::prelude::*;

/// particle
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    position: Point2,
    radius: f32,
    window_rect: Rect,
    hue: f32,
}

impl Particle {
    pub fn new(window_rect: Rect) -> Self {
        Self {
            position: Point2::new(
                window_rect.right() + 10.0,
                0.0,
            ),
            radius: 1.7,
            window_rect: window_rect,
            hue: 0.0,
        }
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .hsla(self.hue, 1.0, 0.5, 0.5)
            .w(self.radius)
            .h(self.radius)
            .xy(self.position);
    }

    pub fn update(
        &mut self,
        field: &ForceField,
        color_noise: &mut ColorNoise,
    ) {
        self.position.x +=
            field.get_x(self.position.x, self.position.y);
        self.position.y +=
            field.get_y(self.position.x, self.position.y);

        if self.position.x < self.window_rect.left()
            || self.position.x > self.window_rect.right()
            || self.position.y > self.window_rect.top()
        {
            self.respawn(color_noise);
        }
    }

    fn respawn(&mut self, color: &mut ColorNoise) {
        self.position.x = random_range(
            self.window_rect.left() / 2.0,
            self.window_rect.right() / 2.0,
        );
        self.position.y = random_range(
            self.window_rect.bottom(),
            self.window_rect.bottom() + 70.0,
        ) - 150.0;

        self.hue = color.get_hue(self.position.x);
    }
}
