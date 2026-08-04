pub mod canvas;
pub mod base;
pub mod object;
pub mod viewer;
pub mod settings;
pub mod controller;


use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::path::Path;
use base::{Vec2i, RGBA};
use canvas::Canvas;
use object::Object;
use crate::base::Vec3d;

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
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut config = settings::Settings::default_settings();

    let mut camera = viewer::Viewer::new(Vec3d(0.0, 0.0, 0.0));


    let objects = match object::parse_file(Path::new("test.obj")) {
        Ok(objects) => {
            println!("Successfully parsed obj file");
            objects
        },
        Err(e) => {panic!("Failed parsing obj file:\t{}", e);}
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {

        controller::process_controls(&mut config, &mut camera, &window.get_keys());

        canvas.fill_rect(Vec2i(0, 0), Vec2i(200, 200), RGBA { r: 0, g: 0, b: 255, a: 0 });
        canvas.draw_line(Vec2i(0, 0), Vec2i(200, 200), RGBA { r: 255, g: 255, b: 255, a: 0 });
        canvas.fill_triangle(Vec2i(0, 300), Vec2i(300, 300), Vec2i(300, 450), RGBA { r: 255, g: 0, b: 0, a: 0 });

        canvas.fill_polygon(vec!{Vec2i(0, 0), Vec2i(200, 200)}, RGBA { r: 255, g: 0, b: 255, a: 0 });
        canvas.fill_polygon(vec!{Vec2i(0, 300), Vec2i(300, 300), Vec2i(300, 450), Vec2i(450, 1000)}, RGBA { r: 255, g: 255, b: 0, a: 0 });

        window.update_with_buffer(&canvas.buffer, canvas.width as usize, canvas.height as usize).unwrap();
    }


}