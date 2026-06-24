use std::cmp::{max, min};

use crate::base::{Vec2isize, RGBA, LineCoefficients};

pub struct Canvas {
    pub width: isize,
    pub height: isize,
    pub buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width as isize,
            height: height as isize,
            buffer: vec![0; width * height],
        }
    }

    pub fn fixed(&self, mut pos: Vec2isize) -> Vec2isize {
        pos.0 = min(pos.0, self.width);
        pos.0 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.0 = max(pos.1, 0);

        pos
    }

    pub fn fix(&self, pos: &mut Vec2isize) {
        pos.0 = min(pos.0, self.width);
        pos.0 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.0 = max(pos.1, 0);
    }

    pub fn check(&self, pos: Vec2isize) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    pub fn idx(pos: Vec2isize, canvas_size: Vec2isize) -> usize {
        min(max(pos.1 - 1, 0) * canvas_size.0 + pos.0, canvas_size.0 * canvas_size.1 - 1) as usize
    }

    pub fn idx_of(&self, pos: Vec2isize) -> usize {
        min(max(pos.1 - 1, 0) * self.width + pos.0, self.width * self.height - 1) as usize
    }

    pub fn set(&mut self, pos: Vec2isize, color: RGBA) {
        if !self.check(pos) { return; }
        self.buffer[Self::idx(pos, Vec2isize(self.width, self.height))] = color.to_argb_u32();
    }

    pub fn get(&self, pos: Vec2isize) -> RGBA {
        RGBA::from_argb_u32(self.buffer[self.idx_of(pos)])
    }

    pub fn set_fixed(&mut self, mut pos: Vec2isize, color: RGBA) {
        self.fix(&mut pos);
        self.buffer[(max(pos.1 - 1, 0) * self.width + pos.0) as usize] = color.to_argb_u32();
    }

    pub fn draw_line(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: RGBA) {
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

    pub fn fill_rect(&mut self, mut start: Vec2isize, mut end: Vec2isize, color: RGBA) {
        self.fix(&mut start);
        self.fix(&mut end);

        for y in min(start.1, end.1)..=max(start.1, end.1) {
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                self.set(Vec2isize(x, y), color);
            }
        }
    }

    pub fn fill_triangle(&mut self, mut lv: Vec2isize, mut mv: Vec2isize, mut hv: Vec2isize, color: RGBA) {
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

    pub fn fill_polygon(&mut self, points: Vec<Vec2isize>, color: RGBA) {
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