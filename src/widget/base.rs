use crate::color::Color;
use crate::graphic::Graphic;
use super::{
    align::{Alignment, Movable}, scale::{Scale, Scalable},
    rect::Rect, textbox::TextBox, outline::{Outline, set_radius},
    draw::{draw_outline, draw_background},
    TOP, BOTTOM, LEFT, RIGHT
};
use std::default::Default;

mod setters;

#[derive(Clone)]
pub struct Base {
    x: f32, y: f32, w: f32, h: f32,
    font: usize, font_size: f32, font_color: Color, font_underline: Option<Color>,
    w_scale: Option<Scale>, h_scale: Option<Scale>,
    vertical_align: Alignment, horizontal_align: Alignment,
    text: String,
    paddings: [f32;4],
    outline: [Option<Outline>;4],
    outline_radius: f32,
    text_outline: [Option<Outline>;4],
    text_outline_radius: f32,
    background: Option<Color>,
    text_background: Option<Color>,
    rendered: Vec<Graphic>,
    is_rendered: bool
}

impl Base {

    pub fn new(x: f32, y: f32, w: f32, h: f32, text: &str) -> Self {
        Base {
            x, y, w, h, text: text.to_string(),
            ..Base::default()
        }
    }

    pub fn is_mouse_on(&self, mouse: (f32, f32)) -> bool {
        self.x <= mouse.0 && mouse.0 <= self.x + self.w && self.y <= mouse.1 && mouse.1 <= self.y + self.h
    }

    pub fn render(&mut self) -> Vec<Graphic> {

        if self.is_rendered {
            return self.rendered.clone();
        }

        self.rendered = vec![];
        let (text_x, text_y, text_w, text_h) = (self.x + self.paddings[LEFT], self.y + self.paddings[TOP], self.w - self.paddings[LEFT] - self.paddings[RIGHT], self.h - self.paddings[TOP] - self.paddings[BOTTOM]);
        let textbox_rendered = TextBox::new(&self.text, text_x, text_y, text_w, text_h, self.font_size)
                .set_vertical_align(self.vertical_align)
                .set_horizontal_align(self.horizontal_align)
                .set_color(self.font_color.clone())
                .set_font(self.font)
                .set_underline(self.font_underline.clone())
                .render();

        // outline radius cannot exceed the box's size
        self.text_outline_radius = self.text_outline_radius.min(text_w / 2.0).min(text_h / 2.0);
        self.outline_radius = self.outline_radius.min(self.w / 2.0).min(self.h / 2.0);

        for text_outline in self.text_outline.iter_mut() {
            set_radius(text_outline, self.text_outline_radius);
        }

        for outline in self.outline.iter_mut() {
            set_radius(outline, self.outline_radius);
        }

        draw_background(&self.background, self.outline_radius, self.x, self.y, self.w, self.h, &mut self.rendered);
        draw_background(&self.text_background, self.text_outline_radius, text_x, text_y, text_w, text_h, &mut self.rendered);

        draw_outline(&self.outline, self.outline_radius, self.x, self.y, self.w, self.h, &mut self.rendered);
        draw_outline(&self.text_outline, self.text_outline_radius, text_x, text_y, text_w, text_h, &mut self.rendered);

        self.rendered = vec![
            self.rendered.clone(),
            textbox_rendered
        ].concat();
        self.is_rendered = true;

        self.rendered.clone()
    }

    pub fn fit_to_rect(&mut self, rect: &Rect) {
        self.x = rect.x;
        self.y = rect.y;
        self.w = rect.w;
        self.h = rect.h;
    }

}

impl Default for Base {
    fn default() -> Self {
        Base {
            x: 0.0, y: 0.0, w: 0.0, h: 0.0,
            text: "".to_string(), font: 0, font_size: 18.0, font_color: Color::new(0, 0, 0, 255), font_underline: None,
            vertical_align: Alignment::First, horizontal_align: Alignment::First,
            w_scale: None, h_scale: None,
            paddings: [0.0; 4],
            outline: [None, None, None, None],
            outline_radius: 0.0,
            text_outline: [None, None, None, None],
            text_outline_radius: 0.0,
            background: None,
            text_background: None,
            rendered: vec![],
            is_rendered: false
        }
    }
}

impl Movable for Base {

    #[inline]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    fn w(&self) -> f32 {
        self.w
    }

    #[inline]
    fn h(&self) -> f32 {
        self.h
    }

    #[inline]
    fn set_x(&mut self, x: f32) {
        self.x = x;
        self.is_rendered = false;
    }

    #[inline]
    fn set_y(&mut self, y: f32) {
        self.y = y;
        self.is_rendered = false;
    }
}

impl Scalable for Base {

    fn w_scale(&self) -> Scale {
        match self.w_scale {
            None => Scale::new_abs(self.w),
            Some(s) => s
        }
    }

    fn h_scale(&self) -> Scale {
        match self.h_scale {
            None => Scale::new_abs(self.h),
            Some(s) => s
        }
    }

    fn set_w(&mut self, w: f32) {
        self.w = w;
        self.is_rendered = false;
    }

    fn set_h(&mut self, h: f32) {
        self.h = h;
        self.is_rendered = false;
    }

}