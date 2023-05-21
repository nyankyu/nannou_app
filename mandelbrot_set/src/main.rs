mod mandelbrot_set;

use mandelbrot_set::*;
use nannou::{prelude::*, wgpu::*};
use std::process::exit;

const FILE_MAX: u32 = 5000;
//const WINDOW_H: u32 = 720;
const WINDOW_H: u32 = 1280;
const WINDOW_W: u32 = 1280;
const ZOOM_RATIO: f32 = 0.6;

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    texture: Texture,
    mandelbrot_set: MandelbrotSet,
    draw_frame: u64,
    file_num: u32,
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
        file_num: 0,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    if model.mandelbrot_set.is_auto()
        && model.draw_frame < app.elapsed_frames()
    {
        model.mandelbrot_set.auto_next();
        draw_next_frame(app, model);
    }

    match event {
        Event::WindowEvent {
            id: _id,
            simple: Some(window_event),
        } => match window_event {
            MousePressed(MouseButton::Left) => {
                model.mandelbrot_set.zoom_to_click(
                    app.window_rect(),
                    app.mouse.x,
                    app.mouse.y,
                    ZOOM_RATIO,
                );
                draw_next_frame(app, model);
            }
            MousePressed(MouseButton::Right) => {
                model.mandelbrot_set.zoom_to_click(
                    app.window_rect(),
                    app.mouse.x,
                    app.mouse.y,
                    ZOOM_RATIO.inv(),
                );
                draw_next_frame(app, model);
            }
            KeyPressed(Key::Space) => {
                model.mandelbrot_set.auto();
            }
            _ => (),
        },
        _ => (),
    }
}

fn draw_next_frame(app: &App, model: &mut Model) {
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
        .join("mandelbrot_set")
        .join(file_num.to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}
