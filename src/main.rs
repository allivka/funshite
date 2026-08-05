pub mod canvas;
pub mod base;
pub mod object;
pub mod viewer;
pub mod settings;
pub mod controller;
pub mod renderer;


use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use std::path::Path;
use base::{RGBA};
use canvas::Canvas;
use crate::base::Vec3d;
use crate::renderer::Renderer;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: true,
            scale: Scale::X1,
            scale_mode: ScaleMode::Center,
            topmost: false,
            transparency: false,
            none: false,

        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut config = settings::Settings::default_settings();

    let mut camera = viewer::Viewer::new(Vec3d(0.0, 0.0, 0.0));

    let objects = match object::parse_file(Path::new("test.obj")) {
        Ok(objects) => objects,
        Err(e) => {
            panic!("Failed parsing obj file:\t{}", e);
        }
    };

    let renderer = Renderer {
        objects,
        polygon_outline_color: RGBA::g(255),
        polygon_outline_thickness: 1,
        fov_degrees: 60.0,
        close_visibility_distance: 0.1,
        index_pair_buffer: Vec::new(),
        focal_length: 0.0,
    }.init(&canvas);

    let mut mouse_pos_prev: (f32, f32) = (0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mut keys = window.get_keys();

        window.get_scroll_wheel().map(|scroll| {
            config.set_speed_factor((config.speed_factor + scroll.1.signum() as f64 * settings::DEFAULT_SPEED_FACTOR_ADDITION).max(0.0));
        });

        if window.is_key_down(Key::T) {
            config.speed_factor = 1.0;
        }

        if window.is_key_down(Key::R) {
            config.generate_rotation_matrices();
        }

        window.get_mouse_pos(MouseMode::Pass).map(|mouse_pos| {

            if !window.get_mouse_down(MouseButton::Left) {
                mouse_pos_prev = mouse_pos;
                return;
            }

            match (((mouse_pos.0 - mouse_pos_prev.0) / 1.0) as i32).signum() {
                1 => keys.push(Key::Right),
                -1 => keys.push(Key::Left),
                _ => (),
            }

            match (((mouse_pos.1 - mouse_pos_prev.1) / 1.0) as i32).signum(){
                1 => keys.push(Key::Down),
                -1 => keys.push(Key::Up),
                _ => (),
            }

            mouse_pos_prev = mouse_pos;

        });

        controller::process_controls(&mut config, &mut camera, &keys);

        canvas.clear(RGBA::new());
        renderer.render(&mut canvas, &camera);
        window.update_with_buffer(canvas.unsafe_slice(), canvas.width as usize, canvas.height as usize).unwrap();
    }


}