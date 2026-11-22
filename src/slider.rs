use minifb::{MouseButton, MouseMode, Window};
use std::time::Instant;
use crate::base::{Vec2i, RGBA};
use crate::canvas::{Canvas, CBF};
use num_traits::{AsPrimitive, Num};

pub const DEFAULT_SIZE: Vec2i = Vec2i(100, 10);
pub const DEFAULT_SLIDER_COLOR: RGBA = RGBA::black();
pub const DEFAULT_SLIDER_THICKNESS: isize = 2;

pub const DEFAULT_OUTLINE_COLOR: RGBA = RGBA::black();
pub const DEFAULT_OUTLINE_THICKNESS: isize = 3;

pub const DEFAULT_BACKGROUND_COLOR: RGBA = RGBA::white();
pub const DEFAULT_FILLER_COLOR: RGBA = RGBA::new(105, 105, 105);

const ARROW_SIZE: isize = 20;
const ARROW_GAP: isize = 4;
const SLIDER_GAP: isize = 8;
const RESET_BUTTON_SIZE: Vec2i = Vec2i(72, 24);
const RESET_BUTTON_TOP_GAP: isize = 38;
const ARROW_INITIAL_REPEAT_RATE: f64 = 4.0;
const ARROW_REPEAT_ACCELERATION: f64 = 24.0;
const ARROW_MAX_REPEAT_RATE: f64 = 48.0;

pub trait SliderShader {
    fn get_color(&self, size: Vec2i, shift: Vec2i, slider_offset: isize) -> RGBA;

}

pub struct DefaultShader {
    pub slider_color: RGBA,
    pub slider_thickness: isize,
    pub outline_color: RGBA,
    pub outline_thickness: isize,
    pub outline_bottom: bool,
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
            (self.outline_bottom && (shift.1 - size.1).abs() <= self.outline_thickness) {
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
            outline_bottom: true,
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
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
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
    pub name: String,
    pub r: Slider<i32, DefaultShader>,
    pub g: Slider<i32, DefaultShader>,
    pub b: Slider<i32, DefaultShader>,
    pub a: Slider<i32, DefaultShader>,
    arrow_hold: Option<ArrowHold>,
    reset_offsets: [isize; 4],
}

struct ArrowHold {
    channel: usize,
    direction: i32,
    started: Instant,
    applied_steps: u32,
}

impl ColorSlider {
    pub fn new(pos: Vec2i, size: Vec2i) -> Self {
        Self::with_name(pos, size, "COLOR")
    }

