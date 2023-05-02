mod mandelbrot_set;

use mandelbrot_set::*;
use nannou::{prelude::*, wgpu::*};

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: Texture,
    mandelbrot_set: MandelbrotSet,
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

    let target_area = Rect::from_xy_wh_f64(
        dvec2(0.0, 0.0),
        dvec2(4.0, 4.0),
    );

    Model {
        texture: texture,
        mandelbrot_set: MandelbrotSet::new(
            1024.0,
            1024.0,
            target_area,
        ),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let image = model.mandelbrot_set.make_image();
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
