use minifb::{MouseButton, MouseMode, Window};
use crate::base::{Vec2i, RGBA};
use crate::canvas::{Canvas, CBF};
use num_traits::{AsPrimitive, Num};

pub const DEFAULT_SIZE: Vec2i = Vec2i(100, 10);
pub const DEFAULT_SLIDER_COLOR: RGBA = RGBA::black();
pub const DEFAULT_SLIDER_THICKNESS: isize = 3;

pub const DEFAULT_OUTLINE_COLOR: RGBA = RGBA::black();
pub const DEFAULT_OUTLINE_THICKNESS: isize = 3;

pub const DEFAULT_BACKGROUND_COLOR: RGBA = RGBA::white();
pub const DEFAULT_FILLER_COLOR: RGBA = RGBA::new(105, 105, 105);

pub trait SliderShader {
    fn get_color(&self, size: Vec2i, shift: Vec2i, slider_offset: isize) -> RGBA;

}

pub struct DefaultShader {
    pub slider_color: RGBA,
    pub slider_thickness: isize,
    pub outline_color: RGBA,
    pub outline_thickness: isize,
    pub background_color: RGBA,
    pub filler_color: RGBA
}

impl SliderShader for DefaultShader {
    fn get_color(&self, size: Vec2i, shift: Vec2i, slider_offset: isize) -> RGBA {
        if (shift.0 - slider_offset).abs() <= self.slider_thickness {
            self.slider_color
        } else if
        (shift.0 - 0).abs() <= self.outline_thickness ||
            (shift.1 - 0).abs() <= self.outline_thickness ||
            (shift.0 - size.0).abs() <= self.outline_thickness ||
            (shift.1 - size.1).abs() <= self.outline_thickness {
            return self.outline_color
        } else if shift.0 < slider_offset{
            return self.filler_color
        } else {
            return self.background_color
        }
    }
}

impl Default for DefaultShader {
    fn default() -> Self {
        DefaultShader {
            slider_color: DEFAULT_SLIDER_COLOR,
            slider_thickness: DEFAULT_SLIDER_THICKNESS,
            outline_color: DEFAULT_OUTLINE_COLOR,
            outline_thickness: DEFAULT_OUTLINE_THICKNESS,
            background_color: DEFAULT_BACKGROUND_COLOR,
            filler_color: DEFAULT_FILLER_COLOR
        }
    }
}

pub struct Slider<T, S: SliderShader> {
    pub pos: Vec2i,
    pub size: Vec2i,
    pub lower_bound: T,
    pub upper_bound: T,
    pub shader: S,
    pub slider_offset: isize
}

impl<T: Num + AsPrimitive<f64>, S: SliderShader> Slider<T, S> {

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

    pub fn draw<R: CBF>(&self, canvas: &Canvas<R>) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                canvas.set(
                    Vec2i(self.pos.0 + x, self.pos.1 + y),
                    self.shader.get_color(self.size, Vec2i(x, y), self.slider_offset),
                );
            }
        }
    }

    pub fn calc_value(&self) -> T
    where f64: AsPrimitive<T>
    {
        (self.slider_offset as f64 / self.size.0 as f64 * (self.upper_bound - self.lower_bound).as_()).as_() + self.lower_bound
    }

}

pub struct ColorSlider {
    pub r: Slider<i32, DefaultShader>,
    pub g: Slider<i32, DefaultShader>,
    pub b: Slider<i32, DefaultShader>,
    pub a: Slider<i32, DefaultShader>,
}

impl ColorSlider {
    pub fn new(pos: Vec2i, size: Vec2i) -> Self {
        let r = Slider::new(
            pos,
            size,
            0,
            255,
            DefaultShader{
                filler_color: RGBA::r(255),
                ..DefaultShader::default()
            },
        );

        let g = Slider::new(
            Vec2i(pos.0, pos.1 + size.1 * 1),
            size,
            0,
            255,
            DefaultShader{
                filler_color: RGBA::g(255),
                ..DefaultShader::default()
            },
        );

        let b = Slider::new(
            Vec2i(pos.0, pos.1 + size.1 * 2),
            size,
            0,
            255,
            DefaultShader{
                filler_color: RGBA::b(255),
                ..DefaultShader::default()
            },
        );

        let a = Slider::new(
            Vec2i(pos.0, pos.1 + size.1 * 3),
            size,
            0,
            255,
            DefaultShader{
                ..DefaultShader::default()
            },
        );

        ColorSlider { r, g, b, a }

    }
    pub fn update(&mut self, window: &Window) {
        self.r.update(window);
        self.g.update(window);
        self.b.update(window);
        self.a.update(window);
    }

    pub fn draw<R: CBF>(&self, canvas: &Canvas<R>) {
        self.r.draw(canvas);
        self.g.draw(canvas);
        self.b.draw(canvas);
        self.a.draw(canvas);
    }

    pub fn calc_color(&self) -> RGBA {
        RGBA {
            r: self.r.calc_value() as u8,
            g: self.g.calc_value() as u8,
            b: self.b.calc_value() as u8,
            a: self.a.calc_value() as u8
        }
    }
}

impl Default for ColorSlider {
    fn default() -> Self {
        Self::new(Vec2i(0, 0), DEFAULT_SIZE)
    }
}