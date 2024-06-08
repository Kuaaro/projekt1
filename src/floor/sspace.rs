use svg::node::element::Polygon;

use crate::geometry::cycle::Cycle;
use crate::geometry::point::{Point, PointBasics};

use super::disp::Disp;

#[derive(Debug)]
pub struct SSpace {
    shape: Cycle,
    id: u16,
    center: Point
}

impl SSpace {
    pub fn new(shape: Cycle) -> Self {
        return SSpace{center: shape.get_mass_center(), shape, id: 0};
    }

    pub fn new_with_id(shape: Cycle, id: u16) -> Self {
        return SSpace{center: shape.get_mass_center(), shape, id};
    }

    pub fn assign_id(&mut self, id: u16) {
        self.id = id;
    }

    pub fn get_id(&self) -> u16 {
        return self.id;
    }

    pub fn get_center(&self) -> &Point {
        return &self.center;
    }

    pub fn divide_sspace(&self, p1: &dyn PointBasics, p2: &dyn PointBasics) -> Vec<SSpace>{
        return self.shape.divide_cycle(p1, p2).into_iter().filter(|c: &Cycle| c.get_points().len() > 3).map(|v| SSpace::new(v)).collect();
        }

    pub fn get_points(&self) -> &Vec<Point> {
        return self.shape.get_points();
    }

    pub fn get_cycle(&self) -> &Cycle {
        return &self.shape;
    }
}

impl Disp for SSpace {
    fn disp(&self, mut doc: svg::node::element::SVG) -> svg::node::element::SVG {
        doc = doc.add(Polygon::new()
            .set("points", self.get_cycle()
                .get_points().iter()
                .map(|p| (p.get_x(), p.get_y()))
                .collect::<Vec<(i32, i32)>>())
                .set("fill", "white")
                .set("stroke", "black")
                .set("stroke-width", 3.));
        
        return doc;
    }
}
