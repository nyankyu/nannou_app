mod particle_system;

use nannou::prelude::*;
use particle_system::ParticleSystem;

const WINDOW_H: u32 = 2048;
const WINDOW_W: u32 = 1024;
const PARTICLE_NUM: usize = 6000;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  system: ParticleSystem,
}

impl Model {
  fn new() -> Self {
    Self {
      system: ParticleSystem::new(
        WINDOW_W,
        WINDOW_H,
        PARTICLE_NUM,
      ),
    }
  }

  fn update(&mut self) {
    self.system.update();
  }

  fn display(&self, draw: &Draw) {
    self.system.display(draw);
  }
}

fn model(app: &App) -> Model {
  app
    .new_window()
    .size(WINDOW_W, WINDOW_H)
    .view(view)
    .build()
    .unwrap();

  Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
  model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(BLACK);
  model.display(&draw);

  draw.to_frame(app, &frame).unwrap();
}
