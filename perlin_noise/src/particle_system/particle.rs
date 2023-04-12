use super::force_field::ForceField;
use nannou::prelude::*;

type Rgb = Srgb<u8>;

/// particle
#[derive(Debug, Clone, Copy)]
pub struct Particle {
  position: Point2,
  radius: f32,
  color: Rgb,
  window_rect: Rect,
}

impl Particle {
  pub fn new(window_rect: Rect) -> Self {
    Self {
      position: Point2::new(
        random_range(
          window_rect.left(),
          window_rect.right(),
        ),
        random_range(
          window_rect.bottom(),
          window_rect.top(),
        ),
      ),
      radius: 1.0,
      color: GRAY,
      window_rect: window_rect,
    }
  }

  pub fn display(&self, draw: &Draw) {
    draw
      .ellipse()
      .color(self.color)
      .w(self.radius)
      .h(self.radius)
      .xy(self.position);
  }

  pub fn update(&mut self, field: &ForceField) {
    self.position.x +=
      field.get_x(self.position.x, self.position.y);
    self.position.y +=
      field.get_y(self.position.x, self.position.y) + 2.0;

    if self.position.x < self.window_rect.left()
      || self.position.x > self.window_rect.right()
      || self.position.y > self.window_rect.top()
    {
      self.respawn();
    }
  }

  fn respawn(&mut self) {
    self.position.x = random_range(
      self.window_rect.left() / 3.0,
      self.window_rect.right() / 3.0,
    );
    self.position.y = random_range(
      self.window_rect.bottom(),
      self.window_rect.bottom() + 100.0,
    ) - 100.0;
  }
}
