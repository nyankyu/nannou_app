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
        let limit: u32 = 256_000;
        let mut iteration_counts =
            vec![
                0;
                self.window_w as usize
                    * self.window_h as usize
            ];

        let bands: Vec<(usize, &mut [u32])> =
            iteration_counts
                .chunks_mut(self.window_w as usize)
                .enumerate()
                .collect();

        bands.into_par_iter().for_each(
            |(pixel_y, band)| {
                for pixel_x in 0..self.window_w as usize {
                    let (x, y) = self.pixel_to_complex(
                        pixel_x as u32,
                        pixel_y as u32,
                    );
                    band[pixel_x] =
                        escape_time(x, y, limit);
                }
            },
        );

        ImageBuffer::from_fn(
            self.window_w,
            self.window_h,
            |x, y| {
                let index =
                    (y * self.window_h + x) as usize;
                let r =
                    (iteration_counts[index] % 256) as u8;
                image::Rgba([r, r, r, 255])
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

    let mut old_re = 0.0;
    let mut old_im = 0.0;
    let mut period = 0;

    let mut i = 0;
    while i < limit && re2 + im2 <= 4.0 {
        im = (re + re) * im + y;
        re = re2 - im2 + x;

        re2 = re * re;
        im2 = im * im;

        i += 1;

        if abs(re - old_re) < 1e-9
            && abs(im - old_im) < 1e-9
        {
            i = limit;
            break;
        }

        period += 1;
        if period > 30 {
            period = 0;
            old_re = re;
            old_im = im;
        }
    }
    return i;
}
