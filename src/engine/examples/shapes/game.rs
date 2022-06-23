use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::sound::SoundAction;
use crate::color::Color;

pub struct Game {
    // Game data
}


impl Game {

    pub fn new() -> Self {
        Game {}
    }

}


impl Context for Game {
    fn frame(mut self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {

        let red = Color::new(192, 64, 64, 255);

        let graphics = vec![
            Graphic::new_rect(90.0, 90.0, 90.0, 90.0, 0.0, red.clone()),
            Graphic::new_rect(270.0, 90.0, 90.0, 90.0, 4.0, red.clone()),
            Graphic::new_circle(495.0, 135.0, 45.0, 0.0, red.clone()),
            Graphic::new_circle(675.0, 135.0, 45.0, 4.0, red.clone()),
            Graphic::new_triangle(90.0, 360.0, 180.0, 360.0, 135.0, 270.0, 0.0, red.clone()),
            Graphic::new_triangle(270.0, 360.0, 360.0, 360.0, 315.0, 270.0, 4.0, red.clone()),
            Graphic::new_round_rect(450.0, 270.0, 90.0, 90.0, 12.0, 0.0, red.clone()),
            Graphic::new_round_rect(630.0, 270.0, 90.0, 90.0, 12.0, 4.0, red.clone()),
            Graphic::new_polygon(vec![(135.0, 450.0), (90.0, 495.0), (135.0, 540.0), (180.0, 495.0)], 0.0, red.clone()),
            Graphic::new_polygon(vec![(315.0, 450.0), (270.0, 495.0), (315.0, 540.0), (360.0, 495.0)], 4.0, red.clone()),
        ];

        (self, graphics, vec![])
    }
}
