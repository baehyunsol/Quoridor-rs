use super::{
    base::Base,
    align::Alignment
};
use crate::engine::graphic::Graphic;
use crate::engine::color::Color;

pub struct Button {
    x: f32, y: f32,
    base: Base,
    graphics: Vec<Vec<Graphic>>,
    hover: usize
}

impl Button {

    pub fn new(x: f32, y: f32, message: &str) -> Self {
        let mut base = Base::new(0.0, 0.0, 120.0, 32.0, message)
            .set_font_color(Color::new(255, 255, 255, 255))
            .set_horizontal_align(Alignment::Center)
            .set_vertical_align(Alignment::Center)
            .set_outline_radius(8.0)
            .to_owned();

        let mut graphics = vec![];

        for i in 0..25 {
            base.set_background(Some(Color::new(i * 6, i * 6, i * 6, 255)));
            graphics.push(base.render());
        }

        Button {
            x, y,
            base, graphics, hover: 0
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

        self.graphics[self.hover].iter().map(|graphic| graphic.move_rel(self.x, self.y)).collect()
    }

}