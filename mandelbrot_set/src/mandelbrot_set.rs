use nannou::{
    image::{self, ImageBuffer},
    prelude::*,
};

pub(crate) struct MandelbrotSet {
    window_h: f64,
    window_w: f64,
    target_area: Rect<f64>,
}

impl MandelbrotSet {
    pub(crate) fn new(
        h: f64,
        w: f64,
        target_area: Rect<f64>,
    ) -> Self {
        Self {
            window_h: h,
            window_w: w,
            target_area: target_area,
        }
    }

    pub(crate) fn make_image(
        &self,
    ) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        ImageBuffer::from_fn(
            self.window_w as u32,
            self.window_h as u32,
            |x, y| {
                let (x, y) = self.pixel_to_complex(
                    self.target_area,
                    x,
                    y,
                );
                let r = escape_time(x, y, 1000);
                image::Rgba([
                    (r % 255) as u8,
                    (r % 255) as u8,
                    (r % 255) as u8,
                    255,
                ])
            },
        )
    }

    fn pixel_to_complex(
        &self,
        target_area: Rect<f64>,
        x: u32,
        y: u32,
    ) -> (f64, f64) {
        let scale_x = target_area.w() / self.window_w;
        let scale_y = target_area.h() / self.window_h;
        (
            target_area.left() + x as f64 * scale_x,
            target_area.top() - y as f64 * scale_y,
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
