pub mod perlin_noise;

use nannou::prelude::*;
use perlin_noise::{Dot, Rgb};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

pub struct Model {
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
        draw.background().color(self.bg_color);
        self.dot.display(draw);
    }

    fn update(&mut self) {
        self.dot.update();
    }
}

fn model(_app: &App) -> Model {
    Model::default()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.display(&draw);

    draw.background().color(model.bg_color);

    draw.to_frame(app, &frame).unwrap();
}
