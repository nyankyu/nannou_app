mod mandelbrot_set;

use std::process::exit;

use mandelbrot_set::MandelbrotSet;
use nannou::{
    image::{self, ImageBuffer},
    prelude::*,
    wgpu::*,
};
use num::{complex::Complex64, Complex};

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: Texture,
    //mandelbrot_set: MandelbrotSet,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let window = app.main_window();
    let win_rect = window.rect();
    let texture = TextureBuilder::new()
        .size([win_rect.w() as u32, win_rect.h() as u32])
        .format(TextureFormat::Rgba8Unorm)
        .usage(
            TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING,
        )
        .build(window.device());

    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win_rec = app.window_rect();

    let image = ImageBuffer::from_fn(
        win_rec.w() as u32,
        win_rec.h() as u32,
        |x, y| {
            let (x, y) = pixel_to_complex(x, y);
            let r = escape_time(x, y, 512);
            image::Rgba([
                ((r * 10) % 255) as u8,
                ((r * 30) % 255) as u8,
                ((r * 80) % 255) as u8,
                255,
            ])
        },
    );

    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut frame.command_encoder(),
        flat_samples.as_slice(),
    );

    let draw = app.draw();

    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
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

fn pixel_to_complex(x: u32, y: u32) -> (f64, f64) {
    let scale = 4.0 / 1024.0;
    (-2.0 + x as f64 * scale, 2.0 - y as f64 * scale)
}
