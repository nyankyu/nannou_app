use super::target_area::TargetArea;
use nannou::prelude::{dvec2, DVec2, PI_F64, TAU_F64, };

const ZOOM_RATIO: f64 = 9e-2;
//const ZOOM_RATIO: f64 = 0.9;
const FINAL_TARGET: (f64, f64) =
    //(-0.15773294549873199, 1.0253132341831026);
    //(-1.26222162762384535, -0.04591700163513884);
    (-0.8606130991081128, -0.2351181260185022);
//(-0.7694116949467069, -0.11523600047980796);

pub(super) struct AutoZoom {
    auto: bool,
    auto_type: AutoType,
    index: usize, // NOT GOOD : for AutoType::FromPlaceToPlace.target_list
}

impl AutoZoom {
    pub(super) fn new(zoom_in: bool) -> Self {
        let auto_type = if zoom_in {
            AutoType::ZoomIn {
                final_target: DVec2::from(FINAL_TARGET),
            }
        } else {
            let target_list = (2..=200).map(|q| {
                let q = q as f64;
                let theta = TAU_F64 / q;
                let x = 0.5 * theta.cos() - 0.25 * (2.0 * theta).cos();
                let y = 0.5 * theta.sin() - 0.25 * (2.0 * theta).sin();
                let r = 6.0 / q / q * (PI_F64 / q).sin();
                (dvec2(x, y), r)
            }).collect();
            AutoType::FromPlaceToPlace { target_list }
        };

        Self {
            auto: false,
            auto_type: auto_type,
            index: 0,
        }
    }
    pub(super) fn auto(&mut self) {
        self.auto ^= true;
    }

    pub(super) fn is_auto(&self) -> bool {
        self.auto
    }

    pub(super) fn next(&mut self, target: &mut TargetArea) {
        match &self.auto_type {
            AutoType::ZoomIn { final_target } => {
                let to = (*final_target - target.center)
                    * 0.12
                    + target.center;
                target.move_to(to);
                target.zoom(ZOOM_RATIO);
            },
            AutoType::FromPlaceToPlace { target_list } => {
                let (center, magnification) =
                    target_list[self.index];
                self.index += 1;
                if self.index == target_list.len() {
                    self.auto = false;
                    self.index = 0;
                }
                target.change(center, magnification);
            },
        }
    }
}

pub(super) enum AutoType {
    ZoomIn { final_target: DVec2 },
    FromPlaceToPlace { target_list: Vec<(DVec2, f64)> },
}
