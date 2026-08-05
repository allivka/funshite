use std::cmp::{max, min};
use std::fmt;
use crate::base::{Vec2i, RGBA, LineCoefficients};

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
    
    pub fn clear_buffer(&mut self) {
        self.buffer = vec![0; self.width as usize * self.height as usize];
    }
    
    pub fn translate_centered_to_standard(&self, pos: Vec2i) -> Vec2i {
        Vec2i(
            pos.0 + self.width / 2,
            self.height - (pos.1 + self.height / 2),
        )
    }

    pub fn fixed(&self, mut pos: Vec2i) -> Vec2i {
        pos.0 = min(pos.0, self.width);
        pos.1 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.1 = max(pos.1, 0);

        pos
    }

    pub fn fix(&self, pos: &mut Vec2i) {
        pos.0 = min(pos.0, self.width);
        pos.1 = min(pos.1, self.height);
        pos.0 = max(pos.0, 0);
        pos.1 = max(pos.1, 0);
    }

    pub fn check(&self, pos: Vec2i) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    pub fn idx(pos: Vec2i, canvas_size: Vec2i) -> usize {
        min(max(pos.1, 0) * canvas_size.0 + max(pos.0, 0), canvas_size.0 * canvas_size.1 - 1) as usize
    }

    pub fn idx_of(&self, pos: Vec2i) -> usize {
        min(max(pos.1, 0) * self.width + max(pos.0, 0), self.width * self.height - 1) as usize
    }

    pub fn set(&mut self, pos: Vec2i, color: RGBA) {
        if !self.check(pos) { return; }
        self.buffer[Self::idx(pos, Vec2i(self.width, self.height))] = color.to_argb_u32();
    }

    pub fn get(&self, pos: Vec2i) -> RGBA {
        RGBA::from_argb_u32(self.buffer[self.idx_of(pos)])
    }

    pub fn set_fixed(&mut self, mut pos: Vec2i, color: RGBA) {
        self.fix(&mut pos);
        self.buffer[(max(pos.1, 0) * self.width + pos.0) as usize] = color.to_argb_u32();
    }

    pub fn draw_line(&mut self, start: Vec2i, end: Vec2i, color: RGBA) {


        let dx = (end.0 - start.0).abs();
        let dy = (end.1 - start.1).abs();

        let sx = if start.0 < end.0 { 1 } else { -1 };
        let sy = if start.1 < end.1 { 1 } else { -1 };

        let mut err = dy - dx;
        let mut e2: isize;

        let mut y: isize = 0;
        let mut x: isize = 0;

        loop {
            self.set(Vec2i(x + start.0, y + start.1), color);

            if x == dx * sx && y == dy * sy { break; }

            e2 = 2 * err;

            if e2 > dx {
                y += sy;
                err -= dx;
            }

            if e2 < dy {
                x += sx;
                err += dy;
            }

        }

    }

    pub fn draw_line_thick(&mut self, start: Vec2i, end: Vec2i, color: RGBA, thickness: isize) {

        if thickness < 1 {
            return;
        }

        let mut shift: isize = 0;

        for i in 1..=thickness {
            self.draw_line(
                Vec2i(start.0 + shift, start.1 + shift),
                Vec2i(end.0 + shift, end.1 + shift),
                color,
            );

            if i % 2 != 0 {
                shift = shift.abs() + 1
            } else {
                shift = -shift;
            }
        }
    }

    pub fn draw_rect(&mut self, pos: Vec2i, size: Vec2i, color: RGBA, thickness: isize) {
        self.draw_line_thick(pos, Vec2i(pos.0 + size.0, pos.1), color, thickness);
        self.draw_line_thick(pos, Vec2i(pos.0, pos.1 + size.1), color, thickness);
        self.draw_line_thick(Vec2i(pos.0 + size.0, pos.1), Vec2i(pos.0 + size.0, pos.1 + size.1), color, thickness);
        self.draw_line_thick( Vec2i(pos.0, pos.1 + size.1), Vec2i(pos.0 + size.0, pos.1 + size.1), color, thickness);
    }

    pub fn draw_polygon(&mut self, points: &Vec<Vec2i>,  color: RGBA, thickness: isize) {
        if points.is_empty() {
            return;
        }

        if points.len() == 1 {
            self.set(points[0], color);
            return;
        }

        match points.len() {
            0 => {
                return;
            },
            1 => {
                self.set(points[0], color);
                return;
            },
            2 => {
                self.draw_line_thick(points[0], points[1], color, thickness);
                return;
            }

            _ => ()
        }

        for i in 0..points.len() - 1 {
            self.draw_line_thick(points[i], points[i + 1], color, thickness);
        }
        
        self.draw_line_thick(points[0], points[points.len() - 1], color, thickness);

    }

    pub fn fill_rect(&mut self, mut start: Vec2i, mut end: Vec2i, color: RGBA) {
        self.fix(&mut start);
        self.fix(&mut end);

        for y in min(start.1, end.1)..=max(start.1, end.1) {
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                self.set(Vec2i(x, y), color);
            }
        }
    }

    pub fn fill_triangle(&mut self, mut lv: Vec2i, mut mv: Vec2i, mut hv: Vec2i, color: RGBA) {
        if mv.1 < lv.1 { std::mem::swap(&mut mv, &mut lv); }
        if hv.1 < lv.1 { std::mem::swap(&mut hv, &mut lv); }
        if hv.1 < mv.1 { std::mem::swap(&mut hv, &mut mv); }

        let lh_coef = LineCoefficients::new_from_line(lv, hv);
        let lm_coef = LineCoefficients::new_from_line(lv, mv);
        let mh_coef = LineCoefficients::new_from_line(mv, hv);

        let mut fill_half = |low: &Vec2i, high: &Vec2i, coef: &LineCoefficients | {
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
                    self.set(Vec2i(x, y), color);
                }
            }
        };

        fill_half(&lv, &mv, &lm_coef);
        fill_half(&mv, &hv, &mh_coef);

    }

    pub fn fill_polygon(&mut self, points: Vec<Vec2i>, color: RGBA) {
        match points.len() {
            0 => {
                return
            },
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