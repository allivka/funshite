pub mod canvas;
pub mod base;
pub mod object;
pub mod viewer;
pub mod settings;
pub mod controller;
pub mod renderer;
pub mod slider;

use std::sync::atomic::AtomicU32;
use std::time::{Duration, Instant};
use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use base::{RGBA};
use canvas::Canvas;
use crate::base::{Vec2i, Vec3d};
use crate::renderer::Renderer;
use std::cell::Cell;
use crate::slider::ColorSlider;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const DEFAULT_FOV_DEGREES: f64 = 60.0;
const MODEL_SPEED_SCALE_COEFFICIENT: f64 = 1.3;

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

        let mut background_slider = ColorSlider::with_name(
            Vec2i(100, 50),
            slider_size,
            "BACKGROUND COLOR",
        );

        background_slider.a.slider_offset = slider_size.0;

        let mut outline_slider = ColorSlider::with_name(
            Vec2i(100, 50 + slider_size.1 * 4 + 50),
            slider_size,
            "OUTLINE COLOR",
        );

        outline_slider.g.slider_offset = slider_size.0;
        outline_slider.a.slider_offset = slider_size.0;
        background_slider.set_reset_to_current();
        outline_slider.set_reset_to_current();

        Self {
            w,
            h,
            canvas,
            background_slider,
            outline_slider
        }
    }

    fn work(&mut self, main_window: &mut Window) -> ColorsData {

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

        while window.is_open()
            && main_window.is_open()
            && !window.is_key_down(Key::Escape)
            && !window.is_key_down(Key::G)
        {
            self.background_slider.update(&window);
            self.outline_slider.update(&window);
            self.canvas.clear(RGBA::white());

            self.background_slider.draw(&self.canvas);
            self.outline_slider.draw(&self.canvas);

            window.update_with_buffer(self.canvas.unsafe_slice(), self.w, self.h).unwrap();
            main_window.update();

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

    show_keybinds_guide(&mut window, &mut canvas);
    wait_for_keys_release(&mut window);
    if !window.is_open() {
        return;
    }

    let mut config = settings::Settings::default_settings();

    let objects;

    if let Some(file_path) = rfd::FileDialog::new().add_filter("OBJ file", &["obj"]).pick_file() {
        objects = match object::parse_file(file_path.as_path()) {
            Ok(objects) => objects,
            Err(e) => {
                panic!("Failed parsing obj file:\t{}", e);
            }
        };
    } else {
        return;
    };

    config.model_speed_scale = model_size_scale(&objects);
    let mut model_polygon_count = polygon_count(&objects);
    let mut camera_start_position = model_view_position(&objects, DEFAULT_FOV_DEGREES);
    let mut camera = viewer::Viewer::new(camera_start_position);

    let mut renderer = Renderer {
        objects,
        polygon_outline_color: RGBA::g(255),
        polygon_outline_thickness: 1,
        fov_degrees: DEFAULT_FOV_DEGREES as f32,
        close_visibility_distance: 0.1,
        far_visibility_distance: f32::MAX,
        index_pair_buffer: Vec::new(),
        focal_length: 0.0,
    }.init(&canvas);
    let mut model_loaded_at = Instant::now();

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
            camera = viewer::Viewer::new(camera_start_position);
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

        if window.is_key_released(Key::L) {
            if let Some(file_path) =
                rfd::FileDialog::new().add_filter("OBJ file", &["obj"]).pick_file()
            {
                match object::parse_file(file_path.as_path()) {
                    Ok(objects) => {
                        config.model_speed_scale = model_size_scale(&objects);
                        model_polygon_count = polygon_count(&objects);
                        camera_start_position =
                            model_view_position(&objects, DEFAULT_FOV_DEGREES);
                        camera = viewer::Viewer::new(camera_start_position);
                        renderer.objects = objects;
                        renderer.index_pair_buffer.clear();
                        renderer.initialize_index_pair_buffer();
                        model_loaded_at = Instant::now();
                    }
                    Err(error) => {
                        eprintln!("Failed parsing OBJ file: {error}");
                    }
                }
            }
            wait_for_keys_release(&mut window);
            mouse_pos_prev = window
                .get_mouse_pos(MouseMode::Pass)
                .unwrap_or(mouse_pos_prev);
            continue;
        }

        if window.is_key_released(Key::G) {
            let colors = settings_window.work(&mut window);
            renderer.polygon_outline_color = colors.outline;
            background_color = colors.background;
            wait_for_keys_release(&mut window);
        }

        if window.is_key_released(Key::F1) {
            show_keybinds_guide(&mut window, &mut canvas);
            wait_for_keys_release(&mut window);
            mouse_pos_prev = window
                .get_mouse_pos(MouseMode::Pass)
                .unwrap_or(mouse_pos_prev);
            continue;
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
        if model_loaded_at.elapsed() < Duration::from_secs(5) {
            let status_text = format!(
                "{} POLYGONS MODEL LOADED",
                format_count_with_underscores(model_polygon_count)
            );
            let status_scale = 2;
            let status_width = status_text.chars().count() as isize * 6 * status_scale;
            draw_guide_text(
                &canvas,
                Vec2i(canvas.width - status_width - 32, 24),
                &status_text,
                status_scale,
                renderer.polygon_outline_color,
            );
        }
        window.update_with_buffer(canvas.unsafe_slice(), canvas.width as usize, canvas.height as usize).unwrap();
    }


}

fn polygon_count(objects: &[object::Object]) -> usize {
    objects.iter().map(|object| object.faces.len()).sum()
}

fn format_count_with_underscores(count: usize) -> String {
    let digits = count.to_string();
    let mut formatted = String::with_capacity(digits.len() + digits.len() / 3);

    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            formatted.push('_');
        }
        formatted.push(digit);
    }

    formatted
}

