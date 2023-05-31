use crate::Model;
use nannou::prelude::*;

const WINDOW_H: u32 = 1920;
const WINDOW_W: u32 = 1080;
const SCALSE: f32 = 0.333 * WINDOW_H as f32;

pub(crate) fn update(app: &App, model: &mut Model) {
    let count = app.elapsed_frames();
    match count {
        0 => model.mandelbrot_set.zoom_to_click(
            app.window_rect(),
            0.0,
            0.0,
            0.75,
        ),
        301..=400 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 301, 400, 0.0, -0.125),
            map_range(count, 301, 400, 0.0, 0.7445),
            map_range(count, 301, 400, 0.75, 0.37),
        ),
        501..=600 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 501, 600, -0.125, 0.0),
            map_range(count, 501, 600, 0.7445, 0.0),
            map_range(count, 501, 600, 0.37, 0.75),
        ),
        801..=900 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 801, 900, 0.0, 0.379),
            map_range(count, 801, 900, 0.0, 0.336),
            map_range(count, 801, 900, 0.75, 0.09),
        ),
        1001..=1100 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 1001, 1100, 0.379, 0.0),
            map_range(count, 1001, 1100, 0.336, 0.0),
            map_range(count, 1001, 1100, 0.09, 0.75),
        ),
        1401..=1500 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 1401, 1500, 0.0, -0.583),
            map_range(count, 1401, 1500, 0.0, -0.5805),
            map_range(count, 1401, 1500, 0.75, 0.09),
        ),
        _ => (),
    }
}

pub(crate) fn view(
    app: &App,
    model: &Model,
    frame: &Frame,
) {
    frame.clear(BLACK);

    let draw = app.draw();

    let f_count = app.elapsed_frames();
    match f_count {
        0..=232 => {
            let count = f_count as f32;
            draw_cardioid(app, model, frame, &draw, count);
        }
        233..=300 => {
            draw_cardioid(app, model, frame, &draw, 233.0);
            draw_ratio3(app, model, &frame, &draw);
        }
        301..=400 => {
            draw_mandelbrot(app, model, frame, &draw);
            draw_ratio3(app, model, &frame, &draw);
        }
        401..=500 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_ratio3(app, model, &frame, &draw);
            draw_branch3(app, model, &frame, &draw);
        }
        501..=600 => {
            draw_mandelbrot(app, model, &frame, &draw);
        }
        601..=693 => {
            let count =
                map_range(f_count, 601, 693, 232, 140);
            draw_cardioid(
                app,
                model,
                frame,
                &draw,
                count as f32,
            );
        }
        694..=800 => {
            draw_cardioid(app, model, frame, &draw, 140.0);
            draw_ratio5(app, model, &frame, &draw);
        }
        801..=900 => {
            draw_mandelbrot(app, model, frame, &draw);
            draw_ratio5(app, model, &frame, &draw);
        }
        901..=1000 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_ratio5(app, model, &frame, &draw);
            draw_branch5(app, model, &frame, &draw);
        }
        1001..=1100 => {
            draw_mandelbrot(app, model, &frame, &draw);
        }
        1101..=1381 => {
            let count =
                map_range(f_count, 1101, 1381, 140, 420);
            draw_cardioid(
                app,
                model,
                frame,
                &draw,
                count as f32,
            );
        }
        1382..=1400 => {
            draw_cardioid(app, model, frame, &draw, 420.0);
            draw_ratio5_3(app, model, &frame, &draw);
        }
        1401..=1500 => {
            draw_mandelbrot(app, model, frame, &draw);
            draw_ratio5_3(app, model, &frame, &draw);
        }
        1501..=1600 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_ratio5_3(app, model, &frame, &draw);
            draw_branch5_3(app, model, &frame, &draw);
        }
        _ => {
            draw_mandelbrot(app, model, &frame, &draw);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_branch3(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(30.0, 220.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(80.0, 320.0);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(-10.0, 320.0);
}

fn draw_branch5(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(250.0, 20.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(320.0, -20.0);
    draw.text("3").color(RED).font_size(50).x_y(370.0, 0.0);
    draw.text("4")
        .color(RED)
        .font_size(50)
        .x_y(380.0, 60.0);
    draw.text("5")
        .color(RED)
        .font_size(50)
        .x_y(320.0, 120.0);
}

fn draw_branch5_3(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(172.0, -249.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(10.714844, -253.55469);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(-11.03125, -397.52734);
    draw.text("4")
        .color(RED)
        .font_size(50)
        .x_y(104.17578, -475.46094);
    draw.text("5")
        .color(RED)
        .font_size(50)
        .x_y(237.41797, -426.98438);
}

fn draw_ratio3(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("120° / 360° = 1 / ")
        .color(WHITE)
        .font_size(70)
        .w(1000.0)
        .x_y(0.0, -300.0);
    draw.text("3")
        .color(RED)
        .font_size(70)
        .w(100.0)
        .x_y(300.0, -300.0);
}

fn draw_ratio5(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("72° / 360° = 1 / ")
        .color(WHITE)
        .font_size(70)
        .w(1000.0)
        .x_y(0.0, -300.0);
    draw.text("5")
        .color(RED)
        .font_size(70)
        .w(100.0)
        .x_y(275.0, -300.0);
}

fn draw_ratio5_3(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("216° / 360° = 3 / ")
        .color(WHITE)
        .font_size(70)
        .w(1000.0)
        .x_y(0.0, 300.0);
    draw.text("5")
        .color(RED)
        .font_size(70)
        .w(100.0)
        .x_y(275.0, 300.0);
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

fn draw_cardioid(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
    count: f32,
) {
    // mandelbrot set
    draw_mandelbrot(app, model, frame, draw);

    // === cardioid ===
    let theta = count * (TAU / 700.0);

    // base circle
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(0.25 * SCALSE)
        .rgba8(0, 0, 0, 0)
        .stroke(BLUE)
        .stroke_weight(3.0);

    // move circle
    draw.ellipse()
        .xy(radian_to_position(theta, 0.5))
        .radius(0.25 * SCALSE)
        .rgba8(0, 0, 0, 0)
        .stroke(BLUE)
        .stroke_weight(3.0);

    let p = make_position(theta);
    draw.ellipse().xy(p).radius(4.0).color(RED);

    // line
    draw.polyline()
        .weight(3.0)
        .points(vec![
            radian_to_position(0.0, 0.25),
            vec2(0.0, 0.0),
            radian_to_position(theta, 0.5),
            p,
        ])
        .color(BLUE);

    // arc
    let arc1 = (0..100).map(|i| {
        let t = i as f32 * (theta / 100.0);
        pt2(t.cos(), t.sin()) * 30.0
    });
    draw.polyline().weight(3.0).points(arc1).color(WHITE);

    let arc2 = (0..100).map(|i| {
        let t = i as f32 * (theta / 100.0) - PI + theta;
        pt2(t.cos(), t.sin()) * 30.0
            + radian_to_position(theta, 0.5)
    });
    draw.polyline().weight(3.0).points(arc2).color(WHITE);

    // angle
    draw.text(&format!(
        "{:3}°",
        (theta * 180.0 / PI).round()
    ))
    .color(WHITE)
    .font_size(30)
    .x_y(60.0, 30.0);
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
