#[derive(Clone)]
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

pub trait PointBasics {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x==other.x && self.y==other.y;
    }
}

impl PointBasics for Point {
    fn get_x(&self) -> i32 {
        return self.x;
    }

    fn get_y(&self) -> i32 {
        return self.y;
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Point{x, y};
    }

    pub fn distance(p1: &dyn PointBasics, p2: &dyn PointBasics) -> f32{
        return (Point::square_distance(p1, p2) as f32).sqrt();
    }

    pub fn square_distance(p1: &dyn PointBasics, p2: &dyn PointBasics) -> f32{
        let x_diff = p1.get_x() - p2.get_x();
        let y_diff = p1.get_y() - p2.get_y();

        return (x_diff*x_diff + y_diff*y_diff) as f32;
    }

    pub fn city_distance(p1: &dyn PointBasics, p2: &dyn PointBasics) -> f32{
        return ((p1.get_x()-p2.get_x()).abs()+(p1.get_y()-p2.get_y()).abs()) as f32;
    }

    pub fn point_of_cross(p1: &dyn PointBasics, p2: &dyn PointBasics, q1: &dyn PointBasics, _q2: &dyn PointBasics) -> Point {
        if p1.get_x()==p2.get_x() {
            return Point::new(p1.get_x(), q1.get_y());
        } else {
            return Point::new(q1.get_x(), p1.get_y());
        }
    }

    pub fn which_side(p: &dyn PointBasics, p1: &dyn PointBasics, p2: &dyn PointBasics) -> i32 {
        return (p2.get_x() - p1.get_x()) * (p.get_y() - p1.get_y()) - (p2.get_y() - p1.get_y()) * (p.get_x() - p1.get_x());    
    }

    pub fn in_line(p1: &dyn PointBasics, pn: &dyn PointBasics, p2: &dyn PointBasics) -> bool {
        return (p2.get_x()-p1.get_x())*(pn.get_y()-p1.get_y()) == (pn.get_x()-p1.get_x())*(p2.get_y()-p1.get_y());
    }
}