fn model_size_scale(objects: &[object::Object]) -> f64 {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_z = f64::NEG_INFINITY;

    for vertex in objects.iter().flat_map(|object| &object.vertices) {
        if !vertex.0.is_finite() || !vertex.1.is_finite() || !vertex.2.is_finite() {
            continue;
        }

        min_x = min_x.min(vertex.0);
        max_x = max_x.max(vertex.0);
        min_y = min_y.min(vertex.1);
        max_y = max_y.max(vertex.1);
        min_z = min_z.min(vertex.2);
        max_z = max_z.max(vertex.2);
    }

    if !min_x.is_finite() {
        return 1.0;
    }

    let size = (max_x - min_x)
        .max(max_y - min_y)
        .max(max_z - min_z);
    if size > 0.0 {
        size / MODEL_SPEED_SCALE_COEFFICIENT
    } else {
        1.0
    }
}

fn model_view_position(objects: &[object::Object], fov_degrees: f64) -> Vec3d {
    let mut vertex_count = 0usize;
    let mut center_x = 0.0;
    let mut center_y = 0.0;
    let mut center_z = 0.0;
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_z = f64::NEG_INFINITY;

    for vertex in objects.iter().flat_map(|object| &object.vertices) {
        if !vertex.0.is_finite() || !vertex.1.is_finite() || !vertex.2.is_finite() {
            continue;
        }

        vertex_count += 1;
        center_x += vertex.0;
        center_y += vertex.1;
        center_z += vertex.2;
        min_x = min_x.min(vertex.0);
        max_x = max_x.max(vertex.0);
        min_y = min_y.min(vertex.1);
        max_y = max_y.max(vertex.1);
        min_z = min_z.min(vertex.2);
        max_z = max_z.max(vertex.2);
    }

    if vertex_count == 0 {
        return Vec3d(0.0, 0.0, 0.0);
    }

    center_x /= vertex_count as f64;
    center_y /= vertex_count as f64;
    center_z /= vertex_count as f64;

    let half_width = (max_x - center_x).abs().max((min_x - center_x).abs());
    let half_height = (max_y - center_y).abs().max((min_y - center_y).abs());
    let half_depth = (max_z - center_z).abs().max((min_z - center_z).abs());
    let half_fov = (fov_degrees.to_radians() / 2.0).tan();
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let distance = half_depth
        + (half_width / half_fov).max(half_height / (half_fov / aspect_ratio)) * 1.2
        + 0.1;

    Vec3d(center_x, center_y, center_z + distance)
}

