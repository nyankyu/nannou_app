use crate::Model;
use nannou::prelude::*;

enum Chapter {
    Start,
    Branch3_5,
    Zoom8,
    Branch8,
    Zoom13,
    Branch13,
    Zoom21,
    Branch21,
    Zoom34,
    Branch34,

    End,
}

fn get_chapter(count: u64) -> Chapter {
    if count < 100 {
        Chapter::Start
    } else if count < 200 {
        Chapter::Branch3_5
    } else if count < 300 {
        Chapter::Branch3_5
    } else if count < 400 {
        Chapter::Zoom8
    } else if count < 500 {
        Chapter::Branch8
    } else if count < 600 {
        Chapter::Zoom13
    } else if count < 700 {
        Chapter::Branch13
    } else if count < 800 {
        Chapter::Zoom21
    } else if count < 900 {
        Chapter::Branch21
    } else if count < 1000 {
        Chapter::Zoom34
    } else if count < 1100 {
        Chapter::Branch34
    } else {
        Chapter::End
    }
}

pub(crate) fn update(app: &App, model: &mut Model) {
    let count = app.elapsed_frames();
    let d_count = count % 100;
    match get_chapter(count) {
        Chapter::Start => {
            model.mandelbrot_set.zoom_to_point(
                map_range(
                    d_count,
                    0,
                    99,
                    -0.516,
                    -0.39458501871516444,
                ),
                map_range(
                    d_count,
                    0,
                    99,
                    0.0,
                    0.5891464726626496,
                ),
                map_range(d_count, 0, 99, 1.0, 0.4),
            )
        }
        Chapter::Zoom8 => {
            model.mandelbrot_set.zoom_to_point(
                -0.39458501871516444,
                0.5891464726626496,
                map_range(d_count, 0, 99, 0.4, 0.07),
            )
        }
        Chapter::Zoom13 => {
            model.mandelbrot_set.zoom_to_point(
                -0.39458501871516444,
                0.5891464726626496,
                map_range(d_count, 0, 99, 0.07, 0.032),
            )
        }
        Chapter::Zoom21 => {
            model.mandelbrot_set.zoom_to_point(
                -0.39458501871516444,
                0.5891464726626496,
                map_range(d_count, 0, 99, 0.032, 0.008),
            )
        }
        Chapter::Zoom34 => {
            model.mandelbrot_set.zoom_to_point(
                -0.39458501871516444,
                0.5891464726626496,
                map_range(d_count, 0, 99, 0.008, 0.0017),
            )
        }
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

    draw_mandelbrot(app, model, frame, &draw);

    match get_chapter(app.elapsed_frames()) {
        Chapter::Start => {}
        Chapter::Branch3_5 => {
            branch_3_5(&draw);
            draw_3_5(&draw);
        }
        Chapter::Zoom8 => {
            draw_3_5(&draw);
        }
        Chapter::Branch8 => {
            branch_8(&draw);
            draw_3_5_8_plus(&draw);
        }
        Chapter::Zoom13 => {
            draw_3_5_8(&draw);
        }
        Chapter::Branch13 => {
            branch_13(&draw);
            draw_3_5_8_13plus(&draw);
        }
        Chapter::Zoom21 => {
            draw_3_5_8_13(&draw);
        }
        Chapter::Branch21 => {
            branch_21(&draw);
            draw_3_5_8_13_21plus(&draw);
        }
        Chapter::Zoom34 => {
            draw_3_5_8_13_21(&draw);
        }
        Chapter::Branch34 => {
            branch_34(&draw);
            draw_3_5_8_13_21_34plus(&draw);
        }
        _ => {
            branch_34(&draw);
            draw_3_5_8_13_21_34plus(&draw);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_3_5(draw: &Draw) {
    draw.text("3, 5,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
}

fn draw_3_5_8(draw: &Draw) {
    draw.text("3, 5, 8,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
}

fn draw_3_5_8_13(draw: &Draw) {
    draw.text("3, 5, 8, 13,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
}

fn draw_3_5_8_13_21(draw: &Draw) {
    draw.text("3, 5, 8, 13, 21,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
}

fn draw_3_5_8_plus(draw: &Draw) {
    draw.text("3+5=8")
        .x_y(0.0, -400.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("3, 5,   ,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("8")
        .x_y(115.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(RED);
}

fn draw_3_5_8_13plus(draw: &Draw) {
    draw.text("5+8=13")
        .x_y(0.0, -400.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("3, 5, 8,      ,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("13")
        .x_y(200.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(RED);
}

fn draw_3_5_8_13_21plus(draw: &Draw) {
    draw.text("8+13=21")
        .x_y(0.0, -400.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("3, 5, 8, 13,     ,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("21")
        .x_y(310.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(RED);
}

fn draw_3_5_8_13_21_34plus(draw: &Draw) {
    draw.text("13+21=34")
        .x_y(0.0, -400.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("3, 5, 8, 13, 21,     ,")
        .x_y(0.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(WHITE);
    draw.text("34")
        .x_y(400.0, -600.0)
        .w(1000.0)
        .font_size(130)
        .color(RED);
}

fn branch_3_5(draw: &Draw) {
    draw_branch(
        draw,
        vec![
            vec2(347.17188, 376.375),
            vec2(429.0703, 477.3164),
            vec2(308.39844, 492.5078),
        ],
    );
    draw_branch(
        draw,
        vec![
            vec2(-169.79688, 13.625),
            vec2(-152.1289, 98.72656),
            vec2(-205.08203, 117.53516),
            vec2(-255.82813, 97.32422),
            vec2(-250.39844, 21.414063),
        ],
    );
}

fn branch_8(draw: &Draw) {
    draw_branch(
        draw,
        vec![
            vec2(172.4375, 399.53516),
            vec2(300.45313, 480.60938),
            vec2(244.19922, 593.5),
            vec2(176.32031, 603.85156),
            vec2(120.46484, 633.4258),
            vec2(23.242188, 597.8906),
            vec2(-2.671875, 506.01172),
            vec2(20.101563, 413.26953),
        ],
    );
}

fn branch_13(draw: &Draw) {
    draw_branch(
        draw,
        vec![
            vec2(-279.16016, 114.65625),
            vec2(-169.60156, 200.54688),
            vec2(-211.90625, 277.91797),
            vec2(-240.52344, 332.64844),
            vec2(-304.25, 323.20703),
            vec2(-337.0703, 307.2578),
            vec2(-371.46484, 345.6875),
            vec2(-400.1328, 317.33594),
            vec2(-455.85547, 305.59766),
            vec2(-474.92188, 237.76563),
            vec2(-464.21094, 199.10938),
            vec2(-504.23828, 133.76172),
            vec2(-445.28906, 44.714844),
        ],
    );
}

fn branch_21(draw: &Draw) {
    draw_branch(
        draw,
        vec![
            vec2(381.27344, 436.70703),
            vec2(480.32813, 539.0781),
            vec2(489.95703, 636.25),
            vec2(489.95703, 711.41797),
            vec2(473.3164, 776.64453),
            vec2(425.8086, 822.9844),
            vec2(356.21484, 799.3203),
            vec2(316.90234, 818.41797),
            vec2(301.03906, 735.96875),
            vec2(283.6914, 773.5508),
            vec2(236.71875, 823.3672),
            vec2(238.125, 733.2539),
            vec2(205.50781, 777.4414),
            vec2(127.47656, 796.2539),
            vec2(144.59375, 715.2656),
            vec2(93.10547, 694.8711),
            vec2(139.20313, 648.8047),
            vec2(85.65625, 617.65234),
            vec2(79.9375, 553.90234),
            vec2(102.19141, 473.1211),
            vec2(159.49219, 383.42578),
        ],
    );
}

fn branch_34(draw: &Draw) {
    draw_branch(
        draw,
        vec![
            vec2(172.29688, -176.8711),
            vec2(370.75, 29.148438),
            vec2(390.8125, 197.84766),
            vec2(380.1953, 346.82813),
            vec2(275.2539, 415.375),
            vec2(203.99219, 400.6914),
            vec2(209.86328, 492.8203),
            vec2(130.59766, 475.34766),
            vec2(87.00781, 561.60156),
            vec2(21.769531, 496.375),

            vec2(7.46875, 436.0625),
            vec2(-29.828125, 508.40625),
            vec2(-38.01172, 430.25),
            vec2(-47.085938, 383.0664),
            vec2(-68.32031, 462.02734),
            vec2(-83.0, 389.47656),
            vec2(-120.0625, 530.3633),
            vec2(-138.1289, 443.58594),
            vec2(-140.66406, 385.98438),
            vec2(-175.44531, 459.0039),

            vec2(-210.01953, 418.36328),
            vec2(-283.53125, 476.21094),
            vec2(-289.35938, 383.5586),
            vec2(-276.51172, 339.0078),
            vec2(-345.79297, 336.91797),
            vec2(-308.72656, 278.6797),
            vec2(-283.82813, 245.6875),
            vec2(-357.57813, 243.0),
            vec2(-346.875, 188.4414),
            vec2(-473.40625, 141.97656),

            vec2(-409.7578, 22.839844),
            vec2(-331.08203, -43.007813),
            vec2(-278.60156, -141.10156),
            vec2(-106.59375, -179.84375),
        ],
    );
}

fn draw_branch(draw: &Draw, vec_p: Vec<Vec2>) {
    for i in 1..=vec_p.len() {
        draw.text(&i.to_string())
            .color(RED)
            .font_size(50)
            .xy(vec_p[i - 1]);
    }
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
