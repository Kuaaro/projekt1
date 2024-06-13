use crate::{constants, geometry::point::{Point, PointBasics}};

#[derive(Clone)]
#[derive(Debug)]
pub struct Cycle {
    points: Vec<Point>,
}

impl Cycle {
    pub fn get_mass_center(&self) -> Point {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for p in self.points.iter() {
            x += p.get_x();
            y += p.get_y();
        }

        x /= self.points.len() as i32;
        y /= self.points.len() as i32;

        return Point::new(x, y);
    }

    pub fn new(points: Vec<Point>) -> Self {
        return Cycle{points};
    }

    pub fn get_points(&self) -> &Vec<Point> {
        return &self.points;
    }

    pub fn divide_cycle(&self, p1: &dyn PointBasics, p2: &dyn PointBasics) -> Vec<Cycle> {
        let out = self.divide_cycle_without_limits(p1, p2);

        let min_room_wall_length = constants::CONSTANTS.lock().unwrap().min_room_wall_length;
        let min_room_area = constants::CONSTANTS.lock().unwrap().min_room_area;

        if out.len() == 1 {
            return out;
        }

        if (out[0].points[0].get_x()-out[0].points[2].get_x()).abs() < min_room_wall_length || (out[0].points[0].get_y()-out[0].points[2].get_y()).abs() < min_room_wall_length {
            return vec![self.clone()];
        }

        if (out[1].points[0].get_x()-out[1].points[2].get_x()).abs() < min_room_wall_length || (out[1].points[0].get_y()-out[1].points[2].get_y()).abs() < min_room_wall_length {
            return vec![self.clone()];
        }

        if out[0].get_area() < min_room_area || out[1].get_area() < min_room_area {
            return vec![self.clone()];
        }

        

        return out;
    }

    pub fn divide_cycle_without_limits(&self, p1: &dyn PointBasics, p2: &dyn PointBasics) -> Vec<Cycle> { 
        let mut last_point: &Point = self.points.last().unwrap();
    
        let mut last_side: i32 = Point::which_side(last_point, p1, p2);
        let mut current_side: i32;
    
        let mut c1v: Vec<Point> = Vec::new();
        let mut c2v: Vec<Point> = Vec::new();
    
        for i in self.points.iter() {
            current_side = Point::which_side(i, p1, p2);
    
            if different_sign(last_side, current_side) {
                let p = Point::point_of_cross(p1, p2, i, last_point);
                c1v.push(p.clone());
                c2v.push(p.clone());
            }

            if current_side <= 0 {
                c1v.push(i.clone());
            }
            
            if current_side >= 0 {
                c2v.push(i.clone());
            }
    
            last_side = current_side;
            last_point = i;
        }

        let c1 = Cycle::new(c1v);
        let c2 = Cycle::new(c2v);

        if c1.points.len() < 4 || c2.points.len() < 4 {
            return vec![self.clone()];
        }

        return vec![c1, c2];
    }

    fn get_area(&self) -> i32 {
        let mut last_point: &Point;
        if let Some(tmp) = self.points.last() {
            last_point = tmp;
        } else {
            return 0;
        }
        let mut area = 0;

        for p in self.points.iter() {
            area += last_point.get_x() * p.get_y() - last_point.get_y() * p.get_x();
            last_point = p;
        }
        return area.abs() >> 1;
    }

    pub fn connect_cycles(vec_in: &Vec<&Cycle>) -> Vec<Cycle> {
        let arr: Vec<[Point; 2]> = vec_in.iter().map(|c| c.into_segments()).flatten().collect();
        let mut vec: Vec<[Point; 2]> = arr.clone().into_iter().filter(|[p1, p2]| !arr.contains(&[p2.clone(), p1.clone()])).collect();
        
        let mut out: Vec<Cycle> = Vec::new();
        while vec.len() > 0 {
            Cycle::segment_to_front(&mut vec);
            let vector_cycle = Cycle::segments_to_polygon(&mut vec);
            out.push(Cycle::new(vector_cycle.into_iter().map(|[p1, _]| p1).collect()));
        }
        
        return out;
    }

    pub fn remove_maruders(&mut self) {
        let mut i = 0;
        let mut len = self.points.len()-2;

        while i < len {
            if Point::in_line(&self.points[i], &self.points[i+1], &self.points[i+2]) {
                len -= 1;
                self.points.remove(i+1);
            } else {
                i += 1;
            }
        }
        
        if Point::in_line(&self.points[len], &self.points[len+1], &self.points[0]) {
            self.points.pop();
            
        }
    }

    fn segment_to_front(vec: &mut Vec<[Point; 2]>) {
        let mut min_x: i32 = i32::MAX;
        let mut min_y: i32 = i32::MAX;
        let mut index = 0;
        for [p1, _] in vec.iter() {
            min_x = min_x.min(p1.get_x());
        }
        for (i, [p1, _]) in vec.iter().enumerate() {
            if p1.get_x()==min_x && min_y > p1.get_y() {
                min_y = p1.get_y();
                index = i;
            }
        }
    
        vec.swap(0, index);
    }
    
    fn segments_to_polygon(vec: &mut Vec<[Point; 2]>) -> Vec<[Point; 2]> {
        for index0 in 0..vec.len()-2 {
            let [_, p0] = &vec[index0];
            for index_s in (index0+1)..vec.len() {
                let [p_s, _] = vec.get(index_s).unwrap();
                if p_s==p0 {
                    vec.swap(index_s, index0+1);
                    break;
                }
            }
            
            if vec.get(index0+1).unwrap()[1] == vec.first().unwrap()[0] {
                return vec.drain(0..index0+2).collect();
            }
        }
    
        return vec.drain(..).collect();
    }

    pub fn into_segments(&self) -> Vec<[Point; 2]> {
        let mut out: Vec<[Point; 2]> = Vec::with_capacity(self.points.len());

        for i in 1..self.points.len() {
            out.push([self.points[i-1].clone(), self.points[i].clone()]);
        }

        out.push([self.points.last().unwrap().clone(), self.points[0].clone()]);

        return out;
    }

    pub fn min_x(&self) -> i32 {
        let mut out = i32::MAX;
        for i in self.points.iter() {
            out = out.min(i.get_x());
        }
        return out;
    }

    pub fn min_y(&self) -> i32 {
        let mut out = i32::MAX;
        for i in self.points.iter() {
            out = out.min(i.get_y());
        }
        return out;
    }

    pub fn max_x(&self) -> i32 {
        let mut out = i32::MIN;
        for i in self.points.iter() {
            out = out.max(i.get_x());
        }
        return out;
    }

    pub fn max_y(&self) -> i32 {
        let mut out = i32::MIN;
        for i in self.points.iter() {
            out = out.max(i.get_y());
        }
        return out;
    }
}



fn different_sign(a: i32, b: i32) -> bool {
    return (a > 0 && b < 0) || (a < 0 && b > 0);
}




