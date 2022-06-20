#![allow(dead_code)]

use crate::color::Color;

#[derive(Clone)]
pub enum Graphic {
    Rect {
        x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color
    },
    Circle {
        x: f32, y: f32, r: f32, thickness: f32, color: Color
    },
    Line {
        x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color
    },
    Triangle {
        x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, color: Color
    },
    Text {
        font: usize, string: String, x: f32, y: f32, size: u16, color: Color
    },
    Image {
        image_index: usize, x: f32, y: f32, color: Color
    }
}


impl Graphic {
    pub fn new_rect(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) -> Graphic {
        Graphic::Rect {x, y, w, h, thickness, color}
    }
    pub fn new_circle(x: f32, y: f32, r: f32, thickness: f32, color: Color) -> Graphic {
        Graphic::Circle {x, y, r, thickness, color}
    }
    pub fn new_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) -> Graphic {
        Graphic::Line {x1, y1, x2, y2, thickness, color}
    }
    pub fn new_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, color: Color) -> Graphic {
        Graphic::Triangle {x1, y1, x2, y2, x3, y3, thickness, color}
    }
    pub fn new_text(x: f32, y: f32, font: usize, string: String, size: u16, color: Color) -> Graphic {
        Graphic::Text {x, y, font, string, size, color}
    }
    pub fn new_image(x: f32, y: f32, image_index: usize, color: Color) -> Graphic {
        Graphic::Image {x, y, image_index, color}
    }
    pub fn move_rel(&self, dx: f32, dy: f32) -> Self {
        match self {
            Graphic::Rect {x, y, w, h, thickness, color} => Graphic::Rect {
                x: x + dx,
                y: y + dy,
                w: *w, h: *h, thickness: *thickness, color: color.clone()
            },
            Graphic::Circle {x, y, r, thickness, color} => Graphic::Circle {
                x: x + dx,
                y: y + dy,
                r: *r, thickness: *thickness, color: color.clone()
            },
            Graphic::Line {x1, y1, x2, y2, thickness, color} => Graphic::Line {
                x1: x1 + dx,
                y1: y1 + dy,
                x2: x2 + dx,
                y2: y2 + dy,
                thickness: *thickness, color: color.clone()
            },
            Graphic::Text {font, string, x, y, color, size} => Graphic::Text {
                x: x + dx,
                y: y + dy,
                size: *size,
                font: font.clone(), string: string.clone(), color: color.clone()
            },
            Graphic::Image {x, y, image_index, color} => Graphic::Image {
                x: x + dx,
                y: y + dy,
                image_index: *image_index,
                color: color.clone()
            },
            Graphic::Triangle {x1, y1, x2, y2, x3, y3, thickness, color} => Graphic::Triangle {
                x1: x1 + dx,
                y1: y1 + dy,
                x2: x2 + dx,
                y2: y2 + dy,
                x3: x3 + dx,
                y3: y3 + dy,
                thickness: *thickness, color: color.clone()
            }
        }
    }
}
