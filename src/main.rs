use minifb::{Key, Window, WindowOptions};
use std::cmp::{min, max};
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Color(u8, u8, u8, u8);

impl Color {
    fn tou32(self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32)
    }
}

struct Vec2isize(isize, isize);


struct Canvas {
    width: isize,
    height: isize,
    buffer: Vec<u32>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width as isize,
            height: height as isize,
            buffer: vec![0; width * height],
        }
    }


    fn fix(&self, mut pos: Vec2isize) -> Vec2isize {
        pos.0 = min(pos.0, self.width);
        pos.0 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.0 = max(pos.1, 0);

        pos
    }

    fn fix_r(&self, pos: &mut Vec2isize) {
        pos.0 = min(pos.0, self.width);
        pos.0 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.0 = max(pos.1, 0);

    }

    fn set(&mut self, pos: Vec2isize, color: Color) {
        self.buffer[((pos.1 - 1) * self.width + pos.0) as usize] = color.tou32();
    }

    fn set_fix(&mut self, mut pos: Vec2isize, color: Color) {
        self.fix_r(&mut pos);
        self.buffer[((pos.1 - 1) * self.width + pos.0) as usize] = color.tou32();
    }

    fn draw_line(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: Color) {
        self.fix_r(&mut start);
        self.fix_r(&mut end);

        let d0 = end.0 - start.0;
        let d1 = end.1 - start.1;
        let k = d1 as f32 / d0 as f32 * d1.signum() as f32;

        for i in start.0..end.0 {
            self.set(Vec2isize(start.0 + i * d0.signum(), (start.1 as f32 + (i as f32 * k)) as isize), color);
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}