fn wait_for_keys_release(window: &mut Window) {
    while window.is_open() && !window.get_keys().is_empty() {
        window.update();
    }
    if window.is_open() {
        window.update();
    }
}

fn show_keybinds_guide(window: &mut Window, canvas: &mut Canvas<AtomicU32>) {
    let lines = [
        "CPU BASED REAL TIME 3D RENDERER",
        "Load an OBJ model and inspect its polygon outlines.",
        "Rendering is performed entirely on the CPU.",
        "Press F to save the current view as a PNG image.",
        "",
        "CAMERA MOVEMENT",
        "W / A / S / D   Move forward / left / backward / right",
        "SPACE / CTRL    Move up / down",
        "",
        "CAMERA ROTATION",
        "UP / DOWN       Rotate upward / downward",
        "LEFT / RIGHT    Rotate right / left",
        "LEFT MOUSE + DRAG  Rotate with mouse movement",
        "Q / E           Rotate around Z axis left / right",
        "",
        "SPEED AND OTHER CONTROLS",
        "PAGE UP / WHEEL UP    Increase movement speed",
        "PAGE DOWN / WHEEL DOWN  Decrease movement speed",
        "T               Reset movement speed",
        "R               Sync rotation speed with movement speed",
        "COMMA / PERIOD  Decrease / increase outline thickness",
        "F               Save the view as PNG",
        "L               Load a different OBJ model",
        "H               Reset camera position",
        "G               Open the color settings window",
        "F1              Show this guide again",
        "",
        "Press ENTER, ESC, or F1 to close this guide.",
    ];

    while window.is_open()
        && !window.is_key_down(Key::Enter)
        && !window.is_key_down(Key::Escape)
        && !window.is_key_down(Key::F1)
    {
        canvas.clear(RGBA::white());
        draw_guide_text(
            canvas,
            Vec2i(64, 54),
            lines[0],
            4,
            RGBA::black(),
        );

        for (index, line) in lines.iter().enumerate().skip(1) {
            draw_guide_text(
                canvas,
                Vec2i(68, 130 + (index as isize - 1) * 32),
                line,
                2,
                RGBA::black(),
            );
        }

        if window
            .update_with_buffer(canvas.unsafe_slice(), canvas.width as usize, canvas.height as usize)
            .is_err()
        {
            break;
        }
    }
}

fn draw_guide_text(
    canvas: &Canvas<AtomicU32>,
    pos: Vec2i,
    text: &str,
    scale: isize,
    color: RGBA,
) {
    for (index, character) in text.chars().enumerate() {
        let rows = match character.to_ascii_uppercase() {
            'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
            'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
            'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
            'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
            'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
            'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
            'G' => [0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111],
            'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
            'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
            'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100],
            'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
            'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
            'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
            'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
            'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
            'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
            'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
            'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
            'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
            'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
            'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
            'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
            'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
            'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
            'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
            'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
            '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
            '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
            '2' => [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111],
            '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
            '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
            '5' => [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110],
            '6' => [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
            '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
            '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
            '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
            '/' => [0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000],
            '+' => [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
            '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
            '_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
            '.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00110, 0b00110],
            ',' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00110, 0b00110, 0b00100],
            _ => [0; 7],
        };
        let char_x = pos.0 + index as isize * 6 * scale;
        for (y, row) in rows.iter().enumerate() {
            for x in 0..5 {
                if row & (1 << (4 - x)) != 0 {
                    for pixel_y in 0..scale {
                        for pixel_x in 0..scale {
                            canvas.set(
                                Vec2i(
                                    char_x + x * scale + pixel_x,
                                    pos.1 + y as isize * scale + pixel_y,
                                ),
                                color,
                            );
                        }
                    }
                }
            }
        }
    }
}