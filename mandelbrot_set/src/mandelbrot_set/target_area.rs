use nannou::prelude::*;

pub(super) struct TargetArea {
    pub(super) window_w: u32,
    pub(super) window_h: u32,
    pub(super) center: DVec2,
    pub(super) base: DVec2,
    pub(super) per_pixel: f64,
}

impl TargetArea {
    pub(super) fn new(w: f32, h: f32) -> Self {
        Self {
            window_w: w as u32,
            window_h: h as u32,
            center: dvec2(0.0, 0.0),
            base: dvec2(-2.0 * (w / h) as f64, 2.0),
            per_pixel: 4.0 / h as f64,
        }
    }

    pub(super) fn move_by_vector(&mut self, vector: DVec2) {
        self.center += vector;
        self.base += vector;
    }

    pub(super) fn move_to(&mut self, to: DVec2) {
        let diff = to - self.center;
        self.center = to;
        self.base += diff;
    }

    pub(super) fn zoom(&mut self, magnification: f64) {
        self.base = (self.base - self.center)
            * magnification
            + self.center;
        self.per_pixel *= magnification;
    }

    pub(crate) fn change(
        &mut self,
        center: DVec2,
        magnification: f64,
    ) {
        self.center = dvec2(0.0, 0.0);
        self.base = dvec2(
            -2.0 * (self.window_w / self.window_h) as f64,
            2.0,
        );
        self.per_pixel = 4.0 / self.window_h as f64;

        self.move_to(center);
        self.zoom(magnification);
    }
}
