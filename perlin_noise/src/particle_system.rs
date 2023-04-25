mod color_noise;
mod force_field;
mod particle;

use color_noise::ColorNoise;
use force_field::*;
use nannou::prelude::*;
use particle::Particle;

pub struct ParticleSystem {
    win_rect: Rect,
    field: ForceField,
    particles: Vec<Particle>,
    color_noise: ColorNoise,
}

impl ParticleSystem {
    pub fn new(
        win_rect: Rect,
        particle_num: usize,
    ) -> Self {
        let mut particles =
            Vec::with_capacity(particle_num);
        for _i in 0..30 {
            particles.push(Particle::new(win_rect));
        }

        Self {
            win_rect: win_rect,
            field: ForceField::new(),
            particles: particles,
            color_noise: ColorNoise::new(),
        }
    }

    pub fn update(&mut self) {
        if self.particles.len() < self.particles.capacity()
        {
            for _i in 0..5 {
                self.particles
                    .push(Particle::new(self.win_rect))
            }
        }
        self.field.update();

        self.particles.iter_mut().for_each(|p| {
            p.update(&self.field, &mut self.color_noise)
        });
    }

    pub fn display(&self, draw: &Draw) {
        self.particles.iter().for_each(|p| p.display(draw));
    }
}
