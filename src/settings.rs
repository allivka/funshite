use minifb::{Key};
use std::collections::HashMap;
use std::f64::consts::PI;
use crate::base;
use crate::base::{Axis, DirectionKind, Motion, MotionKind};

pub const DEFAULT_LINEAR_SPEED: f64 = 1.0 / 15.0;
pub const DEFAULT_ROTATION_SPEED: f64 = PI / 180.0;
pub const DEFAULT_SPEED_FACTOR_ADDITION: f64 = 0.1;

pub struct Settings {
    pub keys: HashMap<Key, Motion>,
    pub speed_factor: f64,
    pub rotation_matrices: [[[f64; 3]; 3]; 6],
}

impl Settings {

    pub fn generate_rotation_matrices(&mut self) {
        self.rotation_matrices = [
            base::new_vec3d_rotation_matrix(Axis::X(DEFAULT_ROTATION_SPEED * self.speed_factor)),
            base::new_vec3d_rotation_matrix(Axis::Y(DEFAULT_ROTATION_SPEED * self.speed_factor)),
            base::new_vec3d_rotation_matrix(Axis::Z(DEFAULT_ROTATION_SPEED * self.speed_factor)),

            base::new_vec3d_rotation_matrix(Axis::X(-DEFAULT_ROTATION_SPEED * self.speed_factor)),
            base::new_vec3d_rotation_matrix(Axis::Y(-DEFAULT_ROTATION_SPEED * self.speed_factor)),
            base::new_vec3d_rotation_matrix(Axis::Z(-DEFAULT_ROTATION_SPEED * self.speed_factor)),
        ];
    }

    pub fn new() -> Settings {
        Settings {
            keys: HashMap::new(),
            speed_factor: 1.0,
            rotation_matrices: [base::I3X3; 6],
        }
    }

    pub fn set_default_keys(&mut self) {
        self.keys.clear();

        //surface linear movement keys - WASD

        self.keys.insert(Key::W, Motion {
            kind: MotionKind::Linear,
            direction: DirectionKind::Straight,
            axis: Axis::Z(-DEFAULT_LINEAR_SPEED),
        });

        self.keys.insert(Key::S, Motion {
            kind: MotionKind::Linear,
            direction: DirectionKind::Straight,
            axis: Axis::Z(DEFAULT_LINEAR_SPEED),
        });

        self.keys.insert(Key::D, Motion {
            kind: MotionKind::Linear,
            direction: DirectionKind::Horizontal,
            axis: Axis::X(DEFAULT_LINEAR_SPEED),
        });

        self.keys.insert(Key::A, Motion {
            kind: MotionKind::Linear,
            direction: DirectionKind::Horizontal,
            axis: Axis::X(-DEFAULT_LINEAR_SPEED),
        });

        //vertical movement keys - Space, LeftCtrl

        self.keys.insert(Key::Space, Motion {
            kind: MotionKind::Linear,
            direction: DirectionKind::Vertical,
            axis: Axis::Y(DEFAULT_LINEAR_SPEED),
        });

        self.keys.insert(Key::LeftCtrl, Motion {
            kind: MotionKind::Linear,
            direction: DirectionKind::Vertical,
            axis: Axis::Y(-DEFAULT_LINEAR_SPEED),
        });

        //Rotation keys - Left, Right, Up, Down keys

        self.keys.insert(Key::Up, Motion {
            kind: MotionKind::Rotation,
            direction: DirectionKind::Horizontal,
            axis: Axis::X(DEFAULT_ROTATION_SPEED),
        });

        self.keys.insert(Key::Down, Motion {
            kind: MotionKind::Rotation,
            direction: DirectionKind::Horizontal,
            axis: Axis::X(-DEFAULT_ROTATION_SPEED),
        });

        self.keys.insert(Key::Right, Motion {
            kind: MotionKind::Rotation,
            direction: DirectionKind::Vertical,
            axis: Axis::Y(-DEFAULT_ROTATION_SPEED),
        });

        self.keys.insert(Key::Left, Motion {
            kind: MotionKind::Rotation,
            direction: DirectionKind::Vertical,
            axis: Axis::Y(DEFAULT_ROTATION_SPEED),
        });

        // Z-rotation keys - Q, E

        self.keys.insert(Key::Q, Motion {
            kind: MotionKind::Rotation,
            direction: DirectionKind::Straight,
            axis: Axis::Z(DEFAULT_ROTATION_SPEED),
        });

        self.keys.insert(Key::E, Motion {
            kind: MotionKind::Rotation,
            direction: DirectionKind::Straight,
            axis: Axis::Z(-DEFAULT_ROTATION_SPEED),
        });


    }

    pub fn default_settings() -> Settings {
        let mut settings = Settings::new();
        settings.set_default_keys();
        settings.generate_rotation_matrices();

        settings
    }

    pub fn get_speed_factor(&self) -> f64 {
        self.speed_factor
    }

    pub fn set_speed_factor(&mut self, speed_factor: f64) {
        self.speed_factor = speed_factor;
    }

}


