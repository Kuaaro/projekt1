use svg::node::element::{Line, SVG};

use crate::geometry::point::{PointBasics, Point};
use crate::floor::{center::CenterBasics, floor::Floor, disp::Disp};
use crate::constants;

pub struct Window {
    x_s: i32, //coords of start and end
    y_s: i32,
    x_e: i32,
    y_e: i32,
    weight: f32,
    //x_max_width: i32,
    //y_max_width: i32,

}

impl PointBasics for Window {
    fn get_x(&self) -> i32 {
        return (self.x_e + self.x_s)/2;
    }

    fn get_y(&self) -> i32 {
        return (self.y_e + self.y_s)/2;
    }
}

impl CenterBasics for Window {
    fn get_weight(&self) -> f32 {
        return self.weight;
    }

    fn into_point_basics(&self) -> &dyn PointBasics {
        return self;
    }

    fn divide_floor(&self, floor: &mut Floor) {
        let window_gap = constants::CONSTANTS.lock().unwrap().window_gap;
        let max_width = constants::CONSTANTS.lock().unwrap().max_width;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height;


        if self.x_s == self.x_e {
            let w_y1 = self.y_s - window_gap;
            let w_y2 = self.y_e + window_gap;
            
            floor.divide_floor(&Point::new(0, w_y1), &Point::new(max_width, w_y1));
            floor.divide_floor(&Point::new(0, w_y2), &Point::new(max_width, w_y2));
        } else {
            let w_x1 = self.x_s - window_gap;
            let w_x2 = self.x_e + window_gap;

            floor.divide_floor(&Point::new(w_x1, 0), &Point::new(w_x1, max_height));
            floor.divide_floor(&Point::new(w_x2, 0), &Point::new(w_x2, max_height));
        }
    }

    
}

impl Disp for Window {
    fn disp(&self, mut doc: SVG) -> SVG {
        let line = Line::new()
            .set("x1", self.x_s)
            .set("y1", self.y_s)
            .set("x2", self.x_e)
            .set("y2", self.y_e)
            .set("stroke-width", 10.0)
            .set("stroke", "cyan");

        doc = doc.add(line);

        return doc;
    }

    
}

impl Window {
    pub fn new(x_s: i32, y_s: i32, x_e: i32, y_e: i32, weight: f32) -> Self{
        return Window{x_s, x_e, y_s, y_e, weight};
    }
}