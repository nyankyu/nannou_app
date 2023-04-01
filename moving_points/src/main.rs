use nannou::prelude::*;
use nannou::noise::{Perlin, NoiseFn};
use nannou::color;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    points: Vec<Vec2>,
    noise: Perlin,
}

const WINDOW_W: u32 = 512;
const WINDOW_H: u32 = 512;
const MAX_POINTS: usize = 2000;
const INCR_POINTS: usize = 20;
const TAIL_LENG_COEFFICIENT: f32 = 0.025;

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_W, WINDOW_H)
        .view(view)
        .build()
        .unwrap();
    
    let points = Vec::new();
    let noise = Perlin::new();
    Model { points, noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.points.len() < MAX_POINTS {
        let win_w = app.window_rect().w();
        let win_h = app.window_rect().h();
        for _i in 0..INCR_POINTS {
            let p = vec2(
                (random_f32() - 0.5) * win_w,
                (random_f32() - 0.5) * win_h
            );
            model.points.push(p);
        } 
    } else {
        model.points = model.points[INCR_POINTS..].to_vec();
    }

    let sn = 0.01;
    for p in model.points.iter_mut() {
        *p += vec2(
            model.noise.get([sn * p.x as f64, sn * p.y as f64, 0.5]) as f32,
            model.noise.get([sn * p.x as f64, sn * p.y as f64, 1.5]) as f32
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() == 1 {
        frame.clear(BLACK);
    }

    let draw = app.draw();

    draw.rect()
        .wh(app.window_rect().wh())
        .color(rgba(0.0, 0.0, 0.0, TAIL_LENG_COEFFICIENT));

    for &p in model.points.iter() {
        draw.ellipse()
            .xy(p)
            .radius(1.0)
            .color(color::WHITESMOKE);
    }

    draw.to_frame(app, &frame).unwrap();
}