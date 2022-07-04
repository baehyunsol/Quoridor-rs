use crate::engine::graphic::Graphic;
use crate::engine::color::Color;
use std::collections::HashMap;

pub struct MouseTraces {
    traces: HashMap<(i32, i32), MouseTrace>
}

impl MouseTraces {

    pub fn new() -> Self {
        MouseTraces {
            traces: HashMap::new()
        }
    }

    pub fn add(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.traces.insert((x as i32, y as i32), MouseTrace::new((x, y, w, h)));
    }

    pub fn render(&mut self) -> Vec<Graphic> {
        let result = self.traces.values_mut().map(|mt| mt.render()).collect();
        let mut deletions = Vec::with_capacity(self.traces.len());

        for (k, v) in self.traces.iter_mut() {

            if v.life == 0 {
                deletions.push(k.clone());
            }

        }

        for deletion in deletions.iter() {
            self.traces.remove(deletion);
        }

        result
    }

}

pub struct MouseTrace {
    rect: (f32, f32, f32, f32),
    pub life: u8
}

impl MouseTrace {

    pub fn new(rect: (f32, f32, f32, f32)) -> Self {
        MouseTrace {
            rect, life: 13
        }
    }

    pub fn render(&mut self) -> Graphic {

        if self.life > 0 {
            self.life -= 1;
        }

        Graphic::new_round_rect(
            self.rect.0,
            self.rect.1,
            self.rect.2,
            self.rect.3,
            12.0,
            0.0,
            Color::new(255, 255, 255, self.life * 12)
        )
    }

}