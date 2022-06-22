use crate::color::Color;
use crate::graphic::Graphic;
use super::align::Alignment;

mod setters;

// It only supports texts
// If you want something fancier, like outlines and background colors, merge it inside another widget
#[derive(Clone)]
pub struct TextBox {
    pub x: f32, pub y: f32, pub w: f32, pub h: f32, pub font_size: f32,
    pub font: usize, pub color: Color,
    pub string: Vec<u16>,
    pub horizontal_align: Alignment,
    pub vertical_align: Alignment,
    pub underline: Option<Color>,
    pub background: Option<Color>,
    rendered: Vec<Graphic>,
    is_rendered: bool
}

impl TextBox {
    pub fn new(string: &str, x: f32, y: f32, w: f32, h: f32, font_size: f32) -> Self {
        TextBox {
            string: string.to_string().encode_utf16().collect(),
            x, y, w, h, font_size: font_size.max(2.0),
            horizontal_align: Alignment::First,
            vertical_align: Alignment::First,
            font: 0,
            is_rendered: false,
            underline: None,
            background: None,
            color: Color::new(0, 0, 0, 255),
            rendered: vec![]
        }
    }

    pub fn render(&mut self) -> Vec<Graphic> {

        if self.is_rendered {
            return self.rendered.clone();
        }

        self.is_rendered = true;
        let lines: Vec<Vec<u16>> = self.string.split(|c| *c == '\n' as u16).map(|ln| ln.to_vec()).collect();
        let mut fitted_lines = Vec::with_capacity(lines.len() * 2);
        let char_per_line = ((self.w / (self.font_size * FONT_WIDTH_RATIO)) as usize).max(2) - 1;

        for i in 0..lines.len() {

            if lines[i].len() <= char_per_line {
                fitted_lines.push(lines[i].clone());
            }

            else {

                for line in break_line(&lines[i], char_per_line).into_iter() {
                    fitted_lines.push(line);
                }

            }

        }

        self.rendered = Vec::with_capacity(self.string.len());
        let font_size = self.font_size as u16;

        let vertical_gap = match self.vertical_align {
            Alignment::Uniform => (self.h - self.font_size * fitted_lines.len() as f32) / (fitted_lines.len() as f32 + 1.0) + self.font_size,
            _ => self.font_size * (1.0 + LINE_GAP),
        };

        let vertical_gap = vertical_gap.max(self.font_size);

        let mut curr_y = match self.vertical_align {
            Alignment::First => self.y + self.font_size,
            Alignment::Last => self.y + self.h - self.font_size * (1.0 + LINE_GAP) * fitted_lines.len() as f32 + self.font_size,
            Alignment::Center => self.y + (self.h - self.font_size * (1.0 + LINE_GAP) * fitted_lines.len() as f32) / 2.0 + self.font_size,
            Alignment::Uniform => self.y + vertical_gap / 2.0 + self.font_size
        };

        let font_width = self.font_size * FONT_WIDTH_RATIO;

        for line in fitted_lines.iter() {

            if curr_y < self.y {
                curr_y += vertical_gap;
                continue;
            }

            let horizontal_gap = match self.horizontal_align {
                Alignment::Uniform => (self.w - font_width * line.len() as f32) / (line.len() as f32 + 1.0) + font_width,
                _ => font_width
            };
            let mut curr_x = match self.horizontal_align {
                Alignment::First => self.x,
                Alignment::Last => self.x + self.w - font_width * line.len() as f32,
                Alignment::Center => self.x + (self.w - font_width * line.len() as f32) / 2.0,
                Alignment::Uniform => self.x + horizontal_gap / 2.0
            };
            let start_x = curr_x;

            match &self.background {
                Some(color) => {
                    if line.len() > 0 {
                        self.rendered.push(Graphic::new_rect(curr_x, curr_y - self.font_size, horizontal_gap * (line.len() as f32 - 1.0) + font_width, self.font_size, 0.0, color.clone()));
                    }
                }
                _ => {}
            }

            for character in line.iter() {

                if *character != 32 {
                    self.rendered.push(Graphic::new_text(curr_x, curr_y, self.font, String::from_utf16(&[*character]).unwrap(), font_size, self.color.clone()));
                }

                curr_x += horizontal_gap;
            }

            match &self.underline {
                Some(color) => {
                    self.rendered.push(Graphic::new_line(start_x, curr_y, curr_x, curr_y, self.font_size / 16.0 + 1.0, color.clone()));
                }
                _ => {}
            }

            curr_y += vertical_gap;

            if curr_y >= self.y + self.h {
                break;
            }

        }

        self.rendered.clone()
    }
}

fn break_line(long_line: &[u16], char_per_line: usize) -> Vec<Vec<u16>> {

    if long_line.len() <= char_per_line {
        return vec![long_line.to_vec()];
    }

    for index in 0..char_per_line / 3 {

        if long_line[char_per_line - index] == ' ' as u16 {
            return vec![
                vec![long_line[0..char_per_line - index + 1].to_vec()],
                break_line(&long_line[char_per_line - index + 1..long_line.len()], char_per_line)
            ].concat();
        }

    }

    return vec![
        vec![long_line[0..char_per_line + 1].to_vec()],
        break_line(&long_line[char_per_line + 1..long_line.len()], char_per_line)
    ].concat();
}

// It only supports monospace fonts
const FONT_WIDTH_RATIO: f32 = 0.55;
const LINE_GAP: f32 = 0.6;
