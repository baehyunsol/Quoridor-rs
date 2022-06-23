use crate::engine::color::Color;

#[derive(Clone)]
pub struct Outline {
    pub color: Color,
    pub width: f32,
    pub radius: f32
}

impl Outline {

    pub fn new(color: Color, width: f32) -> Self {
        Outline {
            color, width, radius: 0.0
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

}

pub fn set_radius(outline: &mut Option<Outline>, radius: f32) {

    match outline {
        Some(outline) => {
            outline.set_radius(radius);
        }
        _ => {}
    }

}