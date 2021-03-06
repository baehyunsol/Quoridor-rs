#![allow(dead_code)]

use super::graphic::Graphic;
use super::global::GLOBAL_ENV;

impl Graphic {

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
            },
            Graphic::RoundRect {x, y, w, h, radius, thickness, color} => Graphic::RoundRect {
                x: x + dx,
                y: y + dy,
                w: *w, h: *h, radius: *radius, thickness: *thickness, color: color.clone()
            },
            Graphic::Polygon {center_x, center_y, points, thickness, color} => Graphic::Polygon {
                center_x: center_x + dx,
                center_y: center_y + dy,
                points: points.iter().map(|(x, y)| (x + dx, y + dy)).collect(),
                thickness: *thickness, color: color.clone()
            }
        }
    }

    pub fn scale(&self, ref_x: f32, ref_y: f32, zoom_x: f32, zoom_y: f32) -> Self {

        #[cfg(feature = "profile")]
        if zoom_x <= 0.0 || zoom_y <= 0.0 {
            unsafe {GLOBAL_ENV.raise_error("scaling factor has to be positive!")}
            return self.clone();
        }

        match self {
            Graphic::Rect {x, y, w, h, thickness, color} => Graphic::Rect {
                x: scale_scalar(ref_x, *x, zoom_x),
                y: scale_scalar(ref_y, *y, zoom_y),
                w: w * zoom_x,
                h: h * zoom_y,
                thickness: thickness * zoom_x,
                color: color.clone()
            },
            Graphic::Circle {x, y, r, thickness, color} => Graphic::new_ellipse(
                scale_scalar(ref_x, *x, zoom_x),
                scale_scalar(ref_y, *y, zoom_y),
                r * zoom_x,
                r * zoom_y,
                thickness * zoom_x,
                color.clone()
            ),
            Graphic::Line {x1, y1, x2, y2, thickness, color} => Graphic::Line {
                x1: scale_scalar(ref_x, *x1, zoom_x),
                y1: scale_scalar(ref_y, *y1, zoom_y),
                x2: scale_scalar(ref_x, *x2, zoom_x),
                y2: scale_scalar(ref_y, *y2, zoom_y),
                thickness: thickness * zoom_x,
                color: color.clone()
            },
            Graphic::Text {x, y, size, font, string, color} => Graphic::Text {
                x: scale_scalar(ref_x, *x, zoom_x),
                y: scale_scalar(ref_y, *y, zoom_y),
                size: (*size as f32 * zoom_x) as u16,
                font: *font,
                string: string.clone(),
                color: color.clone()
            },
            Graphic::Image {x, y, image_index, color} => {
                unsafe {GLOBAL_ENV.raise_error("scaling an image is not implemented yet!");}

                Graphic::Image {
                    x: scale_scalar(ref_x, *x, zoom_x),
                    y: scale_scalar(ref_y, *y, zoom_y),
                    image_index: *image_index,
                    color: color.clone()
                }
            },
            Graphic::Triangle {x1, y1, x2, y2, x3, y3, thickness, color} => Graphic::Triangle {
                x1: scale_scalar(ref_x, *x1, zoom_x),
                y1: scale_scalar(ref_y, *y1, zoom_y),
                x2: scale_scalar(ref_x, *x2, zoom_x),
                y2: scale_scalar(ref_y, *y2, zoom_y),
                x3: scale_scalar(ref_x, *x3, zoom_x),
                y3: scale_scalar(ref_y, *y3, zoom_y),
                thickness: thickness * zoom_x,
                color: color.clone()
            },
            Graphic::RoundRect {x, y, w, h, radius, thickness, color} => Graphic::RoundRect {
                x: scale_scalar(ref_x, *x, zoom_x),
                y: scale_scalar(ref_y, *y, zoom_y),
                w: w * zoom_x,
                h: h * zoom_y,
                radius: radius * zoom_x,
                thickness: thickness * zoom_x,
                color: color.clone()
            },
            Graphic::Polygon {center_x, center_y, points, thickness, color} => Graphic::Polygon {
                center_x: scale_scalar(ref_x, *center_x, zoom_x),
                center_y: scale_scalar(ref_y, *center_y, zoom_y),
                points: points.iter().map(
                    |(x, y)|
                    (scale_scalar(ref_x, *x, zoom_x), scale_scalar(ref_y, *y, zoom_y))
                ).collect(),
                thickness: thickness * zoom_x,
                color: color.clone()
            }
        }
    }

}

#[inline]
fn scale_scalar(origin: f32, target: f32, zoom: f32) -> f32 {
    origin + (target - origin) * zoom
}