#![allow(dead_code)]

use super::color::Color as GColor;
use super::global::GLOBAL_ENV;
use macroquad::prelude::*;

#[derive(Clone)]
pub enum Graphic {
    Rect {
        x: f32, y: f32, w: f32, h: f32, thickness: f32, color: GColor
    },
    Circle {
        x: f32, y: f32, r: f32, thickness: f32, color: GColor
    },
    Line {
        x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: GColor
    },
    Triangle {
        x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, color: GColor
    },
    Text {
        font: usize, string: String, x: f32, y: f32, size: u16, color: GColor
    },
    Image {
        image_index: usize, x: f32, y: f32, color: GColor
    },
    RoundRect {
        x: f32, y: f32, w: f32, h: f32, radius: f32, thickness: f32, color: GColor
    },
    Polygon {
        center_x: f32, center_y: f32, points: Vec<(f32, f32)>, thickness: f32, color: GColor
    },
}


impl Graphic {
    pub fn new_rect(x: f32, y: f32, w: f32, h: f32, thickness: f32, color: GColor) -> Graphic {
        Graphic::Rect {x, y, w, h, thickness, color}
    }
    pub fn new_circle(x: f32, y: f32, r: f32, thickness: f32, color: GColor) -> Graphic {
        Graphic::Circle {x, y, r, thickness, color}
    }
    pub fn new_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: GColor) -> Graphic {
        Graphic::Line {x1, y1, x2, y2, thickness, color}
    }
    pub fn new_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, thickness: f32, color: GColor) -> Graphic {
        Graphic::Triangle {x1, y1, x2, y2, x3, y3, thickness, color}
    }
    pub fn new_text(x: f32, y: f32, font: usize, string: String, size: u16, color: GColor) -> Graphic {
        Graphic::Text {x, y, font, string, size, color}
    }
    pub fn new_image(x: f32, y: f32, image_index: usize, color: GColor) -> Graphic {
        Graphic::Image {x, y, image_index, color}
    }
    pub fn new_round_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, thickness: f32, color: GColor) -> Graphic {
        let radius = radius.min(w / 2.0).min(h / 2.0);

        Graphic::RoundRect {x, y, w, h, radius, thickness, color}
    }
    pub fn new_polygon(points: Vec<(f32, f32)>, thickness: f32, color: GColor) -> Graphic {

        if points.len() < 3 {
            unsafe { GLOBAL_ENV.raise_error("A polygon with less than three vertexes? No..."); }
            return Graphic::new_rect(0.0, 0.0, 0.0, 0.0, 0.0, color);  // dummy data
        }

        let center_x = (points[0].0 + points[1].0 + points[2].0) / 3.0;
        let center_y = (points[0].1 + points[1].1 + points[2].1) / 3.0;

        Graphic::Polygon {center_x, center_y, points, thickness, color}
    }
    pub fn new_ellipse(x: f32, y: f32, rx: f32, ry: f32, thickness: f32, color: GColor) -> Graphic {

        if rx == ry {
            Graphic::new_circle(x, y, rx, thickness, color)
        }

        else {

            let points = (0..18).map(
                |theta| (x + rx * (theta as f32 / 17.0 * 6.283).cos(), y + ry * (theta as f32 / 17.0 * 6.283).sin())
            ).collect();

            Graphic::Polygon {center_x: x, center_y: y, points, thickness, color}
        }
    }
}

