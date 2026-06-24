pub mod canvas;
pub mod base;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

use base::{Vec2isize, RGBA};
use canvas::Canvas;

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
            topmost: true,
            transparency: false,
            none: false,

        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        canvas.fill_rect(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(0, 0, 255, 0));
        canvas.draw_line(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(255, 255, 255, 0));
        canvas.fill_triangle(Vec2isize(0, 300), Vec2isize(300, 300), Vec2isize(300, 450), RGBA(255, 0, 0, 0));

        canvas.fill_polygon(vec!{Vec2isize(0, 0), Vec2isize(200, 200)}, RGBA(255, 0, 255, 0));
        canvas.fill_polygon(vec!{Vec2isize(0, 300), Vec2isize(300, 300), Vec2isize(300, 450), Vec2isize(450, 1000)}, RGBA(255, 255, 0, 0));

        window.update_with_buffer(&canvas.buffer, WIDTH, HEIGHT).unwrap();
    }
}