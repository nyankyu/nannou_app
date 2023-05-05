use nannou::{
    image::{self, ImageBuffer},
    prelude::*,
};
use rayon::prelude::*;

pub(crate) struct MandelbrotSet {
    window_h: u32,
    window_w: u32,
    base_point: DVec2,
    center: DVec2,
    scale: f64,
}

impl MandelbrotSet {
    pub(crate) fn new(h: f32, w: f32) -> Self {
        Self {
            window_h: h as u32,
            window_w: w as u32,
            base_point: dvec2(-2.0, 2.0),
            center: dvec2(0.0, 0.0),
            scale: 4.0 / w as f64,
        }
    }

    pub(crate) fn make_image(
        &self,
    ) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let mut buf = vec![
            0;
            4 * self.window_w as usize
                * self.window_h as usize
        ];

        let bands: Vec<(usize, &mut [u8])> = buf
            .chunks_mut(4 * self.window_w as usize)
            .enumerate()
            .collect();

        bands.into_par_iter().for_each(
            |(pixel_y, band)| {
                for pixel_x in 0..self.window_w as usize {
                    let (x, y) = self.pixel_to_complex(
                        pixel_x as u32,
                        pixel_y as u32,
                    );
                    let r = escape_time(x, y, 1000);
                    band[pixel_x * 4] = (r % 255) as u8;
                    band[pixel_x * 4 + 1] = (r % 255) as u8;
                    band[pixel_x * 4 + 2] = (r % 255) as u8;
                    band[pixel_x * 4 + 3] = 255;
                }
            },
        );

        ImageBuffer::from_vec(
            self.window_w,
            self.window_h,
            buf,
        )
        .unwrap()
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
