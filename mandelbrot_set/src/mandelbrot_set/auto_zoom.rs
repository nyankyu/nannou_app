use super::target_area::TargetArea;
use nannou::prelude::{dvec2, DVec2};

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
            let target_list = vec![
                (dvec2(-1.0, 0.0), 0.7),
                (dvec2(-0.124866818, 0.74396884), 0.5),
                (dvec2(0.281058181, 0.531069896), 0.4),
                (dvec2(0.37926948, 0.33593786), 0.3),
                (dvec2(-1.1900443, 0.3043895), 0.000_01),
                (dvec2(-0.7436441, 0.1318255), 0.000_01),
            ];
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
                target.change(center, magnification, 0.0);
            },
        }
    }
}

pub(super) enum AutoType {
    ZoomIn { final_target: DVec2 },
    FromPlaceToPlace { target_list: Vec<(DVec2, f64)> },
}
