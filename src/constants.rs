use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct GlobalManager {
    pub max_height: i32,
    pub max_width: i32,
    pub mean: f32,
    pub variance: f32,
    pub window_width: i32,
    pub window_gap: i32,
    pub window_distance_from_edge: i32,
    pub single_window_propability: f32,
    pub double_window_propability: f32,
    pub tripple_window_propability: f32,
    pub single_window_length: i32,
    pub double_window_length: i32,
    pub tripple_window_length: i32,
    pub min_room_wall_length: i32,
    //pub point_eq_diff: f32,
    pub min_room_area: i32
    //todo: maximal distance that is allowed for sspace to be from center
}

impl GlobalManager {
    pub fn set_max_height(&mut self, max_height: i32) {
        self.max_height=max_height;
    }
    pub fn set_max_width(&mut self, max_width: i32) {
        self.max_width=max_width;
    }
    pub fn set_variance(&mut self, variance: f32) {
        self.variance=variance;
    }
    pub fn set_window_width(&mut self, window_width: i32) {
        self.window_width=window_width;
        self.update_window_lengths();
    }
    pub fn set_window_gap(&mut self, window_gap: i32) {
        self.window_gap=window_gap;
        self.update_window_lengths();
    }
    pub fn set_window_distance_from_edge(&mut self, window_distance_from_edge: i32) {
        self.window_distance_from_edge=window_distance_from_edge;
        self.update_window_lengths();
    }
    pub fn set_single_window_propability(&mut self, single_window_propability: f32) {
        self.single_window_propability=single_window_propability;
    }
    pub fn set_double_window_propability(&mut self, double_window_propability: f32) {
        self.double_window_propability=double_window_propability;
    }
    pub fn set_tripple_window_propability(&mut self, tripple_window_propability: f32) {
        self.tripple_window_propability=tripple_window_propability;
    }
    pub fn set_min_room_wall_length(&mut self, min_room_wall_length: i32) {
        //todo: Zmiana minimalnego pola, żeby nie było mniejsze
        self.min_room_wall_length=min_room_wall_length;
    }

    pub fn set_min_room_area(&mut self, max_height: i32) {
        self.max_height=max_height;
    }

    fn update_window_lengths(&mut self) {
        let temp_window_length: i32 = (self.window_gap << 1)+self.window_width;
        let double_window_edge: i32 = self.window_distance_from_edge << 1;
        self.single_window_length = temp_window_length+double_window_edge;
        self.double_window_length = 2*temp_window_length+double_window_edge;
        self.tripple_window_length = 3*temp_window_length+double_window_edge;
    }


    pub fn new() -> Self {
        let max_height: i32 = 500;
        let max_width: i32 = 1000;
        let mean: f32 = 0.5;
        let variance: f32 = 0.2;
        let window_width: i32 = 50;
        let window_gap: i32 = 25;
        let window_distance_from_edge: i32 = 30;
        let single_window_propability: f32 = 0.7;
        let double_window_propability: f32 = 0.5;
        let tripple_window_propability: f32 = 0.3;
        let min_room_wall_length: i32 = 20;
        let min_room_area: i32 = 600;
        //let point_eq_diff: f32 = 0.01;

        let temp_window_length: i32 = (window_gap << 1)+window_width;
        let double_window_edge: i32 = window_distance_from_edge << 1;
        let single_window_length: i32 = temp_window_length+double_window_edge;
        let double_window_length: i32 = 2*temp_window_length+double_window_edge;
        let tripple_window_length: i32 = 3*temp_window_length+double_window_edge;

        return GlobalManager{
            max_height,
            max_width,
            mean,
            variance,
            window_width,
            window_gap,
            window_distance_from_edge,
            single_window_propability,
            double_window_propability,
            tripple_window_propability,
            single_window_length,
            double_window_length,
            tripple_window_length,
            min_room_wall_length,
            //point_eq_diff,
            min_room_area
        };
    }
}

lazy_static! {
    pub static ref CONSTANTS: Mutex<GlobalManager> = Mutex::new(GlobalManager::new());
}
