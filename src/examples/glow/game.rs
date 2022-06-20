use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::global::GLOBAL_ENV;
use crate::sound::SoundAction;
use crate::color::Color;
use crate::keys::*;

pub struct Game {
    div: usize,
    offset: f32
}


impl Game {

    pub fn new() -> Self {
        unsafe { GLOBAL_ENV.show_cursor(false); }
        Game {
            div: 2,
            offset: 0.0
        }
    }

}


impl Context for Game {
    fn frame(mut self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {

        if inputs.key_pressed[KEY_SPACE] {
            self.div += 1;
        }

        let (mouse_x, mouse_y) = inputs.mouse_pos;
        let (screen_width, screen_height) = unsafe { GLOBAL_ENV.screen_size.clone() };

        let x_grid_size = screen_width / self.div as f32;
        let y_grid_size = screen_height / self.div as f32;
        let mut graphics = Vec::with_capacity(self.div * self.div);
        self.offset += 0.15;

        for x in 0..self.div {

            for y in 0..self.div {
                let mouse_dist = ((x as f32 * x_grid_size - mouse_x).powi(2) + (y as f32 * y_grid_size - mouse_y).powi(2)).sqrt().max(1.0);
                graphics.push(
                    Graphic::new_rect(
                        x as f32 * x_grid_size,
                        y as f32 * y_grid_size,
                        x_grid_size,
                        y_grid_size,
                        0.0,
                        get_color(x as f32 * x_grid_size, y as f32 * y_grid_size, self.offset, 120.0 / mouse_dist)
                    )
                );
            }

        }

        (self, graphics, vec![])
    }
}

fn get_color(x: f32, y: f32, offset: f32, mouse_offset: f32) -> Color {
    let c1 = ((0.037 * x + 0.127 * y + 0.6 * offset + mouse_offset).cos() * 96.0 + 120.0) as u8;
    let c2 = ((0.053 * x - 0.053 * y + 0.7 * offset + mouse_offset).cos() * 96.0 + 120.0) as u8;
    let c3 = ((0.127 * x + 0.037 * y - 0.8 * offset + mouse_offset).cos() * 96.0 + 120.0) as u8;
    Color::new(c1, c2, c3, 255)
}
