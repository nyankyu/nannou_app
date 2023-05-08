mod mandelbrot_set;

use std::process::exit;

use mandelbrot_set::*;
use nannou::{prelude::*, wgpu::*};

//const ZOOM_RATIO: f32 = 0.9;
const ZOOM_RATIO: f32 = 0.995;
const FILE_MAX: u32 = 10000;

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    texture: Texture,
    mandelbrot_set: MandelbrotSet,
    draw_frame: u64,
    zoom_in: bool,
    file_num: u32,
    auto: bool,
    auto_target: Vec2,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(3555, 2000)
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
            win_rect.w(),
            win_rect.h(),
        ),
        draw_frame: 0,
        zoom_in: true,
        file_num: 0,
        auto: false,
        auto_target: vec2(-76.171875, 513.993),
        //auto_target: vec2(-39.0, 264.7),
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if model.auto && model.draw_frame < app.elapsed_frames()
    {
        model.mandelbrot_set.update(
            next_target(&mut model.auto_target),
            true,
        );
        model.draw_frame = app.elapsed_frames() + 1;
        model.file_num += 1;
    }

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
            KeyPressed(Key::Space) => {
                model.auto = true;
                model
                    .mandelbrot_set
                    .change_zoo_ratio(ZOOM_RATIO);
                model.mandelbrot_set.update(
                    next_target(&mut model.auto_target),
                    true,
                );
                model.draw_frame = app.elapsed_frames() + 1;
                model.file_num += 1;
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
    model.file_num += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    if model.file_num > FILE_MAX {
        exit(0);
    }

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

    //save_frame(app, model.file_num);
}

#[allow(dead_code)]
fn save_frame(app: &App, file_num: u32) {
    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join(file_num.to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}

fn next_target(target: &mut Vec2) -> Vec2 {
    let next = *target * 0.005;
    *target -= next;
    *target *= ZOOM_RATIO.inv();
    next
}
