use crate::lens::lens::Lens;
use sciimg::drawable::Point;
use sciimg::min;
use sciimg::vector::Vector;

pub struct FisheyeEquisolidLens {
    image_width: usize,
    image_height: usize,

    #[allow(dead_code)]
    focal_length: f64,
    field_of_view: f64,
}

impl FisheyeEquisolidLens {
    pub fn new(
        image_width: usize,
        image_height: usize,
        focal_length: f64,
        field_of_view: f64,
    ) -> FisheyeEquisolidLens {
        FisheyeEquisolidLens {
            image_width,
            image_height,
            focal_length,
            field_of_view,
        }
    }
}

impl Lens for FisheyeEquisolidLens {
    fn vector_to_point(&self, v: &Vector) -> Point {
        let r = (v.y * v.y + v.z * v.z).sqrt().atan2(v.x) / self.field_of_view.to_radians();
        let phi = v.z.atan2(v.y);

        let u = r * phi.cos() + 0.5;
        let v = r * phi.sin() + 0.5;

        let field_width = min!(self.image_width, self.image_height);
        let x_adjust = if self.image_width > self.image_height {
            (self.image_width - self.image_height) / 2
        } else {
            0
        };

        let y_adjust = if self.image_height > self.image_width {
            (self.image_height - self.image_width) / 2
        } else {
            0
        };

        Point::create_rgb(
            u as f64 * (field_width as f64) + x_adjust as f64,
            (1.0 - v as f64) * (field_width as f64) + y_adjust as f64,
            0.0,
            0.0,
            0.0,
        )
    }
}
