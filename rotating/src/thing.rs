use nannou::prelude::*;

pub struct Thing {
    init_param: f32,
    point: Point3,
    radius: f32,
    color: Hsl,
}

impl Thing {
    pub fn new() -> Self {
        Self {
            init_param: random_range(0.0, 5.0),
            point: Point3::new(0.0, 0.0, 0.0),
            radius: 0.0,
            color: hsl(0.5, 0.5, 0.5),
        }
    }

    pub fn update(&mut self, param: f32) {
        self.point = pt3(
            (self.init_param + param * 1.111).sin(),
            (self.init_param + param * 1.222).sin(),
            (self.init_param + param * 0.333).sin(),
        )
        .normalize()
            * 450.0;

        self.radius = (self.point[2] + 450.0) / 100.0;

        self.color = hsl(
            self.init_param / 10.0,
            (self.point[2] + 400.0) / 700.0,
            0.5,
        );
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius)
            .xyz(self.point)
            .color(self.color);
    }
}
