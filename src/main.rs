use crate::floor::{floor::Floor, disp::Disp};
use svg::Document;
use crate::geometry::point::Point;

mod constants;
mod geometry;
mod floor;
mod nat_dis_num_gen;

fn main() {
    let mut doc = Document::new();

    let floor = Floor::random_floor();

    doc = floor.disp(doc);
    let _ = svg::save("./floor.svg", &doc);

}
