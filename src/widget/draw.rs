use crate::color::Color;
use crate::graphic::Graphic;
use super::{
    TOP, BOTTOM, LEFT, RIGHT,
    outline::Outline
};

pub fn draw_background(background: &Option<Color>, radius: f32, x: f32, y: f32, w: f32, h: f32, graphics: &mut Vec<Graphic>) {

    match background {
        Some(color) => {

            if radius < 2.0 {
                graphics.push(Graphic::new_rect(x, y, w, h, 0.0, color.clone()));
            }

            else {
                graphics.push(Graphic::new_round_rect(x, y, w, h, radius, 0.0, color.clone()));
            }

        }
        _ => {}
    }

}

pub fn draw_outline(outlines: &[Option<Outline>; 4], radius: f32, x: f32, y: f32, w: f32, h: f32, graphics: &mut Vec<Graphic>) {

    match &outlines[TOP] {
        Some(outline) => {

            if radius < 2.0 {
                graphics.push(Graphic::new_line(x, y, x + w, y, outline.width, outline.color.clone()));
            }

            else {
                graphics.push(Graphic::new_line(x + 0.293 * radius, y + 0.293 * radius, x + 0.618 * radius, y + 0.077 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + 0.618 * radius, y + 0.077 * radius, x + radius, y, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + radius, y, x + w - radius, y, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w - radius, y, x + w - 0.618 * radius, y + 0.077 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w - 0.618 * radius, y + 0.077 * radius, x + w - 0.293 * radius, y + 0.293 * radius, outline.width, outline.color.clone()));
            }

        }
        _ => {}
    }

    match &outlines[BOTTOM] {
        Some(outline) => {

            if radius < 2.0 {
                graphics.push(Graphic::new_line(x, y + h, x + w, y + h, outline.width, outline.color.clone()));
            }

            else {
                graphics.push(Graphic::new_line(x + 0.293 * radius, y + h - 0.293 * radius, x + 0.618 * radius, y + h - 0.077 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + 0.618 * radius, y + h - 0.077 * radius, x + radius, y + h, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + radius, y + h, x + w - radius, y + h, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w - radius, y + h, x + w - 0.618 * radius, y + h - 0.077 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w - 0.618 * radius, y + h - 0.077 * radius, x + w - 0.293 * radius, y + h - 0.293 * radius, outline.width, outline.color.clone()));
            }

        }
        _ => {}
    }

    match &outlines[LEFT] {
        Some(outline) => {

            if radius < 2.0 {
                graphics.push(Graphic::new_line(x, y, x, y + h, outline.width, outline.color.clone()));
            }

            else {
                graphics.push(Graphic::new_line(x + 0.293 * radius, y + 0.293 * radius, x + 0.077 * radius, y + 0.618 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + 0.077 * radius, y + 0.618 * radius, x, y + radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x, y + radius, x, y + h - radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x, y + h - radius, x + 0.077 * radius, y + h - 0.618 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + 0.077 * radius, y + h - 0.618 * radius, x + 0.293 * radius, y + h - 0.293 * radius, outline.width, outline.color.clone()));
            }

        }
        _ => {}
    }

    match &outlines[RIGHT] {
        Some(outline) => {

            if radius < 2.0 {
                graphics.push(Graphic::new_line(x + w, y, x + w, y + h, outline.width, outline.color.clone()));
            }

            else {
                graphics.push(Graphic::new_line(x + w - 0.293 * radius, y + 0.293 * radius, x + w - 0.077 * radius, y + 0.618 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w - 0.077 * radius, y + 0.618 * radius, x + w, y + radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w, y + radius, x + w, y + h - radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w, y + h - radius, x + w - 0.077 * radius, y + h - 0.618 * radius, outline.width, outline.color.clone()));
                graphics.push(Graphic::new_line(x + w - 0.077 * radius, y + h - 0.618 * radius, x + w - 0.293 * radius, y + h - 0.293 * radius, outline.width, outline.color.clone()));
            }

        }
        _ => {}
    }

}