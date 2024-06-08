use svg::node::element::SVG;

pub trait Disp {
    fn disp(&self, doc: SVG) -> SVG;
}