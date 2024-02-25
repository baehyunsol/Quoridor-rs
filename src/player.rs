use crate::engine::graphic::Graphic;
use crate::engine::color::Color;

#[derive(Clone)]
pub struct Player {
    pub walls: usize,
    pub position: (i32, i32),
    color: Color,
    trace: Vec<(i32, i32)>,
}

impl Player {
    pub fn new(is_player1: bool) -> Self {
        let position = if is_player1 { (0, 4) } else { (8, 4) }; 
        Player {
            walls: 10,
            position,
            color: if is_player1 { Color::player1_trace() } else { Color::player2_trace() },
            trace: vec![position],
        }
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.position = (x, y);
        self.trace.push(self.position);
    }

    pub fn show_trace(&self, box_x: f32, box_y: f32) -> Vec<Graphic> {
        let mut result = Vec::with_capacity(self.trace.len() - 1);

        for i in 0..self.trace.len() - 1 {
            result.push(
                Graphic::new_line(
                    box_x + (self.trace[i].0 * 72) as f32 + 45.0,
                    box_y + (self.trace[i].1 * 72) as f32 + 45.0,
                    box_x + (self.trace[i + 1].0 * 72) as f32 + 45.0,
                    box_y + (self.trace[i + 1].1 * 72) as f32 + 45.0,
                    6.0,
                    self.color.clone(),
                )
            );
        }

        result
    }
}
