use super::{
    base::Base,
    align::Alignment,
    scale::Scalable,
    outline::Outline
};
use crate::engine::graphic::Graphic;
use crate::engine::color::Color;

pub struct Button {
    x: f32, y: f32,
    base: Base,
    graphics: Vec<Vec<Graphic>>,
    offset: Vec<(f32, f32)>,
    hover: usize,
}

impl Button {
    pub fn new(x: f32, y: f32, message: &str) -> Self {
        let mut base = Base::new(0.0, 0.0, 180.0, 32.0, message)
            .set_font_size(21.0)
            .set_font_color(Color::new(255, 255, 255, 255))
            .set_horizontal_align(Alignment::Center)
            .set_vertical_align(Alignment::Center)
            .set_outline_radius(8.0)
            .to_owned();

        let mut graphics = vec![];

        for i in 0..25 {
            base.set_background(Some(Color::new(i * 6, i * 6, i * 6, 255)));
            base.set_outlines(Some(Outline::new(Color::new(255, 255, 255, i * 4), 4.0)));
            base.set_w(180.0 + i as f32);
            base.set_h(32.0 + i as f32 / 4.0);
            graphics.push(base.render());
        }

        Button {
            x, y,
            base, graphics, hover: 0,
            offset: (0..25).map(|i| (12.0 - i as f32 / 2.0, 3.0 - i as f32 / 8.0)).collect(),
        }
    }

    pub fn check_mouse(&mut self, mouse: (f32, f32)) -> bool {
        let checked = self.base.is_mouse_on((mouse.0 - self.x, mouse.1 - self.y));

        if checked {
            self.hover = 25;
        }

        checked
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&mut self) -> Vec<Graphic> {
        if self.hover > 0 {
            self.hover -= 1;
        }

        self.graphics[self.hover].iter().map(
            |graphic| {
                let (offset_x, offset_y) = self.offset[self.hover];
                graphic.move_rel(
                    self.x + offset_x,
                    self.y + offset_y,
                )
            }
        ).collect()
    }
}
