mod particle_system;

use nannou::prelude::*;
use particle_system::ParticleSystem;

const WINDOW_H: u32 = 2048;
const WINDOW_W: u32 = 1024;
const PARTICLE_NUM: usize = 40000;

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
                Rect::from_w_h(
                    WINDOW_W as f32,
                    WINDOW_H as f32,
                ),
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
    app.new_window()
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

    //save_frame(app);
}

#[allow(dead_code)]
fn save_frame(app: &App) {
    let frame_num = app.elapsed_frames();

    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join(frame_num.to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}
