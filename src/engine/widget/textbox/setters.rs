use crate::engine::widget::align::Alignment;
use crate::engine::color::Color;
use super::TextBox;

impl TextBox {

    pub fn set_horizontal_align(&mut self, horizontal_align: Alignment) -> &mut Self {
        self.horizontal_align = horizontal_align;
        self.is_rendered = false;
        self
    }

    pub fn set_vertical_align(&mut self, vertical_align: Alignment) -> &mut Self {
        self.vertical_align = vertical_align;
        self.is_rendered = false;
        self
    }

    pub fn set_underline(&mut self, underline: Option<Color>) -> &mut Self {
        self.underline = underline;
        self.is_rendered = false;
        self
    }

    pub fn set_background(&mut self, background: Option<Color>) -> &mut Self {
        self.background = background;
        self.is_rendered = false;
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self.is_rendered = false;
        self
    }

    pub fn set_font(&mut self, font: usize) -> &mut Self {
        self.font = font;
        self.is_rendered = false;
        self
    }

    pub fn set_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self.is_rendered = false;
        self
    }

    pub fn set_size(&mut self, w: f32, h: f32) -> &mut Self {
        self.w = w;
        self.h = h;
        self.is_rendered = false;
        self
    }

    pub fn set_string(&mut self, string: &str) -> &mut Self {
        self.string = string.to_string().encode_utf16().collect();
        self.is_rendered = false;
        self
    }

    pub fn align_center(&mut self) -> &mut Self {
        self.horizontal_align = Alignment::Center;
        self.vertical_align = Alignment::Center;
        self.is_rendered = false;
        self
    }

}