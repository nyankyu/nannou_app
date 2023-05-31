use crate::Model;
use nannou::prelude::*;

pub(crate) fn update(app: &App, model: &mut Model) {
    let count = app.elapsed_frames();
    match count {
        0..=100 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 0, 100, 0.0, -0.101),
            map_range(count, 0, 100, 0.0, 0.956),
            map_range(count, 0, 100, 1.0, 0.1),
        ),
        501..=600 => model.mandelbrot_set.zoom_to_point(
            map_range(count, 501, 600, 0.0, 0.30339),
            map_range(count, 501, 600, 0.0, 0.513058),
            map_range(count, 501, 600, 1.0, 0.1),
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
        101..=200 => {
            draw_mandelbrot(app, model, frame, &draw);
            draw_branch3_1(app, model, &frame, &draw);
        }
        201..=300 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_branch3_2(app, model, &frame, &draw);
        }
        301..=400 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_branch3_3(app, model, &frame, &draw);
        }
        401..=500 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_branch3_4(app, model, &frame, &draw);
        }
        501..=600 => {
            draw_mandelbrot(app, model, frame, &draw);
        }
        601..=700 => {
            draw_mandelbrot(app, model, frame, &draw);
            draw_branch4_1(app, model, &frame, &draw);
        }
        701..=800 => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_branch4_2(app, model, &frame, &draw);
        }
        _ => {
            draw_mandelbrot(app, model, &frame, &draw);
            draw_branch4_3(app, model, &frame, &draw);
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
        .x_y(-9.0, -65.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(59.0, 34.0);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(-50.0, 50.0);
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(7.0)
        .color(RED);
}

fn draw_branch3_2(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(-357.0, 559.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(-305.0, 662.0);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(-406.0, 661.0);
    draw.ellipse()
        .x_y(-356.0, 622.0)
        .radius(7.0)
        .color(RED);
}

fn draw_branch3_3(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(169.0, 88.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(260.0, 144.0);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(180.0, 196.0);
    draw.ellipse()
        .x_y(207.0, 134.0)
        .radius(7.0)
        .color(RED);
}

fn draw_branch3_4(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(-192.0, -330.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(-239.0, -235.0);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(-294.0, -299.0);
    draw.ellipse()
        .x_y(-237.0, -293.0)
        .radius(7.0)
        .color(RED);
}

fn draw_branch3_5(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(224.0, -617.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(306.0, -573.0);
    draw.text("3")
        .color(RED)
        .font_size(50)
        .x_y(237.0, -499.0);
    draw.ellipse()
        .x_y(238.0, -580.0)
        .radius(7.0)
        .color(RED);
}

fn draw_branch4_1(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.ellipse()
        .x_y(304.0, 371.0)
        .radius(7.0)
        .color(RED);
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(234.0, 328.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(356.0, 322.0);
    draw.text("3").color(RED).font_size(50).x_y(376.0, 416.0);
    draw.text("4")
        .color(RED)
        .font_size(50)
        .x_y(280.0, 467.0);
}

fn draw_branch4_2(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.ellipse()
        .x_y(322.0, 757.0)
        .radius(7.0)
        .color(RED);
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(284.0, 709.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(374.0, 724.0);
    draw.text("3").color(RED).font_size(50).x_y(375.0, 806.0);
    draw.text("4")
        .color(RED)
        .font_size(50)
        .x_y(275.0, 821.0);
}

fn draw_branch4_3(
    app: &App,
    model: &Model,
    frame: &Frame,
    draw: &Draw,
) {
    draw.ellipse()
        .x_y(-112.0, 438.0)
        .radius(7.0)
        .color(RED);
    draw.text("1")
        .color(RED)
        .font_size(50)
        .x_y(-151.0, 391.0);
    draw.text("2")
        .color(RED)
        .font_size(50)
        .x_y(-40.0, 450.0);
    draw.text("3").color(RED).font_size(50).x_y(-111.0, 510.0);
    draw.text("4")
        .color(RED)
        .font_size(50)
        .x_y(-191.0, 466.0);
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
