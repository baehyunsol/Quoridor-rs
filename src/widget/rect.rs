use super::scale::{Scalable, Scale};
use super::align::Movable;

// use this for layouts
#[derive(Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub w_scale: Scale,
    pub h_scale: Scale
}

impl Rect {

    pub fn dummy() -> Self {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect {
            x, y, w, h, w_scale: Scale::new_abs(w), h_scale: Scale::new_abs(h)
        }
    }

    pub fn set_w_scale(&mut self, w_scale: Scale) -> &mut Self {
        self.w_scale = w_scale;
        self
    }

    pub fn set_h_scale(&mut self, h_scale: Scale) -> &mut Self {
        self.h_scale = h_scale;
        self
    }

}

impl Scalable for Rect {

    #[inline]
    fn w_scale(&self) -> Scale {
        self.w_scale
    }

    #[inline]
    fn h_scale(&self) -> Scale {
        self.h_scale
    }

    #[inline]
    fn set_w(&mut self, w: f32) {
        self.w = w;
    }

    #[inline]
    fn set_h(&mut self, h: f32) {
        self.h = h;
    }

}

impl Movable for Rect {

    #[inline]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    fn w(&self) -> f32 {
        self.w
    }

    #[inline]
    fn h(&self) -> f32 {
        self.h
    }

    #[inline]
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    #[inline]
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}