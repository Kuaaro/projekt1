use std::sync::Mutex;
use lazy_static::lazy_static;

pub const FLOOR_TYPE: char = 'h';
pub const MAX_HEIGHT: i32 = 500;
pub const MAX_WIDTH: i32 = 1000;
pub const MEAN: f32 = 0.5;
pub const VARIANCE: f32 = 0.2;
pub const WINDOW_WIDTH: i32 = 50;
pub const WINDOW_GAP: i32 = 25;
pub const WINDOW_DISTANCE_FROM_EDGE: i32 = 30;
pub const SINGLE_WINDOW_PROPABILITY: f32 = 0.7;
pub const DOUBLE_WINDOW_PROPABILITY: f32 = 0.5;
pub const TRIPPLE_WINDOW_PROPABILITY: f32 = 0.3;
pub const MIN_ROOM_WALL_LENGTH: i32 = 20;
pub const MIN_ROOM_AREA: i32 = 600;
pub const MIN_DOOR_LENGTH: i32 = 50;
pub const STAIRCASE_SIZE: i32 = 10;
pub const STAIRCASE_SSPACE: i32 = 10;
pub const DISTANCE_METHOD: char = 'r';
pub const WEIGHTS_USED: bool = true;
pub const PORK_BELLY: i32 = 20;

pub struct GlobalManager {
    pub floor_type: char,
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
    pub min_room_area: i32,
    //todo: maximal distance that is allowed for sspace to be from center
    pub min_door_length: i32,
    pub staircase_size: i32,
    pub staircase_sspace: i32,
    pub distance_method: char,
    pub weights_used: bool
}

impl GlobalManager {

    pub fn new() -> Self {


        let temp_window_length: i32 = (WINDOW_GAP << 1)+WINDOW_WIDTH;
        let double_window_edge: i32 = WINDOW_DISTANCE_FROM_EDGE << 1;
        let single_window_length: i32 = temp_window_length+double_window_edge;
        let double_window_length: i32 = 2*temp_window_length+double_window_edge;
        let tripple_window_length: i32 = 3*temp_window_length+double_window_edge;

        return GlobalManager{
            floor_type: FLOOR_TYPE,
            max_height: MAX_HEIGHT,
            max_width: MAX_HEIGHT,
            mean: MEAN,
            variance: VARIANCE,
            window_width: WINDOW_WIDTH,
            window_gap: WINDOW_GAP,
            window_distance_from_edge: WINDOW_DISTANCE_FROM_EDGE,
            single_window_propability: SINGLE_WINDOW_PROPABILITY,
            double_window_propability: DOUBLE_WINDOW_PROPABILITY,
            tripple_window_propability: TRIPPLE_WINDOW_PROPABILITY,
            single_window_length,
            double_window_length,
            tripple_window_length,
            min_room_wall_length: MIN_ROOM_WALL_LENGTH,
            min_room_area: MIN_ROOM_AREA,
            min_door_length: MIN_DOOR_LENGTH,
            staircase_size: STAIRCASE_SIZE,
            staircase_sspace: STAIRCASE_SSPACE,
            distance_method: DISTANCE_METHOD,
            weights_used: WEIGHTS_USED
        };
    }
}

lazy_static! {
    pub static ref CONSTANTS: Mutex<GlobalManager> = Mutex::new(GlobalManager::new());
}
