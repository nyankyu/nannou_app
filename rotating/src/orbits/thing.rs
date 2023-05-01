use nannou::prelude::*;

pub(super) struct Thing {
    param: f32,
    point: Point3,
    radius: f32,
    color: Hsl,
    hue: f32,
}

impl Thing {
    pub(super) fn new(param: f32, hue: f32) -> Self {
        Self {
            param: param,
            point: Point3::new(0.0, 0.0, 0.0),
            radius: 0.0,
            color: hsl(0.0, 0.0, 0.0),
            hue: hue,
        }
    }

    pub(super) fn update(&mut self, orbital_param: Vec3) {
        let param = self.param * orbital_param;
        self.point = pt3(
            param.x.sin(),
            param.y.sin(),
            param.z.sin(),
        )
        .normalize()
            * 450.0;

        self.radius = (self.point[2] + 500.0) / 100.0;

        self.hue += 0.0005;
        if self.hue > 1.0 {
            self.hue = 0.0;
        }

        self.color = if self.point[2] > 0.0 {
            hsl(
                self.hue,
                (self.point[2] + 550.0) / 800.0,
                0.5,
            )
        } else {
            hsl(self.hue, 0.0, 0.05)
        };

        self.param += 0.02;
    }

    pub(super) fn display(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius)
            .xyz(self.point)
            .color(self.color);
    }
}
