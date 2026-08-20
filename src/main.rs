pub mod canvas;
pub mod base;
pub mod object;
pub mod viewer;
pub mod settings;
pub mod controller;
pub mod renderer;
pub mod slider;

use std::sync::atomic::AtomicU32;
use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use base::{RGBA};
use canvas::Canvas;
use crate::base::{Vec2i, Vec3d};
use crate::renderer::Renderer;
use std::cell::Cell;
use crate::slider::{ColorSlider};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[derive(Copy, Clone)]
struct ColorsData {
    background: RGBA,
    outline: RGBA,
}


struct SettingWindow {
    w: usize,
    h: usize,
    canvas: Canvas<Cell<u32>>,
    background_slider: ColorSlider,
    outline_slider: ColorSlider,
}

impl SettingWindow {
    fn new() -> Self {
        let w = 700;
        let h = 950;

        let canvas: Canvas<Cell<u32>> = Canvas::new(w, h);

        let slider_size = Vec2i(500, 100);

        let mut background_slider = ColorSlider::new(
            Vec2i(100, 50),
            slider_size,
        );

        background_slider.a.slider_offset = slider_size.0;

        let mut outline_slider = ColorSlider::new(
            Vec2i(100, 50 + slider_size.1 * 4 + 50),
            slider_size,
        );

        outline_slider.g.slider_offset = slider_size.0;
        outline_slider.a.slider_offset = slider_size.0;

        Self {
            w,
            h,
            canvas,
            background_slider,
            outline_slider
        }
    }

    fn work(&mut self) -> ColorsData {

        let mut window = Window::new(
            "Settings - ESC to exit",
            self.w,
            self.h,
            WindowOptions {
                borderless: false,
                title: true,
                resize: false,
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

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.background_slider.update(&window);
            self.outline_slider.update(&window);

            self.canvas.clear(RGBA::white());

            self.background_slider.draw(&self.canvas);
            self.outline_slider.draw(&self.canvas);

            window.update_with_buffer(self.canvas.unsafe_slice(), self.w, self.h).unwrap();

        };

        ColorsData {
            background: self.background_slider.calc_color(),
            outline: self.outline_slider.calc_color(),
        }
    }
}

fn main() {

    let mut canvas = Canvas::<AtomicU32>::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Funshite - ESC to exit",
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

    let objects;

    if let Some(file_path) = rfd::FileDialog::new().add_filter("OBJ file", &["obj"]).pick_file() {
        objects = match object::parse_file(file_path.as_path()) {
            Ok(objects) => objects,
            Err(e) => {
                panic!("Failed parsing obj file:\t{}", e);
            }
        };
    } else {
        panic!("No file found");
    };

    let mut renderer = Renderer {
        objects,
        polygon_outline_color: RGBA::g(255),
        polygon_outline_thickness: 1,
        fov_degrees: 60.0,
        close_visibility_distance: 0.1,
        far_visibility_distance: f32::MAX,
        index_pair_buffer: Vec::new(),
        focal_length: 0.0,
    }.init(&canvas);

    let mut background_color = RGBA::black();

    let mut settings_window = SettingWindow::new();

    let mut mouse_pos_prev: (f32, f32) = (0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mut keys = window.get_keys();

        window.get_scroll_wheel().map(|scroll| {
            config.set_speed_factor((config.speed_factor + scroll.1.signum() as f64 * settings::DEFAULT_SPEED_FACTOR_ADDITION).max(0.0));
        });

        if window.is_key_down(Key::PageUp) {
            config.set_speed_factor((config.speed_factor + settings::DEFAULT_SPEED_FACTOR_ADDITION).max(0.0));
        }

        if window.is_key_down(Key::PageDown) {
            config.set_speed_factor((config.speed_factor - settings::DEFAULT_SPEED_FACTOR_ADDITION).max(0.0));
        }

        if window.is_key_down(Key::H) {
            camera = viewer::Viewer::new(Vec3d(0.0, 0.0, 0.0));
        }

        if window.is_key_down(Key::F) {
            if let Some(image_path) = rfd::FileDialog::new().add_filter("PNG Image", &["png"]).save_file() {
                match canvas.save_to_png(&image_path) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Failed saving png: {:?}", e);
                    }
                }
            }
        }

        if window.is_key_released(Key::G) {
            let colors = settings_window.work();
            renderer.polygon_outline_color = colors.outline;
            background_color = colors.background;
        }

        if window.is_key_down(Key::Comma) {
            if renderer.polygon_outline_thickness > 1 {
                renderer.polygon_outline_thickness -= 1;
            }
        }

        if window.is_key_down(Key::Period) {
            renderer.polygon_outline_thickness += 1;
        }

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

        canvas.clear(background_color);
        renderer.render(&mut canvas, &camera);
        window.update_with_buffer(canvas.unsafe_slice(), canvas.width as usize, canvas.height as usize).unwrap();
    }


}