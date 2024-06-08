use svg::Document;

use crate::{geometry::point::{Point, PointBasics}, nat_dis_num_gen};
use crate::floor::{floor::Floor, disp::Disp};

pub struct Center {
    point: Point,
    weight: f32
}

pub trait CenterBasics: PointBasics + Disp {
    fn get_weight(&self) -> f32;
    fn into_point_basics(&self) -> &dyn PointBasics;
    fn divide_floor(&self, floor: &mut Floor);
}

impl Disp for Center {
    fn disp(&self, doc: svg::node::element::SVG) -> svg::node::element::SVG {
        return doc;
    }
}

impl PointBasics for Center {
    fn get_x(&self) -> i32 {
        return self.point.get_x();
    }

    fn get_y(&self) -> i32 {
        return self.point.get_y();
    }
}

impl CenterBasics for Center {
    fn get_weight(&self) -> f32 {
        return self.weight;
    }

    fn into_point_basics(&self) -> &dyn PointBasics {
        return self;
    }

    fn divide_floor(&self, floor: &mut Floor) {
        return;
    }
}

impl Center {
    pub fn new(x: i32, y: i32) -> Self {
        return Center{point: Point::new(x, y), weight: nat_dis_num_gen::NatDisNumGen::new_const().normal_gen()};
    }
}