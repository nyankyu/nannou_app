use crate::Model;
use nannou::prelude::*;

enum Chapter {
    ToStart,
    Zoom,
}

fn get_chapter(count: u64) -> Chapter {
    if count < 100 {
        Chapter::ToStart
    } else {
        Chapter::Zoom
    }
}

/*
re: -1.25467113016734, im: 0.38170649735476875
re: -1.3726760684050612, im: 0.08537116586111133
*/

pub(crate) fn update(app: &App, model: &mut Model) {
    let count = app.elapsed_frames();
    let d_count = count % 100;
    match get_chapter(count) {
        Chapter::ToStart => {
            model.mandelbrot_set.zoom_to_point(
                map_range(
                    d_count,
                    0,
                    99,
                    -0.516,
                    //0.4267700355568207,
                    //-0.5512105138015796,
                    //0.360226392882476,

                    //-1.25467113016734,
                    -1.3726760684050612,
                ),
                map_range(
                    d_count,
                    0,
                    99,
                    0.0,
                    //0.3410956048121021,
                    //0.6276721712363063,
                    //0.3537611782391899,

                    //0.38170649735476875,
                    0.08537116586111133,
                ),
                //map_range(d_count, 0, 99, 1.0, 0.07),
                //map_range(d_count, 0, 99, 0.0, 1.55),
                //map_range(d_count, 0, 99, 1.0, 0.113),
                //map_range(d_count, 0, 99, 0.0, -0.61),

                //map_range(d_count, 0, 99, 1.0, 0.179),
                map_range(d_count, 0, 99, 1.0, 0.04),

                map_range(d_count, 0, 99, 0.0, -0.44),
            )
        }
        Chapter::Zoom => {
            model.mandelbrot_set.zoom_to_click(app.window_rect(), 0.0, 0.0, 0.99)
        }
    }
}

pub(crate) fn view(
    app: &App,
    model: &Model,
    frame: &Frame,
) {
    frame.clear(BLACK);

    let draw = app.draw();

    draw_mandelbrot(app, model, frame, &draw);

    draw.to_frame(app, &frame).unwrap();
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
