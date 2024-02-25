use crate::engine::global::GLOBAL_ENV;
use crate::engine::graphic::Graphic;
use crate::engine::widget::{base::Base, outline::Outline, align::Alignment};
use crate::engine::color::Color;

pub struct Popup {
    graphics: Base,
    life: u8,
}

impl Popup {
    pub fn dummy() -> Self {
        Popup {
            graphics: Base::default(),
            life: 0,
        }
    }

    pub fn new(message: &str) -> Self {
        let w = (message.len() * 18).min(420).max(120) as f32;
        let h = 96.0;

        let (screen_w, screen_h) = unsafe {GLOBAL_ENV.screen_size};
        let x = (screen_w - w) / 2.0;
        let y = (screen_h - h) / 1.2;

        let graphics = Base::new(x, y, w, h, message)
            .set_font_color(Color::new(255, 255, 255, 255))
            .set_font_size(21.0)
            .set_background(Some(Color::new(128, 128, 128, 255)))
            .set_outlines(Some(Outline::new(Color::new(0, 0, 0, 255), 4.0)))
            .set_outline_radius(16.0)
            .set_horizontal_align(Alignment::Center)
            .set_vertical_align(Alignment::Center)
            .to_owned();

        Popup {
            graphics,
            life: 127,
        }
    }

    pub fn render(&mut self) -> Vec<Graphic> {
        if self.life > 10 {
            self.life -= 2;
            self.graphics
                .set_font_color(Color::new(255, 255, 255, self.life * 2))
                .set_background(Some(Color::new(128, 128, 128, self.life * 2)))
                .set_outlines(Some(Outline::new(Color::new(0, 0, 0, self.life * 2), 4.0)));
            self.graphics.render()
        }

        else {
            vec![]
        }
    }
}
