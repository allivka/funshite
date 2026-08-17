use std::sync::atomic::AtomicU32;
use minifb::{MouseButton, MouseMode, Window};
use crate::base::{Vec2i, RGBA};
use crate::canvas::Canvas;

pub const DEFAULT_SLIDER_COLOR: RGBA = RGBA::black();
pub const DEFAULT_SLIDER_THICKNESS: isize = 3;

pub const DEFAULT_OUTLINE_COLOR: RGBA = RGBA::black();
pub const DEFAULT_OUTLINE_THICKNESS: isize = 3;

pub const DEFAULT_BACKGROUND_COLOR: RGBA = RGBA::white();


pub fn three_color_shader(size: Vec2i, shift: Vec2i, slider_offset: isize, slider_color: RGBA, slider_thickness: isize, outline_color: RGBA, outline_thickness: isize, background_color: RGBA) -> RGBA {
    if (shift.0 - slider_offset).abs() <= slider_thickness {
        slider_color
    } else if
        (shift.0 - 0).abs() <= outline_thickness ||
        (shift.1 - 0).abs() <= outline_thickness ||
        (shift.0 - size.0).abs() <= outline_thickness ||
        (shift.1 - size.1).abs() <= outline_thickness {
        return outline_color
    } else {
        return background_color
    }
}

pub fn black_white_shader(size: Vec2i, shift: Vec2i, slider_offset: isize, slider_thickness: isize, outline_thickness: isize) -> RGBA {
    three_color_shader(size, shift, slider_offset, RGBA::black(), slider_thickness, RGBA::black(), outline_thickness, RGBA::white())
}


pub trait SliderShader {
    fn get_color(&self, size: Vec2i, shift: Vec2i, slider_offset: isize) -> RGBA {
        three_color_shader(size, shift, slider_offset, DEFAULT_SLIDER_COLOR, DEFAULT_SLIDER_THICKNESS, DEFAULT_OUTLINE_COLOR, DEFAULT_OUTLINE_THICKNESS, DEFAULT_BACKGROUND_COLOR)
    }

    fn get_color_mut(&mut self, size: Vec2i, shift: Vec2i, slider_offset: isize) -> RGBA {
        three_color_shader(size, shift, slider_offset, DEFAULT_SLIDER_COLOR, DEFAULT_SLIDER_THICKNESS, DEFAULT_OUTLINE_COLOR, DEFAULT_OUTLINE_THICKNESS, DEFAULT_BACKGROUND_COLOR)
    }
}

pub struct DefaultSliderShader;
impl SliderShader for DefaultSliderShader {}

pub struct Slider<T, S: SliderShader> {
    pub pos: Vec2i,
    pub size: Vec2i,
    pub lower_bound: T,
    pub upper_bound: T,
    pub shader: S,
    slider_offset: isize
}

impl<T, S: SliderShader> Slider<T, S> {

    pub fn new(pos: Vec2i, size: Vec2i, lower_bound: T, upper_bound: T, shader: S) -> Slider<T, S> {
        Slider {
            pos,
            size,
            lower_bound,
            upper_bound,
            shader,
            slider_offset: 0
        }
    }

    pub fn update(&mut self, window: &Window) {

        if !window.get_mouse_down(MouseButton::Left) {
            return;
        }

        let pos = window.get_mouse_pos(MouseMode::Clamp);

        if let None = pos {
            return;
        }

        //stores (flag, x, y)
        let t = pos.map(|(x, y)| -> (bool, isize, isize) {
            let x = x as isize;
            let y = y as isize;

            (x > self.pos.0 && y > self.pos.1 && x < self.pos.0 + self.size.0 && y < self.pos.1 + self.size.1, x, y)
        }).unwrap_or_default();

        if !t.0 {
            return;
        };

        self.slider_offset = t.1 - self.pos.0;

    }

    pub fn draw(&self, canvas: &mut Canvas<AtomicU32>) {
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                canvas.set(
                    Vec2i(self.pos.0 + x, self.pos.1 + y),
                    self.shader.get_color(self.size, Vec2i(x, y), self.slider_offset),
                );
            }
        }
    }

}