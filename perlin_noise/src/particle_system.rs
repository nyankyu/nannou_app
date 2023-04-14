mod force_field;
mod particle;

use force_field::*;
use nannou::prelude::*;
use particle::Particle;

pub struct ParticleSystem {
  field: ForceField,
  particles: Vec<Particle>,
}

impl ParticleSystem {
  pub fn new(
    win_w: u32,
    win_h: u32,
    particle_num: usize,
  ) -> Self {
    let win_rect = Rect::from_x_y_w_h(
      0.0,
      0.0,
      win_w as f32,
      win_h as f32,
    );

    let mut particles = Vec::with_capacity(particle_num);
    for _i in 0..particle_num {
      particles.push(Particle::new(win_rect));
    }

    Self {
      field: ForceField::new(),
      particles: particles,
    }
  }

  pub fn update(&mut self) {
    self.field.update();

    self
      .particles
      .iter_mut()
      .for_each(|p| p.update(&self.field));
  }

  pub fn display(&self, draw: &Draw) {
    self.particles.iter().for_each(|p| p.display(draw));
  }
}
