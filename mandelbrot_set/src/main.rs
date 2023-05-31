mod chapter;
mod mandelbrot_set;

use mandelbrot_set::*;
use nannou::{prelude::*, wgpu::*};

const WINDOW_H: u32 = 1920;
const WINDOW_W: u32 = 1080;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

pub(crate) struct Model {
    texture: Texture,
    mandelbrot_set: MandelbrotSet,
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
        texture,
        mandelbrot_set: MandelbrotSet::new(
            win_rect.w(),
            win_rect.h(),
        ),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    chapter::update(app, model);
}

fn event(app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _id,
            simple: Some(window_event),
        } => match window_event {
            MousePressed(MouseButton::Left) => {
                println!(
                    "x:{}, y:{}",
                    app.mouse.x, app.mouse.y
                );
            }
            MousePressed(MouseButton::Right) => {
                println!(
                    "x:{}, y:{}",
                    app.mouse.x, app.mouse.y
                );
            }
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    chapter::view(app, model, &frame);
    //save_frame(app);
}

#[allow(dead_code)]
fn save_frame(app: &App) {
    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join("mandelbrot_set")
        .join(app.elapsed_frames().to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}
