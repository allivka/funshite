use minifb::{Key};
use std::collections::HashMap;
use std::f64::consts::PI;
use crate::base;
use crate::base::{Axis, DirectionKind, Motion, MotionKind};

const DEFAULT_LINEAR_SPEED: f64 = 1.0 / 2.0;
const DEFAULT_ROTATION_SPEED: f64 = PI / 90.0;

pub struct Settings {
    pub keys: HashMap<Key, Motion>,
    pub speed_factor: f64,
    pub rotation_matrices: [[[f64; 3]; 3]; 3],
}

impl Settings {

    pub fn generate_rotation_matrices(&mut self) {
        self.rotation_matrices = [
            base::new_vec3d_rotation_matrix(base::RotationMatrixKind::X(DEFAULT_ROTATION_SPEED * self.speed_factor)),
            base::new_vec3d_rotation_matrix(base::RotationMatrixKind::Y(DEFAULT_ROTATION_SPEED * self.speed_factor)),
            base::new_vec3d_rotation_matrix(base::RotationMatrixKind::Z(DEFAULT_ROTATION_SPEED * self.speed_factor)),
        ];
    }

    pub fn new() -> Settings {
        Settings {
            keys: HashMap::new(),
            speed_factor: 1.0,
            rotation_matrices: [base::I3X3; 3],
        }
    }

    pub fn set_default_keys(&mut self) {
        self.keys.clear();

        //surface linear movement keys - WASD

        self.keys.insert(Key::W, Motion {
            kind: MotionKind::Linear(-DEFAULT_LINEAR_SPEED),
            direction: DirectionKind::Straight,
            axis: Axis::Z,
        });

        self.keys.insert(Key::S, Motion {
            kind: MotionKind::Linear(DEFAULT_LINEAR_SPEED),
            direction: DirectionKind::Straight,
            axis: Axis::Z,
        });

        self.keys.insert(Key::D, Motion {
            kind: MotionKind::Linear(DEFAULT_LINEAR_SPEED),
            direction: DirectionKind::Horizontal,
            axis: Axis::X,
        });

        self.keys.insert(Key::A, Motion {
            kind: MotionKind::Linear(-DEFAULT_LINEAR_SPEED),
            direction: DirectionKind::Horizontal,
            axis: Axis::X,
        });

        //vertical movement keys - Space, LeftCtrl

        self.keys.insert(Key::Space, Motion {
            kind: MotionKind::Linear(DEFAULT_LINEAR_SPEED),
            direction: DirectionKind::Vertical,
            axis: Axis::Y,
        });

        self.keys.insert(Key::LeftCtrl, Motion {
            kind: MotionKind::Linear(-DEFAULT_LINEAR_SPEED),
            direction: DirectionKind::Vertical,
            axis: Axis::Y,
        });

        //Rotation keys - Left, Right, Up, Down keys

        self.keys.insert(Key::Up, Motion {
            kind: MotionKind::Rotation(DEFAULT_ROTATION_SPEED),
            direction: DirectionKind::Horizontal,
            axis: Axis::X,
        });

        self.keys.insert(Key::Down, Motion {
            kind: MotionKind::Rotation(-DEFAULT_ROTATION_SPEED),
            direction: DirectionKind::Horizontal,
            axis: Axis::X,
        });

        self.keys.insert(Key::Right, Motion {
            kind: MotionKind::Rotation(DEFAULT_ROTATION_SPEED),
            direction: DirectionKind::Vertical,
            axis: Axis::Y,
        });

        self.keys.insert(Key::Left, Motion {
            kind: MotionKind::Rotation(-DEFAULT_ROTATION_SPEED),
            direction: DirectionKind::Vertical,
            axis: Axis::Y,
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


