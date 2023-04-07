pub mod perlin_noise;

use nannou::prelude::*;
use perlin_noise::{Dot, Rgb};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    bg_color: Rgb,
    dot: Dot,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: GOLD,
            dot: Dot::new(),
        }
    }
}

impl Model {
    fn display(&self, draw: &Draw) {
        self.dot.display(draw);
    }

    fn update(&mut self) {
        self.dot.update();
    }
}

const WINDOW_W: u32 = 512;
const WINDOW_H: u32 = 512;

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_W, WINDOW_H)
        .view(view)
        .build()
        .unwrap();

    Model::default()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.display(&draw);

    if app.elapsed_frames() == 1 {
        draw.background().color(model.bg_color);
    }

    draw.to_frame(app, &frame).unwrap();
}
