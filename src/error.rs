use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::sound::SoundAction;
use crate::global::GLOBAL_ENV;
use crate::widget::{textbox::TextBox, align::Alignment};
use crate::color::Color;

pub struct ErrorHandler {
    message: String
}


impl ErrorHandler {

    pub fn new(message: String) -> Self {
        ErrorHandler { message }
    }

}


impl Context for ErrorHandler {
    fn frame(self: Box<Self>, _: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {

        let graphics;

        unsafe {

            if !GLOBAL_ENV.is_font_available {
                panic!("{}", self.message);
            }

            let (screen_w, screen_h) = GLOBAL_ENV.screen_size;
            let header = TextBox::new(
                "FATAL ERROR",
                screen_w * 0.2,
                0.0,
                screen_w * 0.6,
                screen_h * 0.2,
                33.0
            ).set_vertical_align(Alignment::Center)
            .set_horizontal_align(Alignment::Center)
            .set_color(Color::new(255, 0, 0, 255))
            .render();
            let error_message = TextBox::new(
                &self.message,
                screen_w * 0.2,
                screen_h * 0.2,
                screen_w * 0.6,
                screen_h * 0.6,
                24.0
            ).set_vertical_align(Alignment::Center)
            .set_horizontal_align(Alignment::Center)
            .set_color(Color::new(255, 32, 32, 255))
            .render();

            graphics = vec![
                vec![
                    Graphic::new_rect(screen_w * 0.1, screen_h * 0.1, screen_w * 0.8, screen_h * 0.8, 0.0, Color::new(128, 128, 128, 255)),
                    Graphic::new_rect(screen_w * 0.2, 0.0, screen_w * 0.6, screen_h * 0.2, 0.0, Color::new(192, 192, 192, 255)),
                ],
                error_message,
                header
            ].concat();
        }

        (self, graphics, vec![])
    }
}