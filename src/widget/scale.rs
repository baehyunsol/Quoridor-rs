#[derive(Copy, Clone)]
pub struct Scale {
    scale: ScaleValue,
    min: Option<ScaleValue>,
    max: Option<ScaleValue>,
}

#[derive(Copy, Clone)]
pub enum ScaleValue {
    Absolute(f32),
    Relative(f32),  // 0.0 ~ 1.0
}

impl Scale {
    pub fn new_abs(scale: f32) -> Self {
        Scale {
            scale: ScaleValue::Absolute(scale),
            min: None,
            max: None
        }
    }
    pub fn new_rel(scale: f32) -> Self {
        Scale {
            scale: ScaleValue::Relative(scale),
            min: None,
            max: None
        }
    }
    pub fn set_min_abs(&mut self, min: f32) -> &mut Self {
        self.min = Some(ScaleValue::Absolute(min));
        self
    }
    pub fn set_min_rel(&mut self, min: f32) -> &mut Self {
        self.min = Some(ScaleValue::Relative(min));
        self
    }
    pub fn set_max_abs(&mut self, max: f32) -> &mut Self {
        self.max = Some(ScaleValue::Absolute(max));
        self
    }
    pub fn set_max_rel(&mut self, max: f32) -> &mut Self {
        self.max = Some(ScaleValue::Relative(max));
        self
    }
}

pub fn rect_scale(container_width: f32, container_height: f32, widgets: &mut Vec<Box<&mut dyn Scalable>>) {

    for widget in widgets.iter_mut() {
        widget.set_w(calc_min_max_scale(container_width, widget.w_scale()));
        widget.set_h(calc_min_max_scale(container_height, widget.h_scale()));
    }

}

pub fn horizontal_scale(container_width: f32, widgets: &mut Vec<Box<&mut dyn Scalable>>) {

    for widget in widgets.iter_mut() {
        widget.set_w(calc_min_max_scale(container_width, widget.w_scale()));
    }

}

pub fn vertical_scale(container_height: f32, widgets: &mut Vec<Box<&mut dyn Scalable>>) {

    for widget in widgets.iter_mut() {
        widget.set_h(calc_min_max_scale(container_height, widget.h_scale()));
    }

}

pub fn get_max_width(container_width: f32, widgets: Vec<Box<&mut dyn Scalable>>) -> f32 {
    widgets.iter().map(
        |w| calc_min_max_scale(container_width, w.w_scale())
    ).reduce(f32::max).unwrap()
}

pub fn get_max_height(container_height: f32, widgets: Vec<Box<&mut dyn Scalable>>) -> f32 {
    widgets.iter().map(
        |w| calc_min_max_scale(container_height, w.h_scale())
    ).reduce(f32::max).unwrap()
}

pub fn get_width_sum(container_width: f32, widgets: Vec<Box<&mut dyn Scalable>>) -> f32 {
    widgets.iter().map(
        |w| calc_min_max_scale(container_width, w.w_scale())
    ).sum()
}

pub fn get_height_sum(container_height: f32, widgets: Vec<Box<&mut dyn Scalable>>) -> f32 {
    widgets.iter().map(
        |w| calc_min_max_scale(container_height, w.h_scale())
    ).sum()
}

fn calc_relative_scale(container_size: f32, scale_value: ScaleValue) -> f32 {
    match scale_value {
        ScaleValue::Absolute(s) => s,
        ScaleValue::Relative(r) => container_size * r
    }
}

fn calc_min_max_scale(container_size: f32, scale: Scale) -> f32 {
    let Scale {scale, min, max} = scale;

    let scale = calc_relative_scale(container_size, scale);

    let min = match min {
        None => 0.0,
        Some(s) => calc_relative_scale(container_size, s)
    };

    let max = match max {
        None => scale * 2.0,
        Some(s) => calc_relative_scale(container_size, s)
    };

    scale.min(max).max(min)
}

pub trait Scalable {

    fn w_scale(&self) -> Scale;
    fn h_scale(&self) -> Scale;
    fn set_w(&mut self, w: f32);
    fn set_h(&mut self, h: f32);
}