    pub fn with_name(pos: Vec2i, size: Vec2i, name: &str) -> Self {
        let r = Slider::new(
            pos,
            size,
            0,
            255,
            DefaultShader{
                filler_color: RGBA::r(255),
                outline_bottom: false,
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
                outline_bottom: false,
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
                outline_bottom: false,
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
        let mut a = a;
        a.slider_offset = size.0;

        ColorSlider {
            name: name.to_owned(),
            r,
            g,
            b,
            a,
            arrow_hold: None,
            reset_offsets: [0, 0, 0, size.0],
        }

    }

    pub fn update(&mut self, window: &Window) {
        self.r.update(window);
        self.g.update(window);
        self.b.update(window);
        self.a.update(window);

        if !window.get_mouse_down(MouseButton::Left) {
            self.arrow_hold = None;
            return;
        }

        let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Pass) else {
            self.arrow_hold = None;
            return;
        };
        let mouse = Vec2i(mouse_x as isize, mouse_y as isize);
        if point_in_rect(mouse, reset_button_position(&self.r), RESET_BUTTON_SIZE) {
            self.reset();
            return;
        }

        let channel_and_direction = [
            (&self.r, 0),
            (&self.g, 1),
            (&self.b, 2),
            (&self.a, 3),
        ]
        .into_iter()
        .find_map(|(slider, channel)| {
            arrow_button_at(slider, mouse).map(|direction| (channel, direction))
        });

        let Some((channel, direction)) = channel_and_direction else {
            self.arrow_hold = None;
            return;
        };

        let now = Instant::now();
        let hold = self.arrow_hold.get_or_insert_with(|| ArrowHold {
            channel,
            direction,
            started: now,
            applied_steps: 0,
        });
        if hold.channel != channel || hold.direction != direction {
            *hold = ArrowHold {
                channel,
                direction,
                started: now,
                applied_steps: 0,
            };
        }

        let elapsed = hold.started.elapsed().as_secs_f64();
        let ramp_duration =
            (ARROW_MAX_REPEAT_RATE - ARROW_INITIAL_REPEAT_RATE) / ARROW_REPEAT_ACCELERATION;
        let extra_steps = if elapsed <= ramp_duration {
            ARROW_INITIAL_REPEAT_RATE * elapsed
                + 0.5 * ARROW_REPEAT_ACCELERATION * elapsed * elapsed
        } else {
            ARROW_INITIAL_REPEAT_RATE * ramp_duration
                + 0.5 * ARROW_REPEAT_ACCELERATION * ramp_duration * ramp_duration
                + ARROW_MAX_REPEAT_RATE * (elapsed - ramp_duration)
        };
        let total_steps = 1 + extra_steps.floor() as u32;
        let step_delta = total_steps.saturating_sub(hold.applied_steps) as i32;
        hold.applied_steps = total_steps;

        if step_delta > 0 {
            let slider = match channel {
                0 => &mut self.r,
                1 => &mut self.g,
                2 => &mut self.b,
                _ => &mut self.a,
            };
            let value = (slider.calc_value() + direction * step_delta).clamp(0, 255);
            slider.slider_offset =
                ((value as f64 / 255.0) * slider.size.0 as f64).ceil() as isize;
        }
    }

    pub fn draw<R: CBF>(&self, canvas: &Canvas<R>) {
        draw_text(canvas, Vec2i(self.r.pos.0, self.r.pos.1 - 32), &self.name, RGBA::black());
        draw_color_channel(canvas, &self.r, "RED", RGBA::r(255));
        draw_color_channel(canvas, &self.g, "GREEN", RGBA::g(255));
        draw_color_channel(canvas, &self.b, "BLUE", RGBA::b(255));
        draw_color_channel(canvas, &self.a, "ALPHA", DEFAULT_FILLER_COLOR);

        let swatch_pos = Vec2i(
            self.r.pos.0 + self.name.chars().count() as isize * 8 + 12,
            self.r.pos.1 - 36,
        );
        let color = self.calc_color();
        canvas.fill_rect(
            swatch_pos,
            Vec2i(swatch_pos.0 + 19, swatch_pos.1 + 19),
            RGBA::new(color.r, color.g, color.b),
        );
        canvas.draw_rect(swatch_pos, Vec2i(19, 19), RGBA::black(), 1);
        draw_reset_button(canvas, reset_button_position(&self.r));
    }

    pub fn set_reset_to_current(&mut self) {
        self.reset_offsets = [
            self.r.slider_offset,
            self.g.slider_offset,
            self.b.slider_offset,
            self.a.slider_offset,
        ];
    }

    fn reset(&mut self) {
        self.r.slider_offset = self.reset_offsets[0];
        self.g.slider_offset = self.reset_offsets[1];
        self.b.slider_offset = self.reset_offsets[2];
        self.a.slider_offset = self.reset_offsets[3];
        self.arrow_hold = None;
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

fn reset_button_position(slider: &Slider<i32, DefaultShader>) -> Vec2i {
    Vec2i(
        slider.pos.0 + slider.size.0 - RESET_BUTTON_SIZE.0 - 8,
        slider.pos.1 - RESET_BUTTON_TOP_GAP,
    )
}

fn point_in_rect(point: Vec2i, pos: Vec2i, size: Vec2i) -> bool {
    point.0 >= pos.0
        && point.0 < pos.0 + size.0
        && point.1 >= pos.1
        && point.1 < pos.1 + size.1
}

fn draw_reset_button<R: CBF>(canvas: &Canvas<R>, pos: Vec2i) {
    canvas.fill_rect(
        pos,
        Vec2i(
            pos.0 + RESET_BUTTON_SIZE.0 - 1,
            pos.1 + RESET_BUTTON_SIZE.1 - 1,
        ),
        RGBA::new(220, 220, 220),
    );
    canvas.draw_rect(
        pos,
        Vec2i(RESET_BUTTON_SIZE.0 - 1, RESET_BUTTON_SIZE.1 - 1),
        RGBA::black(),
        1,
    );
    draw_text(canvas, Vec2i(pos.0 + 16, pos.1 + 8), "RESET", RGBA::black());
}

fn arrow_button_positions(slider: &Slider<i32, DefaultShader>) -> (Vec2i, Vec2i) {
    let left_x = slider.pos.0 - SLIDER_GAP - ARROW_SIZE * 2 - ARROW_GAP;
    let y = slider.pos.1 + (slider.size.1 - ARROW_SIZE) / 2;
    (
        Vec2i(left_x, y),
        Vec2i(left_x + ARROW_SIZE + ARROW_GAP, y),
    )
}

fn arrow_button_at(slider: &Slider<i32, DefaultShader>, mouse: Vec2i) -> Option<i32> {
    let (decrease, increase) = arrow_button_positions(slider);
    let clicked_button = |pos: Vec2i| {
        mouse.0 >= pos.0
            && mouse.0 < pos.0 + ARROW_SIZE
            && mouse.1 >= pos.1
            && mouse.1 < pos.1 + ARROW_SIZE
    };

    if clicked_button(decrease) {
        Some(-1)
    } else if clicked_button(increase) {
        Some(1)
    } else {
        None
    }
}

fn draw_color_channel<R: CBF>(
    canvas: &Canvas<R>,
    slider: &Slider<i32, DefaultShader>,
    title: &str,
    color: RGBA,
) {
    slider.draw(canvas);
    let (decrease, increase) = arrow_button_positions(slider);

    draw_arrow_button(canvas, decrease, color, false);
    draw_arrow_button(canvas, increase, color, true);

    draw_text(
        canvas,
        Vec2i(slider.pos.0 + 12, slider.pos.1 + 8),
        title,
        RGBA::black(),
    );

    let value = slider.calc_value().clamp(0, 255).to_string();
    draw_text(
        canvas,
        Vec2i(
            slider.pos.0 + slider.size.0 + SLIDER_GAP + 4,
            slider.pos.1 + 8,
        ),
        &value,
        RGBA::black(),
    );
}

fn draw_arrow_button<R: CBF>(canvas: &Canvas<R>, pos: Vec2i, color: RGBA, points_right: bool) {
    canvas.fill_rect(
        pos,
        Vec2i(
            pos.0 + ARROW_SIZE - 1,
            pos.1 + ARROW_SIZE - 1,
        ),
        color,
    );
    canvas.draw_rect(
        pos,
        Vec2i(ARROW_SIZE - 1, ARROW_SIZE - 1),
        RGBA::black(),
        1,
    );

    let center_x = pos.0 + ARROW_SIZE / 2;
    let center_y = pos.1 + ARROW_SIZE / 2;
    for row in -6isize..=6 {
        let half_width = 6 - row.abs();
        let start_x = center_x - 4;
        let end_x = start_x + half_width * 8 / 6;
        for x in start_x..=end_x {
            let x = if points_right {
                x
            } else {
                center_x + 4 - (x - start_x)
            };
            canvas.set(Vec2i(x, center_y + row), RGBA::black());
        }
    }
}

pub(crate) fn draw_text<R: CBF>(canvas: &Canvas<R>, pos: Vec2i, text: &str, color: RGBA) {
    for (index, character) in text.chars().enumerate() {
        let rows = match character {
            'A' => [0b010, 0b101, 0b111, 0b101, 0b101],
            'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
            'C' => [0b011, 0b100, 0b100, 0b100, 0b011],
            'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
            'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
            'F' => [0b111, 0b100, 0b110, 0b100, 0b100],
            'G' => [0b011, 0b100, 0b101, 0b101, 0b011],
            'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
            'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
            'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
            'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
            'N' => [0b101, 0b111, 0b111, 0b111, 0b101],
            'O' => [0b010, 0b101, 0b101, 0b101, 0b010],
            'P' => [0b110, 0b101, 0b110, 0b100, 0b100],
            'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
            'S' => [0b011, 0b100, 0b010, 0b001, 0b110],
            'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
            'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
            'W' => [0b101, 0b101, 0b111, 0b111, 0b101],
            '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
            '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
            '2' => [0b110, 0b001, 0b010, 0b100, 0b111],
            '3' => [0b110, 0b001, 0b010, 0b001, 0b110],
            '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
            '5' => [0b111, 0b100, 0b110, 0b001, 0b110],
            '6' => [0b011, 0b100, 0b110, 0b101, 0b010],
            '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
            '8' => [0b010, 0b101, 0b010, 0b101, 0b010],
            '9' => [0b010, 0b101, 0b011, 0b001, 0b110],
            _ => [0; 5],
        };
        let char_x = pos.0 + index as isize * 8;

        for (y, row) in rows.iter().enumerate() {
            for x in 0..3 {
                if row & (1 << (2 - x)) != 0 {
                    for pixel_y in 0..2 {
                        for pixel_x in 0..2 {
                            canvas.set(
                                Vec2i(
                                    char_x + x * 2 + pixel_x,
                                    pos.1 + y as isize * 2 + pixel_y,
                                ),
                                color,
                            );
                        }
                    }
                }
            }
        }
    }
}

impl Default for ColorSlider {
    fn default() -> Self {
        Self::new(Vec2i(0, 0), DEFAULT_SIZE)
    }
}