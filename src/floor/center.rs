use crate::geometry::point::PointBasics;
use crate::floor::{floor::Floor, disp::Disp};

pub trait CenterBasics: PointBasics + Disp {
    fn get_weight(&self) -> f32;
    fn into_point_basics(&self) -> &dyn PointBasics;
    fn divide_floor(&self, floor: &mut Floor);
}
