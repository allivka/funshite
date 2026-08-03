use minifb::Key;
use crate::base::{Motion, MotionKind};
use crate::settings::Settings;
use crate::viewer::Viewer;

pub fn process_linear_motion(config: &Settings, camera: &mut Viewer, motion: &Motion) {
    //TODO
}

pub fn process_rotation(config: &Settings, camera: &mut Viewer, motion: &Motion) {
    if let MotionKind::Rotation(angle) = motion.kind {
        //TODO
    }
}

pub fn process_controls(config: &Settings, camera: &mut Viewer, pressed_keys: &Vec<Key>) {
    for key in pressed_keys {
        if let Some(motion) = config.keys.get(key) {
            match motion.kind {
                MotionKind::Linear(_) => process_linear_motion(config, camera, motion),
                MotionKind::Rotation(_) => process_rotation(config, camera, motion),
            }
        } else {
            continue;
        }
    }
}