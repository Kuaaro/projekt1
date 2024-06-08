use svg::node::element::{Polygon, SVG};
use std::cmp::min;
use rand::Rng;
use rand_distr::num_traits::abs;
use svg::Document;

use crate::constants;
use crate::floor::sspace::SSpace;
use crate::floor::center::{Center, CenterBasics};
use crate::geometry::point::{self, Point, PointBasics};
use crate::geometry::cycle::Cycle;
use crate::nat_dis_num_gen::NatDisNumGen;
use crate::{constants::CONSTANTS, nat_dis_num_gen};

use super::window::Window;
use crate::floor::disp::Disp;
pub struct Floor {
    rooms: Vec<SSpace>,
    centers: Vec<Box<dyn CenterBasics>>
}

impl Disp for Floor {
    fn disp(&self, mut doc: SVG) -> SVG {
        let max_width = constants::CONSTANTS.lock().unwrap().max_width+25;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height+25;

        doc = doc.add(Polygon::new().set("points", vec![(0, max_height), (max_width, max_height), (max_width, 0), (0, 0)]).set("fill", "black"));

        for sspace in self.rooms.iter() {
            doc = sspace.disp(doc);
        }

        for center in self.centers.iter() {
            doc = center.disp(doc);
        }

        return doc;
    }
}



impl Floor {
    pub fn assign_sspaces(&mut self) {
        let mut distance: f32;
        let mut min_distance: f32;

        for sspace in self.rooms.iter_mut() {
            min_distance = f32::MAX;
            
            for (center_id, center) in self.centers.iter().enumerate() {
                distance = center.get_weight() * Point::distance(sspace.get_center(), center.into_point_basics());

                if distance < min_distance {
                    min_distance = distance;
                    sspace.assign_id(center_id as u16);
                }
            } 
        }
    }

    pub fn divide_floor(&mut self, p1: &dyn PointBasics, p2: &dyn PointBasics) {
        let sspaces = std::mem::replace(&mut self.rooms, Vec::new());
        self.rooms.extend(sspaces.into_iter().map(|s| s.divide_sspace(p1, p2)).flatten());
    }

    pub fn random_floor() -> Self {
        let mut out = Floor::l_floor();
        out.add_windows();

        out.divide_by_centers();
        out.assign_sspaces();

        out.combine_sspaces();
        return out;
    }

    fn l_floor() -> Self {
        let mut rnd_gen = nat_dis_num_gen::NatDisNumGen::new_const(); //here

        let max_width = constants::CONSTANTS.lock().unwrap().max_width;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height;

        let mut out: Floor = Floor::empty_square();

        let rdm_x = rnd_gen.range_gen(100, max_width-100);
        let rdm_y = rnd_gen.range_gen(100, max_height-100);

        out.divide_floor(&Point::new(0, rdm_y), &Point::new(max_width, rdm_y));
        out.divide_floor(&Point::new(rdm_x, 0), &Point::new(rdm_x, max_height));
        
        out.rm_room(rand::thread_rng().gen_range(0..3));

        return out;
    }

    fn h_floor() -> Self {
        let mut rnd_gen = nat_dis_num_gen::NatDisNumGen::new_const();

        let max_width = constants::CONSTANTS.lock().unwrap().max_width;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height;

        let mut out: Floor = Floor::empty_square();

        let rdm_x = rnd_gen.range_gen(1, max_width >>1);
        let rdm_y = rnd_gen.range_gen(1, max_height >>1);

        out.divide_floor(&Point::new(0, rdm_y), &Point::new(max_width, rdm_y));
        out.divide_floor(&Point::new(rdm_x, 0), &Point::new(rdm_x, max_height));

        out.divide_floor(&Point::new(0, max_height-rdm_y), &Point::new(max_width, max_height-rdm_y));
        out.divide_floor(&Point::new(max_width-rdm_x, 0), &Point::new(max_width-rdm_x, max_height));

        out.rm_room(6);
        out.rm_room(1);

        return out;
    }

    fn plus_floor() -> Self {
        let mut rnd_gen = nat_dis_num_gen::NatDisNumGen::new_const();

        let max_width = constants::CONSTANTS.lock().unwrap().max_width;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height;

        let mut out: Floor = Floor::empty_square();

        let rdm = rnd_gen.range_gen(1, (max_width.min(max_height)) >>1);

        out.divide_floor(&Point::new(0, rdm), &Point::new(max_width, rdm));
        out.divide_floor(&Point::new(rdm, 0), &Point::new(rdm, max_height));

        out.divide_floor(&Point::new(0, max_height-rdm), &Point::new(max_width, max_height-rdm));
        out.divide_floor(&Point::new(max_width-rdm, 0), &Point::new(max_width-rdm, max_height));

        out.rm_room(8);
        out.rm_room(5);
        out.rm_room(2);
        out.rm_room(0);

        return out;
    }

    fn empty_square() -> Self {
        let max_width = constants::CONSTANTS.lock().unwrap().max_width;
        let max_height = constants::CONSTANTS.lock().unwrap().max_height;

        return Floor{rooms: vec![SSpace::new(Cycle::new(vec![Point::new(10, max_height+10), Point::new(max_width+10, max_height+10), Point::new(max_width+10, 10), Point::new(10, 10)]))], centers: Vec::new()};
    }

    fn rm_room(&mut self, index: usize) {
        let cycle = self.rooms.remove(index);
    }


