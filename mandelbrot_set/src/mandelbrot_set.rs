mod auto_zoom;
mod coloring;
mod target_area;

use auto_zoom::*;
use coloring::*;
use nannou::{
    image::{ImageBuffer, Rgba},
    prelude::*,
};
use rayon::prelude::*;
use target_area::*;

pub(crate) const ITERATION_LIMIT: u32 = 20_000;

pub(crate) struct MandelbrotSet {
    target: TargetArea,
    iteration_counts: Vec<u32>,
    draw_count: u32,
    coloring: Coloring,
    auto_zoom: AutoZoom,
}

impl MandelbrotSet {
    pub(crate) fn new(
        window_w: f32,
        window_h: f32,
    ) -> Self {
        let mut mandelbrot_set = MandelbrotSet {
            target: TargetArea::new(window_w, window_h),
            iteration_counts: vec![
                0;
                (window_w * window_h)
                    as usize
            ],
            draw_count: 0,
            auto_zoom: AutoZoom::new(false),
            coloring: Coloring::new(
                ColoringMethod::GrayBackAndFroth,
            ),
        };
        mandelbrot_set.count_iteration();

        mandelbrot_set
    }

    pub(crate) fn make_image(
        &self,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        ImageBuffer::from_fn(
            self.target.window_w,
            self.target.window_h,
            |x, y| {
                let index =
                    (y * self.target.window_w + x) as usize;
                let count = self.iteration_counts[index];
                self.coloring.get(count)
            },
        )
    }

    pub(crate) fn zoom_to_click(
        &mut self,
        win_rect: Rect,
        x: f32,
        y: f32,
        magnification: f32,
    ) {
        self.draw_count += 1;

        let diff_x = (x - win_rect.x()) as f64
            * self.target.per_pixel;
        let diff_y = (y - win_rect.y()) as f64
            * self.target.per_pixel;
        self.target.move_by_vector(dvec2(diff_x, diff_y));
        self.target.zoom(magnification as f64);

        self.count_iteration();
    }

    pub(crate) fn auto(&mut self) {
        self.auto_zoom.auto();
    }

    pub(crate) fn is_auto(&self) -> bool {
        self.auto_zoom.is_auto()
    }

    pub(crate) fn auto_next(&mut self) {
        self.auto_zoom.next(&mut self.target);
        self.count_iteration();
    }

    fn count_iteration(&mut self) {
        let band_size = self.target.window_w as usize;
        let bands: Vec<(usize, &mut [u32])> = self
            .iteration_counts
            .chunks_mut(band_size)
            .enumerate()
            .collect();

        bands.into_par_iter().for_each(
            |(pixel_y, band)| {
                for pixel_x in 0..band_size {
                    band[pixel_x] = Self::escape_time(
                        self.target.base[0]
                            + pixel_x as f64
                                * self.target.per_pixel,
                        self.target.base[1]
                            - pixel_y as f64
                                * self.target.per_pixel,
                        ITERATION_LIMIT,
                    );
                }
            },
        );
    }

    fn escape_time(x: f64, y: f64, limit: u32) -> u32 {
        let mut re2 = x * x;
        let mut im2 = y * y;
        let mut re = x;
        let mut im = y;

        // cardioid check
        let q = re2 - 0.5 * re + 0.0625 + im2;
        if im2 >= 4.0 * q * (q + re - 0.25) {
            return limit;
        }
        // bulb check
        if (re + 1.0) * (re + 1.0) + im2 < 0.0625 {
            return limit;
        }

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

            // periodicity check
            if abs(re - old_re) < 1e-11
                && abs(im - old_im) < 1e-11
            {
                i = limit;
                break;
            }

            period += 1;
            if period > 2000 {
                period = 0;
                old_re = re;
                old_im = im;
            }
        }

        return i;
    }
}
