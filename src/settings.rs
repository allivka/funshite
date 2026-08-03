use minifb::{Key};
use std::collections::HashMap;
use crate::base::{Axis, DirectionKind, Motion, MotionKind};

const DEFAULT_LINEAR_SPEED: f64 = 1.0;
const DEFAULT_ROTATION_SPEED: f64 = 1.0;

pub struct Settings {
    pub keys: HashMap<Key, Motion>,
    pub speed_factor: f64,

}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            keys: HashMap::new(),
            speed_factor: 1.0,
        }
    }

    pub fn set_default_keys(&mut self) {
        self.keys.clear();
        
        //linear movement keys - WASD
        
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
        
        settings
    }
}


