use nannou::{
    image::{self, ImageBuffer},
    prelude::*,
};
use rayon::prelude::*;

const ZOOM_RATIO: f64 = 0.9;
const AUTO_ZOOM_RATIO: f64 = 0.995;
const AUTO_ZOOM_TARGET: (f64, f64) =
    (-0.15773294549873199, 1.0253132341831026);
//(-0.16656641350852985, 1.0362047515983437);

pub(crate) struct MandelbrotSet {
    window_h: u32,
    window_w: u32,
    base_point: DVec2,
    center: DVec2,
    scale: f64,
    auto: bool,
    i: u32,
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
        }
    }

    pub(crate) fn make_image(
        &self,
    ) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let limit: u32 = 10_000;
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

        /*
                let mut count_per_limit = vec![0; limit as usize];
                for &count in &iteration_counts {
                    if count == limit {
                        continue;
                    }
                    count_per_limit[count as usize] += 1;
                }
                for i in 1..limit as usize {
                    count_per_limit[i] += count_per_limit[i - 1];
                }
        */

        ImageBuffer::from_fn(
            self.window_w,
            self.window_h,
            |x, y| {
                let index =
                    (y * self.window_w + x) as usize;
                /*
                let r =
                    (iteration_counts[index] % 256) as u8;
                    */
                /*
                let r = iteration_counts[index] % 510;
                let r = if r < 256 {
                    r as u8
                } else {
                    (510 - r) as u8
                };
                */
                /*
                let r = iteration_counts[index] % 256;
                let r = if r % 10 == 0 {
                    255
                } else {
                    r as u8
                };
                */
                /*
                let r = if iteration_counts[index] % 2 == 0 {
                    50
                } else {
                    150
                };
                */
                //let r = (iteration_counts[index] % 4) as u8 * 80;
                /*
                let r = if iteration_counts[index] == limit {
                    0
                } else {
                    (self.i + iteration_counts[index] % 256) as u8
                };
                */
                /*
                let count = iteration_counts[index];
                let r = if count == limit {
                    0
                } else {
                    (count_per_limit[count as usize] as f32
                        / count_per_limit
                            [limit as usize - 1]
                            as f32
                        * 255.0) as u8
                };
                */

                let count = iteration_counts[index];
                let h = count % 200;
                let hue = if h < 100 { 0.65 } else { 0.0 };
                let l = if h < 100 {
                    h as f32 / 100.0
                } else {
                    1.0 - ((h - 100) as f32 / 100.0)
                };
                image::Rgba(hsl_to_rgba(hue, 1.0, l))
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
        //println!("{:1.30}", self.center);
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

        //println!("{:1.30}", self.center);
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

fn hsl_to_rgba(h: f32, s: f32, l: f32) -> [u8; 4] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let (r, g, b) = (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    );

    [r, g, b, 255]
}
