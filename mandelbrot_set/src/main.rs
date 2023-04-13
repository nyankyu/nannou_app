mod mandelbrot_set;

use mandelbrot_set::MandelbrotSet;
use nannou::{
    image::{self, ImageBuffer},
    prelude::*,
    wgpu::*,
};

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
            image::Rgba([
                (x + y) as u8 % 255,
                y as u8 % 255,
                x as u8 % 255,
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
