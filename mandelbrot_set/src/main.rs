mod mandelbrot_set;

use std::process::exit;

use mandelbrot_set::*;
use nannou::{prelude::*, wgpu::*};

const FILE_MAX: u32 = 5000;
const WINDOW_H: u32 = 720;
const WINDOW_W: u32 = 1280;

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
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_W, WINDOW_H)
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
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if model.auto && model.draw_frame < app.elapsed_frames()
    {
        model.mandelbrot_set.auto_next();
        model.draw_frame = app.elapsed_frames() + 1;
        model.file_num += 1;
        //println!("{}", model.file_num);
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
                model.auto ^= true;
                if !model.auto {
                    model.mandelbrot_set.auto_set(false);
                    return;
                }

                model.mandelbrot_set.auto_set(true);
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