    fn add_windows(&mut self) {
        let tripple_window_propability = constants::CONSTANTS.lock().unwrap().tripple_window_propability;
        let double_window_propability = constants::CONSTANTS.lock().unwrap().double_window_propability;
        let single_window_propability = constants::CONSTANTS.lock().unwrap().single_window_propability;

        let tripple_window_length = constants::CONSTANTS.lock().unwrap().tripple_window_length;
        let double_window_length = constants::CONSTANTS.lock().unwrap().double_window_length;
        let single_window_length = constants::CONSTANTS.lock().unwrap().single_window_length;

        let window_gap = constants::CONSTANTS.lock().unwrap().window_gap;
        let window_width = constants::CONSTANTS.lock().unwrap().window_width;
        let window_distance_from_edge = constants::CONSTANTS.lock().unwrap().window_distance_from_edge;

        let to_shift = (window_gap << 1) + window_width;

        let mut rdm_num_gen = rand::thread_rng();
        let mut nat_dis_num_gen = NatDisNumGen::new_const();
        let mut rdm_number: f32;

        let mut segment_lenght;

        let mut shift: i32;
        let segments = self.rooms.iter().map(|sspace| sspace.get_cycle()).map(|c| c.into_segments()).flatten().collect::<Vec<[Point; 2]>>();
        let borders = segments.clone().into_iter().filter(|[p1, p2]| !segments.contains(&[p2.clone(), p1.clone()])).collect::<Vec<[Point;2]>>();

        for [p1, p2] in borders.iter() {
            rdm_number = rdm_num_gen.gen();
            segment_lenght = Point::distance(p1, p2).round() as i32;
            if rdm_number < tripple_window_propability && segment_lenght >= tripple_window_length {
                shift = nat_dis_num_gen.range_gen(1, segment_lenght - tripple_window_length) + window_gap+window_distance_from_edge;
                if p1.get_x()==p2.get_x() {
                    shift += p1.get_y().min(p2.get_y());
                    self.centers.push(Box::new(Window::new(p1.get_x(), shift, p1.get_x(), shift+window_width, nat_dis_num_gen.normal_gen())));
                    shift += to_shift;
                    self.centers.push(Box::new(Window::new(p1.get_x(), shift, p1.get_x(), shift+window_width, nat_dis_num_gen.normal_gen())));
                    shift += to_shift;
                    self.centers.push(Box::new(Window::new(p1.get_x(), shift, p1.get_x(), shift+window_width, nat_dis_num_gen.normal_gen())));
                } else {
                    shift += p1.get_x().min(p2.get_x());
                    self.centers.push(Box::new(Window::new(shift, p1.get_y(), shift+window_width, p1.get_y(), nat_dis_num_gen.normal_gen())));
                    shift += to_shift;
                    self.centers.push(Box::new(Window::new(shift, p1.get_y(), shift+window_width, p1.get_y(), nat_dis_num_gen.normal_gen())));
                    shift += to_shift;
                    self.centers.push(Box::new(Window::new(shift, p1.get_y(), shift+window_width, p1.get_y(), nat_dis_num_gen.normal_gen())));
                }
            } else if rdm_number < double_window_propability && segment_lenght >= double_window_length {
                shift = nat_dis_num_gen.range_gen(1, segment_lenght - double_window_length) + window_gap+window_distance_from_edge;
                if p1.get_x()==p2.get_x() {
                    shift += p1.get_y().min(p2.get_y());
                    self.centers.push(Box::new(Window::new(p1.get_x(), shift, p1.get_x(), shift+window_width, nat_dis_num_gen.normal_gen())));
                    shift += to_shift;
                    self.centers.push(Box::new(Window::new(p1.get_x(), shift, p1.get_x(), shift+window_width, nat_dis_num_gen.normal_gen())));
                } else {
                    shift += p1.get_x().min(p2.get_x());
                    self.centers.push(Box::new(Window::new(shift, p1.get_y(), shift+window_width, p1.get_y(), nat_dis_num_gen.normal_gen())));
                    shift += to_shift;
                    self.centers.push(Box::new(Window::new(shift, p1.get_y(), shift+window_width, p1.get_y(), nat_dis_num_gen.normal_gen())));
                }
            } else if rdm_number < single_window_propability && segment_lenght >= single_window_length {
                shift = nat_dis_num_gen.range_gen(1, segment_lenght - single_window_length) + window_gap+window_distance_from_edge;
                if p1.get_x()==p2.get_x() {
                    shift += p1.get_y().min(p2.get_y());
                    self.centers.push(Box::new(Window::new(p1.get_x(), shift, p1.get_x(), shift+window_width, nat_dis_num_gen.normal_gen())));
                } else {
                    shift += p1.get_x().min(p2.get_x());
                    self.centers.push(Box::new(Window::new(shift, p1.get_y(), shift+window_width, p1.get_y(), nat_dis_num_gen.normal_gen())));
                }
            }
        }
    }


    pub fn divide_by_centers(&mut self) {
        let centers = std::mem::replace(&mut self.centers, Vec::new());
        for center in centers.iter() {
            center.divide_floor(self);
        }
        self.centers = centers;
    }

    fn combine_sspaces(&mut self) {
        self.rooms = self.combine_and_return_sspaces();
    }

    fn combine_and_return_sspaces(&self) -> Vec<SSpace> {
        let mut out_sspaces: Vec<SSpace> = Vec::new();
        let mut to_be_connected = self.rooms.len();
        let mut id: u16 = 0;
        while to_be_connected > 0 {
            let in_sspace = self.rooms.iter().filter(|sspace| sspace.get_id() == id).collect::<Vec<&SSpace>>();
            id += 1;
            if in_sspace.len() == 0 {
                continue;
            }
            to_be_connected -= in_sspace.len();
            out_sspaces.extend(Cycle::connect_cycles(&(in_sspace.iter().map(|sspace| sspace.get_cycle()).collect::<Vec<&Cycle>>())).into_iter().map(|c| SSpace::new(c)).collect::<Vec<SSpace>>());
        }

        return out_sspaces;
    }
}