use coloring::*;
use nannou::{
    image::{ImageBuffer, Rgba},
    prelude::*,
};
use rayon::prelude::*;

mod coloring;

const ZOOM_RATIO: f64 = 0.9;
const AUTO_ZOOM_RATIO: f64 = 0.998;
const AUTO_ZOOM_TARGET: (f64, f64) =
    //(-0.15773294549873199, 1.0253132341831026);
(-0.8606130991081128, -0.2351181260185022);
const ITERATION_LIMIT: u32 = 2_000;

pub(crate) struct MandelbrotSet {
    window_h: u32,
    window_w: u32,
    base_point: DVec2,
    center: DVec2,
    scale: f64,
    auto: bool,
    i: u32,
    //palette: Vec<Rgba<u8>>,
}

impl MandelbrotSet {
    pub(crate) fn new(w: f32, h: f32) -> Self {
        Self {
            window_w: w as u32,
            window_h: h as u32,
            base_point: dvec2(-2.0 * (w / h) as f64, 2.0),
            center: dvec2(0.0, 0.0),
            scale: 4.0 / h as f64,
            auto: false,
            i: 0,
            //palette: make_color_palette(ITERATION_LIMIT),
        }
    }

    pub(crate) fn make_image(
        &self,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
                    band[pixel_x] = Self::escape_time(
                        x,
                        y,
                        ITERATION_LIMIT,
                    );
                }
            },
        );

/*
        let heatmap = make_cumulative_heatmap(
            &iteration_counts,
            ITERATION_LIMIT,
        );
*/

        ImageBuffer::from_fn(
            self.window_w,
            self.window_h,
            |x, y| {
                let index =
                    (y * self.window_w + x) as usize;
                let count = iteration_counts[index];
                if count == ITERATION_LIMIT {
                    Rgba([0, 0, 0, 255])
                } else {
                    gray_cyclic(count)
                    //gray_back_and_froth(count)
                    //gray_strip(count)
                    //two_color(count)
                    //four_color(count)
                    //gray_cyclic_phase(count, self.i)
                    //gray_back_and_froth_phase(count, self.i)
                    //self.palette[count as usize]
                    //gray_heatmap(&heatmap, count)
                }
            },
        )
    }

    pub(crate) fn update(
        &mut self,
        p: Vec2,
        zoom_in: bool,
    ) {
        let zoom_rate = if zoom_in {
            ZOOM_RATIO
        } else {
            ZOOM_RATIO.inv()
        };
        let point =
            dvec2(p.x as f64, p.y as f64) * self.scale;
        let to_base =
            (self.base_point - self.center) * zoom_rate;

        self.center += point;
        self.base_point = self.center + to_base;
        self.scale *= zoom_rate;

        self.i += 1;
        println!("{:1.30}", self.center);
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

            if abs(re - old_re) < 1e-6
                && abs(im - old_im) < 1e-6
            {
                i = limit;
                break;
            }

            period += 1;
            if period > 20 {
                period = 0;
                old_re = re;
                old_im = im;
            }
        }
        return i;
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

    pub(crate) fn auto_set(&mut self, auto: bool) {
        self.auto = auto;
    }

    pub(crate) fn auto_next(&mut self) {
        let center_to_base = (self.base_point
            - self.center)
            * AUTO_ZOOM_RATIO;
        self.center +=
            (dvec2(AUTO_ZOOM_TARGET.0, AUTO_ZOOM_TARGET.1)
                - self.center)
                * (1.0 - AUTO_ZOOM_RATIO);
        self.base_point = self.center + center_to_base;
        self.scale *= AUTO_ZOOM_RATIO;
        self.i += 1;

        //println!("{:1.30}", self.center);
    }
}
