use crate::drawable::Point;
use crate::lens::lens::Lens;
use sciimg::vector::Vector;

pub struct FisheyeEquisolidLens {
    image_width: usize,
    image_height: usize,
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

        Point {
            x: u * (self.image_width as f64),
            y: (1.0 - v) * (self.image_height as f64),
            v: 0.0,
        }
    }
}
