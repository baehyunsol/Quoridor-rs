use super::Base;
use crate::color::Color;
use crate::widget::{align::Alignment, scale::Scale, outline::Outline};

impl Base {

    pub fn set_font(&mut self, font: usize) -> &mut Self {
        self.font = font;
        self.is_rendered = false;
        self
    }

    pub fn set_font_size(&mut self, font_size: f32) -> &mut Self {
        self.font_size = font_size;
        self.is_rendered = false;
        self
    }

    pub fn set_font_color(&mut self, font_color: Color) -> &mut Self {
        self.font_color = font_color;
        self.is_rendered = false;
        self
    }

    pub fn set_font_underline(&mut self, font_underline: Option<Color>) -> &mut Self {
        self.font_underline = font_underline;
        self.is_rendered = false;
        self
    }

    pub fn set_w_scale(&mut self, w_scale: Option<Scale>) -> &mut Self {
        self.w_scale = w_scale;
        self.is_rendered = false;
        self
    }

    pub fn set_h_scale(&mut self, h_scale: Option<Scale>) -> &mut Self {
        self.h_scale = h_scale;
        self.is_rendered = false;
        self
    }

    pub fn set_vertical_align(&mut self, vertical_align: Alignment) -> &mut Self {
        self.vertical_align = vertical_align;
        self.is_rendered = false;
        self
    }

    pub fn set_horizontal_align(&mut self, horizontal_align: Alignment) -> &mut Self {
        self.horizontal_align = horizontal_align;
        self.is_rendered = false;
        self
    }

    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self.is_rendered = false;
        self
    }

    pub fn set_paddings(&mut self, paddings: [f32;4]) -> &mut Self {
        self.paddings = paddings;
        self.is_rendered = false;
        self
    }

    pub fn set_padding(&mut self, index: usize, padding: f32) -> &mut Self {
        // should it raise an error if the index exceeds 3?
        self.paddings[index] = padding;
        self.is_rendered = false;
        self
    }

    pub fn set_outlines_each(&mut self, outlines: [Option<Outline>;4]) -> &mut Self {
        self.outline = outlines;
        self.is_rendered = false;
        self
    }

    pub fn set_outlines(&mut self, outline: Option<Outline>) -> &mut Self {
        // I'm not gonna implement `Copy` for it
        // because I don't like implicit `Copy`s, especially for the `Color` struct
        self.outline = [outline.clone(), outline.clone(), outline.clone(), outline.clone()];
        self.is_rendered = false;
        self
    }

    pub fn set_outline(&mut self, index: usize, outline: Option<Outline>) -> &mut Self {
        // should it raise an error if the index exceeds 3?
        self.outline[index] = outline;
        self.is_rendered = false;
        self
    }

    pub fn set_outline_radius(&mut self, outline_radius: f32) -> &mut Self {
        self.outline_radius = outline_radius;
        self.is_rendered = false;
        self
    }

    pub fn set_text_outlines_each(&mut self, text_outlines: [Option<Outline>;4]) -> &mut Self {
        self.text_outline = text_outlines;
        self.is_rendered = false;
        self
    }

    pub fn set_text_outlines(&mut self, text_outline: Option<Outline>) -> &mut Self {
        // I'm not gonna implement `Copy` for it
        // because I don't like implicit `Copy`s, especially for the `Color` struct
        self.text_outline = [text_outline.clone(), text_outline.clone(), text_outline.clone(), text_outline.clone()];
        self.is_rendered = false;
        self
    }

    pub fn set_text_outline(&mut self, index: usize, text_outline: Option<Outline>) -> &mut Self {
        // should it raise an error if the index exceeds 3?
        self.text_outline[index] = text_outline;
        self.is_rendered = false;
        self
    }

    pub fn set_text_outline_radius(&mut self, text_outline_radius: f32) -> &mut Self {
        self.text_outline_radius = text_outline_radius;
        self.is_rendered = false;
        self
    }

    pub fn set_background(&mut self, background: Option<Color>) -> &mut Self {
        self.background = background;
        self.is_rendered = false;
        self
    }

    pub fn set_text_background(&mut self, text_background: Option<Color>) -> &mut Self {
        self.text_background = text_background;
        self.is_rendered = false;
        self
    }

}