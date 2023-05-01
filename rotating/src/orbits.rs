mod thing;

use nannou::prelude::*;
use thing::*;

pub struct Orbits {
    orbit_vec: Vec<Orbit>,
}

impl Orbits {
    pub fn new(num: usize) -> Self {
        Self {
            orbit_vec: (0..num)
                .map(|_| Orbit::new())
                .collect(),
        }
    }

    pub fn update(&mut self) {
        self.orbit_vec
            .iter_mut()
            .for_each(|orbit| orbit.update());
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .w_h(900.0, 900.0)
            .stroke_weight(2.0)
            .no_fill()
            .stroke_color(rgba(1.0, 1.0, 1.0, 0.01));

        self.orbit_vec
            .iter()
            .for_each(|orbit| orbit.display(draw));
    }
}

struct Orbit {
    things: Vec<Thing>,
    orbital_param: Vec3,
}

impl Orbit {
    fn new() -> Self {
        let rnd = random_f32();
        Self {
            things: (0..40)
                .map(|i| {
                    Thing::new(
                        rnd + 0.17 * i as f32,
                        rnd + 0.013 * i as f32,
                    )
                })
                .collect(),
            orbital_param: pt3(
                random_range(0.1, 0.8),
                random_range(0.1, 0.8),
                random_range(0.1, 0.8),
            ),
        }
    }

    fn update(&mut self) {
        self.things.iter_mut().for_each(|thing| {
            thing.update(self.orbital_param)
        });
    }

    fn display(&self, draw: &Draw) {
        self.things
            .iter()
            .for_each(|thing| thing.display(draw));
    }
}
