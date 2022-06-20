use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::global::GLOBAL_ENV;
use crate::sound::SoundAction;
use crate::widget::{textbox::TextBox, align::Alignment};
use crate::color::Color;
use crate::keys::*;

pub struct Game {
    align_index: usize,
    box_width: f32,
    box_height: f32,
    font_size: f32,
    curr_underline: Option<Color>,
    curr_background: Option<Color>
}


impl Game {

    pub fn new() -> Self {
        Game {
            align_index: 0,
            box_width: 160.0,
            box_height: 160.0,
            font_size: 18.0,
            curr_underline: None,
            curr_background: None
        }
    }

}


impl Context for Game {
    fn frame(mut self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {
        
        if inputs.key_pressed[KEY_SPACE] {
            self.align_index = (self.align_index + 1) % 16;
        }

        if inputs.key_down[KEY_LEFT] {
            self.box_width -= 3.0;

            if self.box_width < 60.0 {
                self.box_width = 60.0;
            }

        }

        if inputs.key_down[KEY_RIGHT] {
            self.box_width += 3.0;

            if self.box_width > 600.0 {
                self.box_width = 600.0;
            }

        }

        if inputs.key_down[KEY_DOWN] {

            if inputs.key_down[KEY_LEFTSHIFT] || inputs.key_down[KEY_RIGHTSHIFT] {
                self.font_size -= 0.5;

                if self.font_size < 5.0 {
                    self.font_size = 5.0;
                }

            }

            else {
                self.box_height -= 3.0;

                if self.box_height < 60.0 {
                    self.box_height = 60.0;
                }

            }

        }

        if inputs.key_down[KEY_UP] {

            if inputs.key_down[KEY_LEFTSHIFT] || inputs.key_down[KEY_RIGHTSHIFT] {
                self.font_size += 0.5;

                if self.font_size > 50.0 {
                    self.font_size = 50.0;
                }

            }

            else {
                self.box_height += 3.0;

                if self.box_height > 600.0 {
                    self.box_height = 600.0;
                }

            }

        }

        if inputs.key_pressed[KEY_X] {

            match self.curr_underline {
                None => {
                    self.curr_underline = Some(Color::new(64, 64, 192, 255));
                }
                _ => {
                    self.curr_underline = None;
                }
            }

        }

        if inputs.key_pressed[KEY_C] {

            match self.curr_background {
                None => {
                    self.curr_background = Some(Color::new(192, 192, 64, 255));
                }
                _ => {
                    self.curr_background = None;
                }
            }

        }

        let mut graphics = vec![];
        let alignments = [
            Alignment::First,
            Alignment::Last,
            Alignment::Center,
            Alignment::Uniform,
        ];

        let (screen_w, screen_h) = unsafe { GLOBAL_ENV.screen_size.clone() };

        let x = (screen_w - self.box_width) / 2.0;
        let y = (screen_h - self.box_height) / 2.0;

        graphics.push(vec![
            Graphic::new_rect(0.0, 0.0, screen_w, screen_h, 0.0, Color::new(64, 192, 64, 255)),
            Graphic::new_rect(x, y, self.box_width, self.box_height, 0.0, Color::new(192, 64, 64, 255)),
        ]);
        graphics.push(TextBox::new(
            "This is a test text for blahblahblah\nNew Lines\nHaHaHaHa\nLet's see how far it can go...\nI need a sentence that is long enough to broken into multiple lines...\nUnfortunately, the previous line doesn't seem to be broken properly...\nMore texts...",
            x,
            y,
            self.box_width,
            self.box_height,
            self.font_size
        ).set_background(self.curr_background.clone()).set_underline(self.curr_underline.clone()).set_horizontal_align(alignments[self.align_index / 4]).set_vertical_align(alignments[self.align_index % 4]).render());

        (self, graphics.concat(), vec![])
    }
}
