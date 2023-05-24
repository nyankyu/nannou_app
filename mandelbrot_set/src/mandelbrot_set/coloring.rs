#![allow(dead_code)]

use nannou::image::Rgba;

use super::ITERATION_LIMIT;

pub(super) struct Coloring {
    method: ColoringMethod,
    palette: Vec<Rgba<u8>>,
}

pub(super) enum ColoringMethod {
    GrayCyclic,
    GrayBackAndFroth,
    Heatmap,
}

impl Coloring {
    pub(super) fn new(
        coloring_method: ColoringMethod,
    ) -> Self {
        match coloring_method {
            ColoringMethod::GrayCyclic => {
                let mut palette = Vec::with_capacity(256);
                for c in 0..=255 {
                    palette.push(Rgba([c, c, c, 255]));
                }
                Self {
                    method: coloring_method,
                    palette: palette,
                }
            },
            ColoringMethod::GrayBackAndFroth => {
                let mut palette = Vec::with_capacity(511);
                for c in 0..511 {
                    let c: u8 = if c < 256 {
                        c as u8
                    } else {
                        (511 - c) as u8
                    };
                    palette.push(Rgba([c, c, c, 255]));
                }
                Self {
                    method: coloring_method,
                    palette: palette,
                }
            },
            ColoringMethod::Heatmap => {
                let mut palette = Vec::with_capacity(256);
                for c in 0..=255 {
                    palette.push(Rgba([c, c, c, 255]));
                }
                Self {
                    method: coloring_method,
                    palette: palette,
                }
            }
        }
    }

    pub(super) fn get(&self, num: u32) -> Rgba<u8> {
        match self.method {
            ColoringMethod::GrayCyclic => {
                if num == ITERATION_LIMIT {
                    self.palette[0]
                } else {
                    self.palette[num as usize % 256]
                }
            },
            ColoringMethod::GrayBackAndFroth => {
                if num == ITERATION_LIMIT {
                    self.palette[0]
                } else {
                    self.palette[num as usize % 511]
                }
            },
            ColoringMethod::Heatmap => {
                self.palette[num as usize % 256]
            }
        }
    }
}

pub(super) fn gray_cyclic(count: u32) -> Rgba<u8> {
    let gray = (count % 256) as u8;
    Rgba([gray, gray, gray, 255])
}

pub(super) fn gray_back_and_froth(count: u32) -> Rgba<u8> {
    let gray = count % 510;
    let gray = if gray <= 255 {
        gray as u8
    } else {
        (510 - gray) as u8
    };
    Rgba([gray, gray, gray, 255])
}

pub(super) fn gray_strip(count: u32) -> Rgba<u8> {
    let gray = count % 256;
    let gray =
        if gray % 10 == 0 { 255 } else { gray as u8 };
    Rgba([gray, gray, gray, 255])
}

pub(super) fn two_color(count: u32) -> Rgba<u8> {
    if count % 2 == 0 {
        Rgba([50, 50, 50, 255])
    } else {
        Rgba([150, 150, 150, 255])
    }
}

pub(super) fn four_color(count: u32) -> Rgba<u8> {
    let gray = (count % 4) as u8 * 80;
    Rgba([gray, gray, gray, 255])
}

pub(super) fn gray_cyclic_phase(
    count: u32,
    phase: u32,
) -> Rgba<u8> {
    let gray = ((count + phase) % 256) as u8;
    Rgba([gray, gray, gray, 255])
}

pub(super) fn gray_back_and_froth_phase(
    count: u32,
    phase: u32,
) -> Rgba<u8> {
    let gray = (count + phase) % 510;
    let gray = if gray <= 255 {
        gray as u8
    } else {
        (510 - gray) as u8
    };
    Rgba([gray, gray, gray, 255])
}

pub(super) fn make_color_palette(
    size: u32,
) -> Vec<Rgba<u8>> {
    let mut palette = Vec::<Rgba<u8>>::with_capacity(200);

    for i in 0..size {
        let h = i % 200;
        let hue = if h < 100 { 0.65 } else { 0.0 };
        let l = if h < 100 {
            h as f32 / 100.0
        } else {
            1.0 - ((h - 100) as f32 / 100.0)
        };

        palette.push(hsl_to_rgba(hue, 1.0, l));
    }

    palette
}

pub(super) fn make_cumulative_heatmap(
    iteration_counts: &Vec<u32>,
    limit: u32,
) -> Vec<u32> {
    let mut heatmap = vec![0; limit as usize];

    for &count in iteration_counts {
        if count == limit {
            continue;
        }
        heatmap[count as usize] += 1;
    }

    for i in 1..limit as usize {
        heatmap[i] += heatmap[i - 1];
    }

    heatmap
}

pub(super) fn gray_heatmap(
    heatmap: &Vec<u32>,
    count: u32,
) -> Rgba<u8> {
    let total = heatmap[heatmap.len() - 2];
    let r = (heatmap[count as usize] as f32 / total as f32
        * 255.0) as u8;

    Rgba([r, r, r, 255])
}

fn hsl_to_rgba(h: f32, s: f32, l: f32) -> Rgba<u8> {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let (r, g, b) = (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    );

    Rgba([r, g, b, 255])
}
