mod mandelbrot_set;

use mandelbrot_set::*;
use nannou::{prelude::*, wgpu::*};

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    texture: Texture,
    mandelbrot_set: MandelbrotSet,
    draw_frame: u64,
    zoom_in: bool,
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

    Model {
        texture: texture,
        mandelbrot_set: MandelbrotSet::new(
            win_rect.h(),
            win_rect.w(),
        ),
        draw_frame: 0,
        zoom_in: true,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _id,
            simple: Some(window_event),
        } => match window_event {
            MousePressed(MouseButton::Left) => {
                model.zoom_in = true;
                update(app, model)
            }
            MousePressed(MouseButton::Right) => {
                model.zoom_in = false;
                update(app, model)
            }
            _ => (),
        },
        _ => (),
    }
}

fn update(app: &App, model: &mut Model) {
    model.mandelbrot_set.update(
        vec2(app.mouse.x, app.mouse.y),
        model.zoom_in,
    );
    model.draw_frame = app.elapsed_frames() + 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() != model.draw_frame {
        return;
    }

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
