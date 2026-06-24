use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::cmp::{min, max};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[derive(Copy, Clone)]
struct Vec2isize(isize, isize);

#[derive(Copy, Clone)]
struct LineCoefficient {
    k: f32,
    b: f32
}

impl LineCoefficient {
    fn new_from_line(start: Vec2isize, end: Vec2isize) -> Self {
        let dx = (end.0 - start.0) as f32;
        let dy = (end.1 - start.1) as f32;

        let k = if dx == 0.0 {
            f32::INFINITY
        } else {
            dy / dx
        };

        let b = if dx == 0.0 {
            start.0 as f32
        } else {
            start.0 as f32 - k * start.0 as f32
        };

        LineCoefficient { k, b }
    }
}

#[derive(Copy, Clone)]
struct RGBA(u8, u8, u8, u8);

impl RGBA {

    fn from_argb_u32(n: u32) -> RGBA {
        RGBA(
            (0b1111_1111 & (n >> 16)) as u8,
            (0b1111_1111 & (n >> 8)) as u8,
            (0b1111_1111 & (n >> 0)) as u8,
            (0b1111_1111 & (n >> 24)) as u8
        )
    }
    fn to_argb_u32(&self) -> u32 {
        (self.3 as u32) << 24 | (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32)
    }
}

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

    fn check(&self, pos: Vec2isize) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    fn idx(pos: Vec2isize, canvas_size: Vec2isize) -> usize {
        min(max(pos.1 - 1, 0) * canvas_size.0 + pos.0, canvas_size.0 * canvas_size.1 - 1) as usize
    }

    fn idx_of(&self, pos: Vec2isize) -> usize {
        min(max(pos.1 - 1, 0) * self.width + pos.0, self.width * self.height - 1) as usize
    }

    fn set(&mut self, pos: Vec2isize, color: RGBA) {
        if !self.check(pos) { return; }
        self.buffer[Self::idx(pos, Vec2isize(self.width, self.height))] = color.to_argb_u32();
    }

    fn get(&self, pos: Vec2isize) -> RGBA {
        RGBA::from_argb_u32(self.buffer[self.idx_of(pos)])
    }

    fn set_fixed(&mut self, mut pos: Vec2isize, color: RGBA) {
        self.fix(&mut pos);
        self.buffer[(max(pos.1 - 1, 0) * self.width + pos.0) as usize] = color.to_argb_u32();
    }

    fn draw_line(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: RGBA) {
        self.fix(&mut start);
        self.fix(&mut end);

        let line = LineCoefficient::new_from_line(start, end);

        if line.k.is_infinite() {
            let y_start = min(start.1, end.1);
            let y_end = max(start.1, end.1);
            for y in y_start..=y_end {
                self.set(Vec2isize(start.0, y), color);
            }
            return;
        }

        let x_start = min(start.0, end.0);
        let x_end = max(start.0, end.0);

        for x in x_start..=x_end {
            let y = (line.k * x as f32 + line.b) as isize;
            self.set(Vec2isize(x, y), color);
        }
    }

    fn fill_rect(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: RGBA) {
        self.fix(&mut start);
        self.fix(&mut end);

        for y in min(start.1, end.1)..=max(start.1, end.1) {
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                self.set(Vec2isize(x, y), color);
            }
        }
    }

    fn fill_triangle(&mut self, mut lv: Vec2isize, mut mv: Vec2isize, mut hv: Vec2isize, color: RGBA) {
        if mv.1 < lv.1 { std::mem::swap(&mut mv, &mut lv); }
        if hv.1 < lv.1 { std::mem::swap(&mut hv, &mut lv); }
        if hv.1 < mv.1 { std::mem::swap(&mut hv, &mut mv); }

        let lh_coef = LineCoefficient::new_from_line(lv, hv);

        fn fill_half(this: &mut Canvas, low: Vec2isize, high: Vec2isize, lh_coef: LineCoefficient, color: RGBA) {

            let coef = LineCoefficient::new_from_line(high, low);
            let mut y: isize;
            let mut x_boundary: isize;

            for x in min(low.0, high.0)..=max(low.0, high.0) {
                y = (x as f32 * coef.k + coef.b) as isize;
                x_boundary = ((y as f32 - lh_coef.b) / lh_coef.k) as isize;

                for i in min(x, x_boundary)..=max(x, x_boundary) {
                    this.set(Vec2isize(i, y), color);
                }

            }
        }

        fill_half(self, lv, mv, lh_coef, color);
        fill_half(self, mv, hv, lh_coef, color);


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

    canvas.fill_rect(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(0, 0, 255, 0));
    canvas.draw_line(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(255, 255, 255, 0));
    canvas.fill_triangle(Vec2isize(0, 300), Vec2isize(300, 300), Vec2isize(300, 450), RGBA(255, 0, 0, 0));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&canvas.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}