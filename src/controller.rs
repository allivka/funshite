use minifb::Key;
use crate::base::{Axis, Motion, MotionKind, Vec3d};
use crate::settings::Settings;
use crate::viewer::Viewer;

pub fn process_linear_motion(config: &Settings, camera: &mut Viewer, motion: &Motion) {

    //TODO: make movement relative to face direction

    let (Axis::X(speed) | Axis::Y(speed) | Axis::Z(speed)) = motion.axis;

    let shift = match motion.axis {
        Axis::X(_) => Vec3d(speed * config.speed_factor, 0.0, 0.0),
        Axis::Y(_) => Vec3d(0.0, speed * config.speed_factor, 0.0),
        Axis::Z(_) => Vec3d(0.0, 0.0, speed * config.speed_factor),
    };

    camera.rel_move(shift);
}

pub fn process_rotation(config: &Settings, camera: &mut Viewer, motion: &Motion) {
    let axis_index = match motion.axis {
        Axis::X(_) => 0,
        Axis::Y(_) => 1,
        Axis::Z(_) => 2,
    };
    camera.rel_rotate(config.rotation_matrices[axis_index]);
}

pub fn process_controls(config: &Settings, camera: &mut Viewer, pressed_keys: &Vec<Key>) {
    for key in pressed_keys {
        if let Some(motion) = config.keys.get(key) {
            match motion.kind {
                MotionKind::Linear => process_linear_motion(config, camera, motion),
                MotionKind::Rotation => process_rotation(config, camera, motion),
            }
        }
    }
}