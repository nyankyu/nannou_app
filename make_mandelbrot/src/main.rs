mod particle;

use std::process::exit;

use nannou::prelude::*;
use particle::*;
use rayon::prelude::*;

//const WINDOW_H: u32 = 720;
pub(crate) const WINDOW_H: u32 = 1080;
pub(crate) const WINDOW_W: u32 = 1920;
//pub(crate) const WINDOW_H: u32 = 500;
//pub(crate) const WINDOW_W: u32 = 500;

pub(crate) const ITERATION: u64 = 30;
const MAX_ITERATION: u64 = ITERATION * 100;
const STOP_ITERATION: u64 = MAX_ITERATION + 50;
const REV_ITERATION: u64 = STOP_ITERATION + MAX_ITERATION;
const LAST_ITERATION: u64 = REV_ITERATION + 50;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

struct Model {
    pixel: Vec<Vec<Particle>>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_W, WINDOW_H)
        .view(view)
        .build()
        .unwrap();

    let mut pixel = Vec::<Vec<Particle>>::new();
    let win_rect = app.window_rect();
    let bottom = win_rect.bottom() as i32;
    let top = win_rect.top() as i32;
    let left = win_rect.left() as i32;
    let right = win_rect.right() as i32;
    for y in bottom..=top {
        let mut row = Vec::<Particle>::new();
        if y % 3 != 0 {
            //continue;
        }
        for x in left..=right {
            if x % 3 != 0 {
                //continue;
            }
            let p = Particle::new(x as f32, y as f32);
            row.push(p);
        }
        pixel.push(row);
    }

    Model { pixel: pixel }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let f = app.elapsed_frames();
    if f >= MAX_ITERATION {
        return;
    }

    model.pixel.par_iter_mut().for_each(|row| {
        row.iter_mut().for_each(|p| p.update(f % 100))
    })
}

fn event(app: &App, model: &mut Model, event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let t = app.elapsed_frames();
    if t == 0 {
        for row in &model.pixel {
            for p in row {
                draw.ellipse()
                    .radius(0.5)
                    .xy(p.get_c())
                    .color(srgba(1.0, 1.0, 1.0, 0.6));
            }
        }
    } else if t < MAX_ITERATION {
        for row in &model.pixel {
            for p in row {
                if !p.on_screen {
                    continue;
                }
                let (pre, last) = p.get_middle_pow2();
                draw.line()
                    .weight(0.5)
                    .start(pre)
                    .end(last)
                    .color(srgba(1.0, 1.0, 1.0, 0.6));
            }
        }

        draw.rect()
            .wh(app.window_rect().wh())
            .color(rgba(0.0, 0.0, 0.0, 0.3));

        draw.text(&format!(
            "{}",
            app.elapsed_frames() / 100 + 1
        ))
        .xy(Vec2::ZERO)
        .font_size(70)
        .color(RED);
    } else if t < STOP_ITERATION {
        draw.rect()
            .wh(app.window_rect().wh())
            .color(rgba(0.0, 0.0, 0.0, 0.2));

        for row in &model.pixel {
            for p in row {
                if !p.on_screen {
                    continue;
                }
                draw.ellipse()
                    .radius(0.5)
                    .xy(p.get_last())
                    .color(WHITE);
            }
        }
    } else if t <= STOP_ITERATION + 5 {
        frame.clear(BLACK);
    } else if t < REV_ITERATION {
        let t = REV_ITERATION - t + 1;
        for row in &model.pixel {
            for p in row {
                if p.on_screen {
                    continue;
                }
                match (p.get_nth(t - 1), p.get_nth(t)) {
                    (_, None) | (None, _) => continue,
                    (Some(pre), Some(last)) => {
                        draw.line()
                            .weight(0.5)
                            .start(pre)
                            .end(last)
                            .color(p.color);
                    }
                }
            }
        }

        draw.rect()
            .wh(app.window_rect().wh())
            .color(rgba(0.0, 0.0, 0.0, 0.1));

        let h = (t / 100) as f32 / ITERATION as f32;
        draw.text(&format!("{}", t / 100 + 1))
            .xy(Vec2::ZERO)
            .font_size(70)
            .color(hsl(h, 1.0, 0.5));
    } else if t < LAST_ITERATION {
        draw.rect()
            .wh(app.window_rect().wh())
            .color(rgba(0.0, 0.0, 0.0, 0.2));

        for row in &model.pixel {
            for p in row {
                if p.on_screen {
                    continue;
                }
                draw.ellipse()
                    .radius(1.0)
                    .xy(p.get_c())
                    .color(p.color);
            }
        }
    } else {
        exit(1);
    }

    draw.to_frame(app, &frame).unwrap();

    //save_frame(app, 0);
}

#[allow(dead_code)]
fn save_frame(app: &App, file_num: u32) {
    let path = app
        .project_path()
        .expect("could not locate project_path")
        .join("snapshots")
        .join("mandelbrot_set")
        .join(app.elapsed_frames().to_string())
        .with_extension("png");

    app.main_window().capture_frame(path);
}
