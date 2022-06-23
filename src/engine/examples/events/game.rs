use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::sound::SoundAction;
use crate::global::GLOBAL_ENV;
use crate::color::Color;
use crate::widget::{
    base::Base, outline::Outline,
    scale::{Scale, rect_scale},
    align::{Alignment, column, row, Movable},
};
use std::collections::VecDeque;

#[derive(Default)]
pub struct Game {
    mouse_info: Base,
    mouse_event_queue_display: Base,
    mouse_event_queue: VecDeque<String>,
    key_down: Base,
    key_pressed: Base,
    misc1: Base,
    misc2: Base
}


impl Game {

    pub fn new() -> Self {
        let mut result = Game::default();

        result.key_down
            .set_horizontal_align(Alignment::Center)
            .set_vertical_align(Alignment::Uniform)
            .set_font_color(Color::new(255, 255, 255, 255))
            .set_paddings([16.0; 4])
            .set_background(Some(Color::new(128, 128, 128, 255)))
            .set_outlines(Some(Outline::new(Color::new(64, 64, 64, 255), 4.0)))
            .set_outline_radius(24.0)
            .set_w_scale(Some(Scale::new_rel(0.5)))
            .set_h_scale(Some(Scale::new_rel(0.8)));

        result.key_pressed = result.key_down.clone()
            .set_w_scale(Some(Scale::new_rel(0.4)))
            .to_owned();

        result.mouse_info = result.key_down.clone()
            .set_w_scale(Some(Scale::new_rel(0.3)))
            .to_owned();

        result.mouse_event_queue_display = result.key_down.clone()
            .set_w_scale(Some(Scale::new_rel(0.6)))
            .to_owned();

        result.misc1 = result.key_down.clone()
            .set_horizontal_align(Alignment::First)
            .set_vertical_align(Alignment::First)
            .set_w_scale(Some(Scale::new_rel(0.45)))
            .to_owned();

        result.misc2 = result.misc1.clone();

        result.locate_all();

        result
    }

    pub fn locate_all(&mut self) {
        let (screen_w, screen_h) = unsafe {GLOBAL_ENV.screen_size};

        let mut row1 = Base::default()
            .set_w_scale(Some(Scale::new_rel(0.9)))
            .set_h_scale(Some(Scale::new_rel(0.2)))
            .to_owned();

        let mut row2 = row1.clone()
            .set_h_scale(Some(Scale::new_rel(0.4)))
            .to_owned();

        let mut row3 = row1.clone()
            .set_h_scale(Some(Scale::new_rel(0.3)))
            .to_owned();

        rect_scale(screen_w, screen_h, &mut vec![Box::new(&mut row1), Box::new(&mut row2), Box::new(&mut row3)]);
        column(0.0, 0.0, screen_w, screen_h, &mut vec![Box::new(&mut row1), Box::new(&mut row2), Box::new(&mut row3)], Alignment::Uniform, Alignment::Center, [0.0;4], 0.0);

        rect_scale(row1.w(), row1.h(), &mut vec![Box::new(&mut self.key_down), Box::new(&mut self.key_pressed)]);
        row(row1.x(), row1.y(), row1.w(), row1.h(), &mut vec![Box::new(&mut self.key_down), Box::new(&mut self.key_pressed)], Alignment::Uniform, Alignment::Center, [0.0;4], 0.0);

        rect_scale(row2.w(), row2.h(), &mut vec![Box::new(&mut self.mouse_info), Box::new(&mut self.mouse_event_queue_display)]);
        row(row2.x(), row2.y(), row2.w(), row2.h(), &mut vec![Box::new(&mut self.mouse_info), Box::new(&mut self.mouse_event_queue_display)], Alignment::Uniform, Alignment::Center, [0.0;4], 0.0);

        rect_scale(row3.w(), row3.h(), &mut vec![Box::new(&mut self.misc1), Box::new(&mut self.misc2)]);
        row(row3.x(), row3.y(), row3.w(), row3.h(), &mut vec![Box::new(&mut self.misc1), Box::new(&mut self.misc2)], Alignment::Uniform, Alignment::Center, [0.0;4], 0.0);
    }

}


impl Context for Game {
    fn frame(mut self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {

        if inputs.is_screen_size_changed {
            self.locate_all();
        }

        let (mouse_x, mouse_y) = inputs.mouse_pos;
        let (mouse_x, mouse_y) = (mouse_x as i32, mouse_y as i32);

        let (screen_w, screen_h) = unsafe {GLOBAL_ENV.screen_size};

        while self.mouse_event_queue.len() > 4 {
            self.mouse_event_queue.pop_front();
        }

        for (i, m) in inputs.mouse_pressed.iter().enumerate() {

            if *m {
                self.mouse_event_queue.push_back(format!("Press {} {:?}", i, (mouse_x, mouse_y)));
            }

        }

        for (i, m) in inputs.mouse_released.iter().enumerate() {

            if *m {
                self.mouse_event_queue.push_back(format!("Release {} {:?}", i, (mouse_x, mouse_y)));
            }

        }

        self.key_down.set_text(
            format!("Key Down\n\n{:?}", rev_index(&inputs.key_down))
        );

        self.key_pressed.set_text(
            format!("Key Pressed\n\n{:?}", rev_index(&inputs.key_pressed))
        );

        self.mouse_info.set_text(
            format!(
                "Mouse Info\n\nPos: ({}, {})\nWheel: {}\nDown: {:?}",
                mouse_x, mouse_y, inputs.mouse_wheel, inputs.mouse_down.iter().map(|n| if *n {1} else {0}).collect::<Vec<usize>>()
            )
        );

        self.mouse_event_queue_display.set_text(
            format!(
                "Mouse Events\n\n{}",
                self.mouse_event_queue.iter().map(|s| s.clone()).collect::<Vec<String>>().join("\n")
            )
        );

        self.misc1.set_text(
            format!(
                "Screen: ({}, {})",
                screen_w as i32,
                screen_h as i32
            )
        );

        let mut graphics = vec![];

        graphics.push(self.key_down.render());
        graphics.push(self.key_pressed.render());
        graphics.push(self.mouse_info.render());
        graphics.push(self.mouse_event_queue_display.render());
        graphics.push(self.misc1.render());
        graphics.push(self.misc2.render());

        (self, graphics.concat(), vec![])
    }
}

fn rev_index(vec: &Vec<bool>) -> Vec<usize> {
    vec.iter().enumerate().filter(|(_, b)| **b).map(|(ind, _)| ind).collect()
}