use std::f64::consts::FRAC_PI_2;

use crate::Model;
use nannou::prelude::*;

enum Chapter {
    LookAround,
}

pub(crate) fn update(app: &App, model: &mut Model) {
    //let count = app.elapsed_frames();
    //move_to_bulb(app, model, count);
}

pub(crate) fn view(
    app: &App,
    model: &Model,
    frame: &Frame,
) {
    frame.clear(BLACK);

    let draw = app.draw();

    draw_mandelbrot(app, model, frame, &draw);

    let p = (app.elapsed_frames() + 1) as f32;
    let q = (app.elapsed_frames() * 2 + 3) as f32;
    let inv_q = q.inv();
    let theta = TAU * p * inv_q;
    let theta2 = 2.0 * theta;
    let x = 0.5 * theta.cos() - 0.25 * theta2.cos();
    let y = 0.5 * theta.sin() - 0.25 * theta2.sin();
    draw.ellipse()
        .xy(vec2(x + 0.516, y) * 960.0 / 4.0)
        .radius(10.0)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
}

struct BulbPoint {
    x: f64,
    y: f64,
    rotate: f64,
    magnification: f64,
}

impl BulbPoint {
    fn new(p: u64, q: u64) -> Self {
        let p = p as f64;
        let inv_q = 1.0 / q as f64;
        let inv_q2 = inv_q / q as f64;
        let theta = TAU_F64 * p * inv_q;
        let theta2 = 2.0 * theta;

        let x = 0.5 * theta.cos() - 0.25 * theta2.cos();
        let y = 0.5 * theta.sin() - 0.25 * theta2.sin();
        let r = inv_q2 * (PI_F64 * p * inv_q).sin();
        let rotate = ((theta.cos() - theta2.cos())
            / (theta.sin() - theta2.sin()))
        .atan();

        Self {
            x: x,
            y: y,
            rotate: rotate,
            magnification: 4.0 * r,
        }
    }
}
fn move_to_bulb(app: &App, model: &mut Model, count: u64) {
    let q = 2 * count + 3;
    let p = count + 1;
    let bulb = BulbPoint::new(p, q);
    model.mandelbrot_set.zoom_to_point(
        bulb.x,
        bulb.y,
        bulb.magnification,
        bulb.rotate,
    );
}

fn draw_mandelbrot(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    let image = model.mandelbrot_set.make_image();
    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut frame.command_encoder(),
        flat_samples.as_slice(),
    );
    draw.texture(&model.texture);
}
