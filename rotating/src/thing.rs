use nannou::prelude::*;

pub struct Thing {
    point: Point3,
    color: f32,
}

impl Thing {
    pub fn new() -> Self {
        Self {
            point: Point3::new(
                random_range(-200.0, 200.0),
                random_range(-200.0, 200.0),
                random_range(-200.0, 200.0),
            ),
            color: 0.5,
        }
    }

    pub fn update(&mut self) {
    }

    pub fn display(&self, draw: &Draw, time: f32) {
        let rotation = vec3(0.1, 0.2, 0.3) * time;
        draw.radians(rotation)
            .ellipse()
            .radius(5.0)
            .xyz(self.point)
            .hsl(self.color, (self.point[2] + 200.0) / 400.0, 0.5);
    }
}
