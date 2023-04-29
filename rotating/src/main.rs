mod thing;

use nannou::prelude::*;
use thing::*;

const WINDOW_H: u32 = 1024;
const WINDOW_W: u32 = 1024;
const THINGS_LEN: usize = 100;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    things: Vec<Thing>,
    time: f32
}

impl Model {
    fn new() -> Self {
        Self {
            things: (0..THINGS_LEN)
                .map(|_| Thing::new())
                .collect(),
            time: 0.0,
        }
    }

    fn update(&mut self) {
        self.things
            .iter_mut()
            .for_each(|thing| thing.update());

        self.time += 0.01;
    }

    fn display(&self, draw: &Draw) {
        self.things
            .iter()
            .for_each(|thing| thing.display(draw, self.time));
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
