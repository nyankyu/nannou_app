use std::{u32::MAX, vec};

use nannou::prelude::*;
use num_complex::Complex32;

use crate::{WINDOW_H, WINDOW_W, ITERATION};

type Complex = Complex32;

const TO_COMPLEX: f32 = 4.0 / WINDOW_H as f32;
const TO_PIXEL: f32 = WINDOW_H as f32 / 4.0;

pub(crate) struct Particle {
    c: Complex,
    locus: Vec<Complex>,
    pub(crate) last: Complex,
    pub(crate) escap: u32,
    pub(crate) on_screen: bool,
    pub(crate) color: Hsl,
}

impl Particle {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        let re = x * TO_COMPLEX;
        let im = y * TO_COMPLEX;
        let z = Complex::new(re, im);

        let r = (x.abs() as u32 % 256) as u8;
        let g = (y.abs() as u32 % 256) as u8;
        Self {
            c: z,
            locus: vec![Complex::zero()],
            last: Complex::zero(),
            escap: 0,
            on_screen: true,
            color: hsl(x / WINDOW_W as f32, 1.0, 0.5),
        }
    }

    pub(crate) fn get_c(&self) -> Vec2 {
        vec2(self.c.re * TO_PIXEL, self.c.im * TO_PIXEL)
    }

    pub(crate) fn get_last(&self) -> Vec2 {
        let &last = self.locus.last().unwrap();
        vec2(last.re * TO_PIXEL, last.im * TO_PIXEL)
    }

    pub(crate) fn get_nth(&self, n: u64) -> Option<Vec2> {
        let n = n as usize;

        if n > self.locus.len() - 1 {
            return None;
        }

        Option::Some(to_pixel(self.locus[n]))
    }

    pub(crate) fn get_middle_pow2(&self) -> (Vec2, Vec2) {
        let i = self.locus.len() - 1;
        (
            to_pixel(self.locus[i - 1]),
            to_pixel(self.locus[i]),
        )
    }

    pub(crate) fn update(&mut self, t: u64) {
        if !self.on_screen {
            return;
        }

        if t > 0 {
            let (r, mut theta) = self.last.to_polar();
            if theta < 0.0 {
                theta += TAU;
            }

            let theta_t = map_range(t, 1, 99, 0.0, theta);
            let r_t = map_range(t, 1, 99, 1.0, r);
            let z_t = Complex::from_polar(r_t, theta_t);
            let c_t = self.c * (t as f32 / 99.0);

            let middle = self.last * z_t + c_t;
            self.locus.push(middle);

            return;
        }

        self.last = self.last.powu(2) + self.c;
        self.locus.push(self.last);

        if is_escap(self.last) {
            let h = self.escap as f32 / ITERATION as f32;
            self.color = hsl(h, 1.0, 0.5);
        } else {
            self.escap += 1;
        }

        self.on_screen = on_screen(self.last);
    }
}

fn is_escap(z: Complex) -> bool {
    z.norm_sqr() >= 4.0
}

fn on_screen(z: Complex) -> bool {
    let z = z * TO_PIXEL;

    z.re.abs() < std::cmp::max(WINDOW_W, WINDOW_H) as f32 / 2.0
        && z.im.abs() < std::cmp::max(WINDOW_W, WINDOW_H) as f32 / 2.0
}

fn to_pixel(z: Complex) -> Vec2 {
    vec2(z.re * TO_PIXEL, z.im * TO_PIXEL)
}
