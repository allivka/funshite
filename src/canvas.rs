use std::cmp::{max, min};
use crate::base::{Vec2i, RGBA};

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
        let dy = -(end.1 - start.1).abs();

        let sx = if start.0 < end.0 { 1 } else { -1 };
        let sy = if start.1 < end.1 { 1 } else { -1 };

        let mut err = dx + dy;

        let mut x = start.0;
        let mut y = start.1;

        loop {
            self.set(Vec2i(x, y), color);

            if x == end.0 && y == end.1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x += sx;
            }

            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_line_thick(&mut self, start: Vec2i, end: Vec2i, color: RGBA, thickness: isize) {
        //TODO: make pixels to count for thickness actually perpendicular to the main line

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

        let color_u32 = color.to_argb_u32();

        let get_x = |y: isize, p1: Vec2i, p2: Vec2i| -> isize {
            let dy = p2.1 - p1.1;

            if dy == 0 {
                return p1.0;
            }

            p1.0 + (y - p1.1) * (p2.0 - p1.0) / dy
        };

        let mut fill_half = |low: Vec2i, high: Vec2i| {
            if low.1 == high.1 {
                return;
            }

            let y_start = max(low.1, 0);
            let y_end = min(high.1, self.height - 1);

            for y in y_start..=y_end {
                let x1 = get_x(y, lv, hv);
                let x2 = get_x(y, low, high);

                let start_x = max(min(x1, x2), 0);
                let end_x = min(max(x1, x2), self.width - 1);

                if start_x <= end_x {
                    let row = (y * self.width) as usize;
                    let start_idx = row + start_x as usize;
                    let end_idx = row + end_x as usize;
                    self.buffer[start_idx..=end_idx].fill(color_u32);
                }
            }
        };

        fill_half(lv, mv);
        fill_half(mv, hv);
    }

    pub fn fill_polygon(&mut self, points: &Vec<Vec2i>, color: RGBA) {
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