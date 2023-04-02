use crate::lens::lens::Lens;
use sciimg::drawable::Point;
use sciimg::vector::Vector;

pub struct LatLon {
    lat: f64,
    lon: f64,
}

trait VectorToCylindrical {
    fn to_cylindrical(&self) -> LatLon;
    fn to_xy(
        &self,
        top_lat: f64,
        bottom_lat: f64,
        left_lon: f64,
        right_lon: f64,
        image_width: usize,
        image_height: usize,
    ) -> Point;
}

impl VectorToCylindrical for Vector {
    fn to_cylindrical(&self) -> LatLon {
        LatLon {
            lat: self
                .z
                .atan2((self.x * self.x + self.y * self.y).sqrt())
                .to_degrees(),
            lon: self.y.atan2(self.x).to_degrees() + 180.0,
        }
    }

    fn to_xy(
        &self,
        top_lat: f64,
        bottom_lat: f64,
        left_lon: f64,
        right_lon: f64,
        image_width: usize,
        image_height: usize,
    ) -> Point {
        let ll = self.to_cylindrical();

        let lat = ll.lat;
        let lon = ll.lon;

        let mut out_y_f = (lat - bottom_lat) / (top_lat - bottom_lat) * image_height as f64;
        let mut out_x_f = (lon - left_lon) / (right_lon - left_lon) * image_width as f64;

        while out_y_f < 0.0 {
            out_y_f += image_height as f64;
        }

        while out_y_f >= image_height as f64 {
            out_y_f -= image_height as f64;
        }

        while out_x_f < 0.0 {
            out_x_f += image_width as f64;
        }

        while out_x_f >= image_width as f64 {
            out_x_f -= image_width as f64;
        }

        Point {
            x: out_x_f,
            y: out_y_f,
            v: 0.0,
        }
    }
}

pub struct CylindricalLens {
    image_width: usize,
    image_height: usize,
    top_lat: f64,
    bottom_lat: f64,
    left_lon: f64,
    right_lon: f64,
}

impl CylindricalLens {
    pub fn new(
        image_width: usize,
        image_height: usize,
        top_lat: f64,
        bottom_lat: f64,
        left_lon: f64,
        right_lon: f64,
    ) -> CylindricalLens {
        CylindricalLens {
            image_width,
            image_height,
            top_lat,
            bottom_lat,
            left_lon,
            right_lon,
        }
    }
}

impl Lens for CylindricalLens {
    fn vector_to_point(&self, v: &Vector) -> Point {
        v.to_xy(
            self.top_lat,
            self.bottom_lat,
            self.left_lon,
            self.right_lon,
            self.image_width,
            self.image_height,
        )
    }
}
