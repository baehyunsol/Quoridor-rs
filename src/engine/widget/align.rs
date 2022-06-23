use super::{TOP, BOTTOM, LEFT, RIGHT};

#[derive(Copy, Clone)]
pub enum Alignment {
    First,   // _W_W_W_________
    Last,    // _________W_W_W_
    Center,  // _____W_W_W_____
    Uniform  // ___W___W___W___
}

// horizontal_align + vertical_align_inline
pub fn row(x: f32, y: f32, w: f32, h: f32, widgets: &mut Vec<Box<&mut dyn Movable>>, alignment: Alignment, inline_alignment: Alignment, paddings: [f32; 4], gaps: f32) {
    horizontal_align(x, x + w, widgets, alignment, (paddings[LEFT], paddings[RIGHT]), gaps);
    vertical_align_inline(y, y + h, widgets, inline_alignment, (paddings[TOP], paddings[BOTTOM]));
}

// vertical_align + horizontal_align_inline
pub fn column(x: f32, y: f32, w: f32, h: f32, widgets: &mut Vec<Box<&mut dyn Movable>>, alignment: Alignment, inline_alignment: Alignment, paddings: [f32; 4], gaps: f32) {
    vertical_align(y, y + h, widgets, alignment, (paddings[TOP], paddings[BOTTOM]), gaps);
    horizontal_align_inline(x, x + w, widgets, inline_alignment, (paddings[LEFT], paddings[RIGHT]));
}

pub fn horizontal_align(x_from: f32, x_to: f32, widgets: &mut Vec<Box<&mut dyn Movable>>, alignment: Alignment, paddings: (f32, f32), gaps: f32) {

    let box_width = x_to - x_from;
    let width_sum = widgets.iter().map(|w| w.w()).sum::<f32>();

    let gaps = match alignment {
        Alignment::Uniform => (box_width - width_sum) / (widgets.len() as f32 + 1.0),
        _ => gaps
    };

    let mut curr_x = match alignment {
        Alignment::Uniform => x_from + gaps,
        Alignment::First => x_from + paddings.0,
        Alignment::Last => x_to - paddings.1 - width_sum - gaps * (widgets.len() as f32 - 1.0),
        Alignment::Center => x_from + (box_width - width_sum - gaps * (widgets.len() as f32 - 1.0)) / 2.0
    };

    for widget in widgets.iter_mut() {
        widget.set_x(curr_x);
        curr_x += gaps + widget.w();
    }

}

pub fn vertical_align(y_from: f32, y_to: f32, widgets: &mut Vec<Box<&mut dyn Movable>>, alignment: Alignment, paddings: (f32, f32), gaps: f32) {

    let box_height = y_to - y_from;
    let height_sum = widgets.iter().map(|w| w.h()).sum::<f32>();

    let gaps = match alignment {
        Alignment::Uniform => (box_height - height_sum) / (widgets.len() as f32 + 1.0),
        _ => gaps
    };

    let mut curr_y = match alignment {
        Alignment::Uniform => y_from + gaps,
        Alignment::First => y_from + paddings.0,
        Alignment::Last => y_to - paddings.1 - height_sum - gaps * (widgets.len() as f32 - 1.0),
        Alignment::Center => y_from + (box_height - height_sum - gaps * (widgets.len() as f32 - 1.0)) / 2.0
    };

    for widget in widgets.iter_mut() {
        widget.set_y(curr_y);
        curr_y += gaps + widget.h();
    }
}

pub fn horizontal_align_inline(x_from: f32, x_to: f32, widgets: &mut Vec<Box<&mut dyn Movable>>, alignment: Alignment, paddings: (f32, f32)) {

    let box_width = x_to - x_from;
    
    for widget in widgets.iter_mut() {

        match alignment {
            Alignment::First => {
                widget.set_x(x_from + paddings.0);
            },
            Alignment::Last => {
                widget.set_x(x_to - paddings.1 - widget.w());
            }
            Alignment::Center => {
                widget.set_x(x_from + (box_width - widget.w()) / 2.0);
            }
            Alignment::Uniform => {
                // don't know what to do
            }
        }

    }

}

pub fn vertical_align_inline(y_from: f32, y_to: f32, widgets: &mut Vec<Box<&mut dyn Movable>>, alignment: Alignment, paddings: (f32, f32)) {

    let box_height = y_to - y_from;
    
    for widget in widgets.iter_mut() {

        match alignment {
            Alignment::First => {
                widget.set_y(y_from + paddings.0);
            },
            Alignment::Last => {
                widget.set_y(y_to - paddings.1 - widget.h());
            }
            Alignment::Center => {
                widget.set_y(y_from + (box_height - widget.h()) / 2.0);
            }
            Alignment::Uniform => {
                // don't know what to do
            }
        }

    }

}

pub trait Movable {

    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn w(&self) -> f32;
    fn h(&self) -> f32;

    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
}