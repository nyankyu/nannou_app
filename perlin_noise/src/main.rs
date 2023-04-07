pub mod perlin_noise;

use nannou::prelude::*;
use perlin_noise::{Dot, Rgb};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    bg_color: Rgb,
    dots: [Dot; 20],
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: GOLD,
            dots: [Dot::new(); 20],
        }
    }
}

impl Model {
    fn display(&self, draw: &Draw) {
        for dot in &self.dots {
            dot.display(draw);
        }
    }

    fn update(&mut self) {
        for dot in &mut self.dots {
            dot.update();
        }
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

    if app.elapsed_frames() == 1 {
        draw.background().color(model.bg_color);
    }

    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
