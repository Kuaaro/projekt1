mod constants;
mod geometry;
mod floor;
mod nat_dis_num_gen;

use egui::Checkbox;
use svg::Document;
use crate::floor::{floor::Floor, disp::Disp};
use eframe::egui;
use std::env;

fn main() -> Result<(), eframe::Error> {

    let path = format!("{}", env::current_exe().unwrap().display());
    let path = path[0..path.rfind("/").unwrap()].to_owned();
    println!("{path}");

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    let mut max_height = constants::MAX_HEIGHT;
    let mut max_width = constants::MAX_WIDTH;
    let mut variance = constants::VARIANCE;
    let mut window_gap = constants::WINDOW_GAP;
    let mut window_distance_from_edge = constants::WINDOW_DISTANCE_FROM_EDGE;
    let mut single_window_propability = constants::SINGLE_WINDOW_PROPABILITY;
    let mut double_window_propability = constants::DOUBLE_WINDOW_PROPABILITY;
    let mut tripple_window_propability = constants::TRIPPLE_WINDOW_PROPABILITY;
    let mut window_width = constants::WINDOW_WIDTH;
    let mut min_room_wall_length = constants::MIN_ROOM_WALL_LENGTH;
    let mut min_room_area = constants::MIN_ROOM_AREA;
    let mut min_door_length = constants::MIN_DOOR_LENGTH;
    let mut staircase_size = constants::STAIRCASE_SIZE;
    let mut staircase_sspace = constants::STAIRCASE_SSPACE;
    let mut floor_type = constants::FLOOR_TYPE;
    let mut distance_method = constants::DISTANCE_METHOD;
    let mut weights_used = constants::WEIGHTS_USED;

    eframe::run_simple_native("Office Generator", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Office Generator");

            ui.add(egui::Slider::new(&mut max_height, 10..=5000).text("Height"));
            ui.add(egui::Slider::new(&mut max_width, 10..=5000).text("Width"));
            ui.add(egui::Slider::new(&mut variance, (0.)..=2.).text("Variance"));
            ui.add(egui::Slider::new(&mut window_width, 0..=200).text("Window Width"));
            ui.add(egui::Slider::new(&mut window_gap, 0..=100).text("Window Gap"));
            ui.add(egui::Slider::new(&mut window_distance_from_edge, 0..=200).text("Window Distance From Edge"));
            ui.add(egui::Slider::new(&mut single_window_propability, (0.)..=1.).text("Single Window Propability"));
            ui.add(egui::Slider::new(&mut double_window_propability, (0.)..=1.).text("Double Window Propability"));
            ui.add(egui::Slider::new(&mut tripple_window_propability, (0.)..=1.).text("Tripple Window Propability"));
            ui.add(egui::Slider::new(&mut min_room_wall_length, 0..=500).text("Min Room Wall Length"));
            ui.add(egui::Slider::new(&mut min_room_area, min_room_wall_length * min_room_wall_length..=250_000).text("Min Room Area"));
            ui.add(egui::Slider::new(&mut min_door_length, 0..=250).text("Min Door Length"));
            ui.add(egui::Slider::new(&mut staircase_size, 10..=100).text("Staircase Size"));
            ui.add(egui::Slider::new(&mut staircase_sspace, staircase_size..=250).text("Staircase Sspace"));

            ui.add(Checkbox::new(&mut weights_used, "Use Weights"));

            egui::ComboBox::from_label("Floor Type")
                .selected_text(String::from(floor_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut floor_type, 'l', "L-Floor");
                    ui.selectable_value(&mut floor_type, 'h', "H-Floor");
                    ui.selectable_value(&mut floor_type, '+', "+-Floor");
            });

            egui::ComboBox::from_label("Distance Method")
                .selected_text(String::from(distance_method))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut distance_method, 'r', "Regular");
                    ui.selectable_value(&mut distance_method, 's', "Squared");
                    ui.selectable_value(&mut distance_method, 'c', "City");
            });


            if ui.button("Build Floor").clicked() {
                {
                    let mut constants_mutex = constants::CONSTANTS.lock().unwrap();
                    
                    constants_mutex.floor_type = floor_type;
                    constants_mutex.distance_method = distance_method;

                    constants_mutex.max_height = max_height;
                    constants_mutex.max_width = max_width;
                    constants_mutex.variance = variance;
                    constants_mutex.window_width = window_width;
                    constants_mutex.window_gap = window_gap;
                    constants_mutex.window_distance_from_edge = window_distance_from_edge;
                    constants_mutex.single_window_propability = single_window_propability;
                    constants_mutex.double_window_propability = double_window_propability;
                    constants_mutex.tripple_window_propability = tripple_window_propability;
                    constants_mutex.min_room_wall_length = min_room_wall_length;
                    constants_mutex.min_room_area = min_room_area;
                    constants_mutex.min_door_length = min_door_length;
                    constants_mutex.staircase_size = staircase_size;
                    constants_mutex.staircase_sspace = staircase_sspace;

                    let temp_window_length: i32 = (window_gap << 1)+window_width;
                    let double_window_edge: i32 = window_distance_from_edge << 1;

                    constants_mutex.single_window_length = temp_window_length+double_window_edge;
                    constants_mutex.double_window_length = 2*temp_window_length+double_window_edge;
                    constants_mutex.tripple_window_length = 3*temp_window_length+double_window_edge;
                }
                make_and_save_floor(&path);
            }
        });
    })
}


pub fn make_and_save_floor(path: &String) {
    let mut doc = Document::new();

    let floor = Floor::random_floor();

    doc = floor.disp(doc);
    let _ = svg::save(format!("{}{}", path, "/floor.svg"), &doc);
    
}
