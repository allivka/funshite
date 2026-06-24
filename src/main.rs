use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::cmp::{min, max};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[derive(Copy, Clone)]
struct Vec2isize(isize, isize);

#[derive(Copy, Clone)]
struct LineCoefficients {
    k: f32,
    b: f32
}

impl LineCoefficients {
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
            start.1 as f32 - k * start.0 as f32  // y = kx + b → b = y - kx
        };

        LineCoefficients { k, b }
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

        let line = LineCoefficients::new_from_line(start, end);

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

        let lh_coef = LineCoefficients::new_from_line(lv, hv);
        let lm_coef = LineCoefficients::new_from_line(lv, mv);
        let mh_coef = LineCoefficients::new_from_line(mv, hv);

        let mut fill_half = |low: &Vec2isize, high: &Vec2isize, coef: &LineCoefficients | {
            let mut x1: f32;
            let mut x2: f32;

            for y in min(low.1, high.1)..=max(low.1, high.1) {
                x1 = if lh_coef.k.is_finite() {
                    ((y as f32) - lh_coef.b) / lh_coef.k
                } else {
                    lv.0 as f32
                };

                x2 = if coef.k.is_finite() {
                    ((y as f32) - coef.b) / coef.k
                } else {
                    low.0 as f32
                };

                for x in min(x1 as isize, x2 as isize)..=max(x1 as isize, x2 as isize) {
                    self.set(Vec2isize(x, y), color);
                }
            }
        };

        fill_half(&lv, &mv, &lm_coef);
        fill_half(&mv, &hv, &mh_coef);

    }

    fn fill_polygon(&mut self, points: Vec<Vec2isize>, color: RGBA) {
        match points.len() {
            1 => {
                self.set(points[0], color);
                return
            },
            2 => {
                self.draw_line(points[0], points[1], color);
                return
            },
            _ => ()
        }

        for i in 1..points.len() - 1 {
            self.fill_triangle(points[0], points[i], points[i + 1], color);
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

    while window.is_open() && !window.is_key_down(Key::Escape) {

        canvas.fill_rect(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(0, 0, 255, 0));
        canvas.draw_line(Vec2isize(0, 0), Vec2isize(200, 200), RGBA(255, 255, 255, 0));
        canvas.fill_triangle(Vec2isize(0, 300), Vec2isize(300, 300), Vec2isize(300, 450), RGBA(255, 0, 0, 0));

        canvas.fill_polygon(vec!{Vec2isize(0, 0), Vec2isize(200, 200)}, RGBA(255, 0, 255, 0));
        canvas.fill_polygon(vec!{Vec2isize(0, 300), Vec2isize(300, 300), Vec2isize(300, 450), Vec2isize(450, 1000)}, RGBA(255, 255, 0, 0));

        window.update_with_buffer(&canvas.buffer, WIDTH, HEIGHT).unwrap();
    }
}