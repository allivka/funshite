use crate::base;
use crate::base::{Vec3d};

pub struct Viewer {
    pub position: Vec3d,
    pub rotation: [[f64; 3]; 3],
}

impl Viewer {
    pub fn new(position: Vec3d) -> Viewer {
        Viewer {
            position,
            rotation: base::I3X3
        }
    }

    pub fn rel_rotate(&mut self, rotation: &[[f64; 3]; 3]) {
        self.rotation = base::multiply_matrix3x3(&self.rotation, &rotation);
    }

    pub fn rel_move(&mut self, shift: Vec3d) {
        self.position.0 += shift.0;
        self.position.1 += shift.1;
        self.position.2 += shift.2;
    }
}