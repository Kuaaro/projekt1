use svg::node::element::Polygon;

use crate::geometry::point::{Point, PointBasics};
use crate::floor::disp::Disp;
use crate::{constants, nat_dis_num_gen};

use super::center::CenterBasics;

pub struct Stairs {
    center: Point,
    weight: f32
}

impl Disp for Stairs {
    fn disp(&self, mut doc: svg::node::element::SVG) -> svg::node::element::SVG {
        let staircase_size = constants::CONSTANTS.lock().unwrap().staircase_size >> 1;
        doc = doc.add(Polygon::new()
            .set("points", vec![(self.center.get_x()-staircase_size, self.center.get_y()-staircase_size), (self.center.get_x()+staircase_size, self.center.get_y()-staircase_size),(self.center.get_x()+staircase_size, self.center.get_y()+staircase_size),(self.center.get_x()-staircase_size, self.center.get_y()+staircase_size)])
            .set("fill", "gray"));
        return doc;
    }
}

impl PointBasics for Stairs {
    fn get_x(&self) -> i32 {
        return self.center.get_x();
    }

    fn get_y(&self) -> i32 {
        return self.center.get_y();
    }
}

impl CenterBasics for Stairs {
    fn divide_floor(&self, floor: &mut super::floor::Floor) {
        let staircase_sspace = constants::CONSTANTS.lock().unwrap().staircase_sspace;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height;
        let max_width = constants::CONSTANTS.lock().unwrap().max_width;

        floor.divide_floor(&Point::new(self.center.get_x()+staircase_sspace, 0), &Point::new(self.center.get_x()+staircase_sspace, max_height));
        floor.divide_floor(&Point::new(self.center.get_x()-staircase_sspace, 0), &Point::new(self.center.get_x()-staircase_sspace, max_height));

        floor.divide_floor(&Point::new(0, self.center.get_y()+staircase_sspace), &Point::new(max_width, self.center.get_y()+staircase_sspace));
        floor.divide_floor(&Point::new(0, self.center.get_y()-staircase_sspace), &Point::new(max_width, self.center.get_y()-staircase_sspace));

    }

    fn get_weight(&self) -> f32 {
        return self.weight;
    }

    fn into_point_basics(&self) -> &dyn PointBasics {
        return self;
    }
}

impl Stairs {
    pub fn new(p: Point) -> Self {
        return Stairs{center: p, weight: nat_dis_num_gen::NatDisNumGen::new_const().normal_gen()};
    }
}