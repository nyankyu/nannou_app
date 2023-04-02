use nannou::{prelude::*, color::named};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x: f32,
    y: f32,
    r: f32,
}

fn model(_app: &App) -> Model {
    Model {
        x: 0.0,
        y: 0.0,
        r: 10.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.r > 500.0 {
        model.r = 10.0;
    } else {
        model.r += 1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background()
        .color(named::HONEYDEW);

    draw.ellipse()
        .color(STEELBLUE)
        .w(model.r)
        .h(model.r)
        .x_y(model.x, model.y);

    draw.to_frame(app, &frame).unwrap();
}