pub fn render(graphics: Vec<Graphic>, textures: &Vec<Texture2D>, fonts: &Vec<Font>) {
    for graphic in graphics.into_iter() {
        match graphic {
            Graphic::Rect {x, y, w, h, thickness, color} => {
                if thickness > 0.0 {
                    draw_rectangle_lines(
                        x, y, w, h, thickness, Color::from_rgba(color.r, color.g, color.b, color.a)
                    );
                }

                else {
                    draw_rectangle(
                        x, y, w, h, Color::from_rgba(color.r, color.g, color.b, color.a)
                    );
                }
            },
            Graphic::Circle {x, y, r, thickness, color} => {
                let sides = if r < 60.0 { 18 } else if r < 180.0 { 24 } else { 30 };

                if thickness > 0.0 {
                    draw_poly_lines(
                        x, y, sides, r, 0.0, thickness, Color::from_rgba(color.r, color.g, color.b, color.a)
                    );
                }

                else {
                    draw_poly(
                        x, y, sides, r, 0.0, Color::from_rgba(color.r, color.g, color.b, color.a)
                    );
                }
            },
            Graphic::Line {x1, y1, x2, y2, thickness, color} => {
                draw_line(x1, y1, x2, y2, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
            },
            Graphic::Triangle {x1, y1, x2, y2, x3, y3, thickness, color} => {
                if thickness > 0.0 {
                    draw_triangle_lines(
                        Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), thickness, Color::from_rgba(color.r, color.g, color.b, color.a)
                    );
                }

                else {
                    draw_triangle(
                        Vec2::new(x1, y1), Vec2::new(x2, y2), Vec2::new(x3, y3), Color::from_rgba(color.r, color.g, color.b, color.a)
                    );
                }
            },
            Graphic::Text {x, y, size, string, font, color} => {
                #[cfg(feature = "profile")]
                if font >= fonts.len() {
                    unsafe { GLOBAL_ENV.raise_error("Uninitialized font used!"); }
                    break;
                }

                draw_text_ex(
                    &string, x, y, TextParams {
                        font_size: size,
                        color: Color::from_rgba(color.r, color.g, color.b, color.a),
                        font: Some(&fonts[font]),
                        ..Default::default()
                    }
                );
            },
            Graphic::Image {x, y, image_index, color} => {
                #[cfg(feature = "profile")]
                if image_index >= textures.len() {
                    unsafe { GLOBAL_ENV.raise_error("Uninitialized image used!"); }
                    break;
                }

                draw_texture(
                    &textures[image_index], x, y, Color::from_rgba(color.r, color.g, color.b, color.a),
                );
            },
            Graphic::RoundRect {x, y, w, h, radius, thickness, color} => {
                if thickness > 0.0 {
                    draw_line(x + 0.293 * radius, y + 0.293 * radius, x + 0.618 * radius, y + 0.077 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + 0.618 * radius, y + 0.077 * radius, x + radius, y, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + radius, y, x + w - radius, y, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - radius, y, x + w - 0.618 * radius, y + 0.077 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - 0.618 * radius, y + 0.077 * radius, x + w - 0.293 * radius, y + 0.293 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + 0.293 * radius, y + h - 0.293 * radius, x + 0.618 * radius, y + h - 0.077 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + 0.618 * radius, y + h - 0.077 * radius, x + radius, y + h, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + radius, y + h, x + w - radius, y + h, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - radius, y + h, x + w - 0.618 * radius, y + h - 0.077 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - 0.618 * radius, y + h - 0.077 * radius, x + w - 0.293 * radius, y + h - 0.293 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + 0.293 * radius, y + 0.293 * radius, x + 0.077 * radius, y + 0.618 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + 0.077 * radius, y + 0.618 * radius, x, y + radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x, y + radius, x, y + h - radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x, y + h - radius, x + 0.077 * radius, y + h - 0.618 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + 0.077 * radius, y + h - 0.618 * radius, x + 0.293 * radius, y + h - 0.293 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - 0.293 * radius, y + 0.293 * radius, x + w - 0.077 * radius, y + 0.618 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - 0.077 * radius, y + 0.618 * radius, x + w, y + radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w, y + radius, x + w, y + h - radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w, y + h - radius, x + w - 0.077 * radius, y + h - 0.618 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_line(x + w - 0.077 * radius, y + h - 0.618 * radius, x + w - 0.293 * radius, y + h - 0.293 * radius, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                }

                else {
                    draw_rectangle(x + radius, y, w - radius * 2.0, h, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_rectangle(x, y + radius, radius, h - radius * 2.0, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_rectangle(x + w - radius, y + radius, radius, h - radius * 2.0, Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + radius), Vec2::new(x, y + radius), Vec2::new(x + 0.077 * radius, y + 0.618 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + radius), Vec2::new(x + 0.077 * radius, y + 0.618 * radius), Vec2::new(x + 0.293 * radius, y + 0.293 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + radius), Vec2::new(x + 0.293 * radius, y + 0.293 * radius), Vec2::new(x + 0.618 * radius, y + 0.077 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + radius), Vec2::new(x + 0.618 * radius, y + 0.077 * radius), Vec2::new(x + radius, y), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + radius), Vec2::new(x + w, y + radius), Vec2::new(x + w - 0.077 * radius, y + 0.618 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + radius), Vec2::new(x + w - 0.077 * radius, y + 0.618 * radius), Vec2::new(x + w - 0.293 * radius, y + 0.293 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + radius), Vec2::new(x + w - 0.293 * radius, y + 0.293 * radius), Vec2::new(x + w - 0.618 * radius, y + 0.077 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + radius), Vec2::new(x + w - 0.618 * radius, y + 0.077 * radius), Vec2::new(x + w - radius, y), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + h - radius), Vec2::new(x, y + h - radius), Vec2::new(x + 0.077 * radius, y + h - 0.618 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + h - radius), Vec2::new(x + 0.077 * radius, y + h - 0.618 * radius), Vec2::new(x + 0.293 * radius, y + h - 0.293 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + h - radius), Vec2::new(x + 0.293 * radius, y + h - 0.293 * radius), Vec2::new(x + 0.618 * radius, y + h - 0.077 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + radius, y + h - radius), Vec2::new(x + 0.618 * radius, y + h - 0.077 * radius), Vec2::new(x + radius, y + h), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + h - radius), Vec2::new(x + w, y + h - radius), Vec2::new(x + w - 0.077 * radius, y + h - 0.618 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + h - radius), Vec2::new(x + w - 0.077 * radius, y + h - 0.618 * radius), Vec2::new(x + w - 0.293 * radius, y + h - 0.293 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + h - radius), Vec2::new(x + w - 0.293 * radius, y + h - 0.293 * radius), Vec2::new(x + w - 0.618 * radius, y + h - 0.077 * radius), Color::from_rgba(color.r, color.g, color.b, color.a));
                    draw_triangle(Vec2::new(x + w - radius, y + h - radius), Vec2::new(x + w - 0.618 * radius, y + h - 0.077 * radius), Vec2::new(x + w - radius, y + h), Color::from_rgba(color.r, color.g, color.b, color.a));
                }
            },
            Graphic::Polygon {center_x, center_y, points, thickness, color} => {
                if thickness > 0.0 {
                    for i in 0..points.len() - 1 {
                        let (x1, y1) = points[i];
                        let (x2, y2) = points[i + 1];
                        draw_line(x1, y1, x2, y2, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                    }

                    let (x1, y1) = points[0];
                    let (x2, y2) = points[points.len() - 1];
                    draw_line(x1, y1, x2, y2, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                }

                else {
                    for i in 0..points.len() - 1 {
                        let (x1, y1) = points[i];
                        let (x2, y2) = points[i + 1];
                        draw_triangle(Vec2::new(center_x, center_y), Vec2::new(x1, y1), Vec2::new(x2, y2), Color::from_rgba(color.r, color.g, color.b, color.a));
                    }

                    let (x1, y1) = points[0];
                    let (x2, y2) = points[points.len() - 1];
                    draw_triangle(Vec2::new(center_x, center_y), Vec2::new(x1, y1), Vec2::new(x2, y2), Color::from_rgba(color.r, color.g, color.b, color.a));
                }
            },
        }
    }
}
