use crate::canvas::Canvas;
use crate::object::Object;
use crate::viewer::Viewer;

pub struct Renderer {
    pub objects: Vec<Object>,
    pub polygon_outline_thickness: isize,
}

impl Renderer {
    pub fn render(canvas: &mut Canvas, camera: &Viewer) {
        //TODO: implement rendering
    }
}

