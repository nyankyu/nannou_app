use nannou::{
    image::{self, ImageBuffer},
    prelude::*,
};

pub(crate) struct MandelbrotSet {
    window_h: f64,
    window_w: f64,
    base_point: DVec2,
    center: DVec2,
    scale: f64,
}

impl MandelbrotSet {
    pub(crate) fn new(h: f32, w: f32) -> Self {
        Self {
            window_h: h as f64,
            window_w: w as f64,
            base_point: dvec2(-2.0, 2.0),
            center: dvec2(0.0, 0.0),
            scale: 4.0 / w as f64,
        }
    }

    pub(crate) fn make_image(
        &self,
    ) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        ImageBuffer::from_fn(
            self.window_w as u32,
            self.window_h as u32,
            |x, y| {
                let (x, y) = self.pixel_to_complex(x, y);
                let r = escape_time(x, y, 10000);
                image::Rgba([
                    (r % 255) as u8,
                    (r % 255) as u8,
                    (r % 255) as u8,
                    255,
                ])
            },
        )
    }

    pub(crate) fn update(
        &mut self,
        p: Vec2,
        zoom_in: bool,
    ) {
        let zoom_rate = if zoom_in { 0.5 } else { 2.0 };
        let point =
            dvec2(p.x as f64, p.y as f64) * self.scale;
        let to_base =
            (self.base_point - self.center) * zoom_rate;

        self.center += point;
        self.base_point = self.center + to_base;
        self.scale *= zoom_rate;
    }

    fn pixel_to_complex(
        &self,
        x: u32,
        y: u32,
    ) -> (f64, f64) {
        (
            self.base_point[0] + x as f64 * self.scale,
            self.base_point[1] - y as f64 * self.scale,
        )
    }
}

fn escape_time(x: f64, y: f64, limit: u32) -> u32 {
    let mut re2 = x * x;
    let mut im2 = y * y;
    let mut re = x;
    let mut im = y;

    let mut i = 0;
    while i < limit && re2 + im2 <= 4.0 {
        im = (re + re) * im + y;
        re = re2 - im2 + x;

        re2 = re * re;
        im2 = im * im;

        i += 1;
    }
    return i;
}
