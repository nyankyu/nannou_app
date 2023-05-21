use super::target_area::TargetArea;
use nannou::prelude::DVec2;

const ZOOM_RATIO: f64 = 0.99;
const FINAL_TARGET: (f64, f64) =
    (-0.15773294549873199, 1.0253132341831026);
    //(-0.8606130991081128, -0.2351181260185022);
    //(-0.7694116949467069, -0.11523600047980796);

pub(super) struct AutoZoom {
    auto: bool,
    final_target: DVec2,
}

impl AutoZoom {
    pub(super) fn new() -> Self {
        Self {
            auto: false,
            final_target: DVec2::from(FINAL_TARGET),
        }
    }
    pub(super) fn auto(&mut self) {
        self.auto ^= true;
    }

    pub(super) fn is_auto(&self) -> bool {
        self.auto
    }

    pub(super) fn next(&mut self, target: &mut TargetArea) {
        let to = (self.final_target - target.center)
            * (1.0 - ZOOM_RATIO)
            + target.center;
        target.move_to(to);
        target.zoom(ZOOM_RATIO);
    }
}
