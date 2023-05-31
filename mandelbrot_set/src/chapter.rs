use crate::Model;
use nannou::prelude::*;

pub(crate) fn update(app: &App, model: &mut Model) {
    let count = app.elapsed_frames();
    match count {
        0..=100 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 0, 100, 0.0, -0.125),
            map_range(count, 0, 100, 0.0, 0.7445),
            map_range(count, 0, 100, 1.0, 0.37),
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
        0..=100 => {
            draw_mandelbrot(app, model, frame, &draw);
        }
        201..=300 => {
            draw_mandelbrot(app, model, frame, &draw);
            draw_branch3_1(app, model, &frame, &draw);
        }
        _ => {
            draw_mandelbrot(app, model, &frame, &draw);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_branch3_1(
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
