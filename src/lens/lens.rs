use crate::drawable::Point;
use sciimg::vector::Vector;

pub trait Lens {
    fn vector_to_point(&self, v:&Vector) -> Point;
}


