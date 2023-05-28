mod mandelbrot_set;

use nannou::prelude::*;
use std::process::exit;

const WINDOW_H: u32 = 1920;
const WINDOW_W: u32 = 1080;
const SCALSE: f32 = 0.333 * WINDOW_H as f32;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_W, WINDOW_H)
        .view(view)
        .build()
        .unwrap();

    Model {
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
}

fn event(app: &App, model: &mut Model, event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let f_count = app.elapsed_frames();

    match f_count {
        0..=700 => cardioid(app, model, frame),
        _ => mandelbrot(app, model, frame),
    }

    save_frame(app);
}

fn mandelbrot(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .translate(vec3(100.0, 0.0, 0.0))
        .scale(1.0)
        .z_radians(0.0);

    // mandelbrot set
    let assets = app.assets_path().unwrap();
    let img_path =
        assets.join("mandelbrot_set").join("1.png");
    let texture =
        wgpu::Texture::from_path(app, img_path).unwrap();
    draw.texture(&texture);

    draw.to_frame(app, &frame).unwrap();
}

fn cardioid(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .translate(vec3(100.0, 0.0, 0.0))
        .scale(1.0)
        .z_radians(0.0);

    // mandelbrot set
    let assets = app.assets_path().unwrap();
    let img_path =
        assets.join("mandelbrot_set").join("1.png");
    let texture =
        wgpu::Texture::from_path(app, img_path).unwrap();
    draw.texture(&texture);

    // === cardioid ===
    let theta = app.elapsed_frames() as f32 * (2.0 * PI / 700.0);

    // base circle
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(0.25 * SCALSE)
        .rgba8(0, 0, 0, 0)
        .stroke(BLUE)
        .stroke_weight(2.0);

    // move circle
    draw.ellipse()
        .xy(radian_to_position(theta, 0.5))
        .radius(0.25 * SCALSE)
        .rgba8(0, 0, 0, 0)
        .stroke(BLUE)
        .stroke_weight(2.0);

    let p = make_position(theta);
    draw.ellipse().xy(p).radius(4.0).color(RED);

    // line
    draw.polyline()
        .points(vec![
            radian_to_position(0.0, 0.25),
            vec2(0.0, 0.0),
            radian_to_position(theta, 0.5),
            p,
        ])
        .color(BLUE);

    // arc
    let arc = (0..100).map(|i| {
        let t = i as f32 * (theta / 100.0);
        pt2(t.cos(), t.sin()) * 30.0
    });
    draw.polyline().points(arc).color(WHITE);

    // angle
    draw.text(&format!(
        "{:3}°",
        (theta * 180.0 / PI).round()
    ))
    .color(WHITE)
    .font_size(20)
    .x_y(50.0, 30.0);

    draw.to_frame(app, &frame).unwrap();

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

fn radian_to_position(theta: f32, r: f32) -> Vec2 {
    let x = theta.cos() * r;
    let y = theta.sin() * r;
    vec2(x, y) * SCALSE
}

fn make_position(theta: f32) -> Vec2 {
    let x = 0.5 * theta.cos() - 0.25 * (2.0 * theta).cos();
    let y = 0.5 * theta.sin() - 0.25 * (2.0 * theta).sin();
    vec2(x, y) * SCALSE
}
