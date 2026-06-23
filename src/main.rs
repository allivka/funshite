use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::cmp::{min, max};
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[derive(Copy, Clone)]
struct RGBA(u8, u8, u8, u8);

impl RGBA {
    fn tou32(&self) -> u32 {
        (self.3 as u32) << 24 | (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32)
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


    fn fixed(&self, mut pos: Vec2isize) -> Vec2isize {
        pos.0 = min(pos.0, self.width);
        pos.0 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.0 = max(pos.1, 0);

        pos
    }

    fn fix(&self, pos: &mut Vec2isize) {
        pos.0 = min(pos.0, self.width);
        pos.0 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.0 = max(pos.1, 0);

    }

    fn idx(pos: Vec2isize, canvas_size: Vec2isize) -> usize {
        min((max(pos.1 - 1, 0) * canvas_size.0 + pos.0), canvas_size.0 * canvas_size.1 - 1) as usize
    }

    fn idxx(self, pos: Vec2isize) -> usize {
        min((max(pos.1 - 1, 0) * self.width + pos.0), self.width * self.height - 1) as usize
    }

    fn set(&mut self, pos: Vec2isize, color: RGBA) {
        self.buffer[Self::idx(pos, Vec2isize(self.width, self.height))] = color.tou32();
    }

    fn set_fixed(&mut self, mut pos: Vec2isize, color: RGBA) {
        self.fix(&mut pos);
        self.buffer[(max(pos.1 - 1, 0) * self.width + pos.0) as usize] = color.tou32();
    }

    fn fill(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: RGBA) {
        self.fix(&mut start);
        self.fix(&mut end);

        for y in min(start.1, end.1)..max(start.1, end.1) {
            for x in min(start.0, end.0)..max(start.0, end.0) {
                self.set(Vec2isize(x, y), color);
            }
        }
    }

    fn draw_line(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: RGBA) {
        self.fix(&mut start);
        self.fix(&mut end);

        let d0 = end.0 - start.0;
        let d1 = end.1 - start.1;
        let k = d1 as f32 / d0 as f32 * d1.signum() as f32;

        for i in min(start.0, end.0)..=max(start.0, end.0) {
            self.set(Vec2isize(start.0 + i * d0.signum(), (start.1 as f32 + (i as f32 * k)) as isize), color);
        }
    }
}

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

    canvas.fill(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(0, 0, 255, 0));
    canvas.draw_line(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(255, 255, 255, 0));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&canvas.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}