use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::sound::SoundAction;
use crate::color::Color;
use crate::keys::*;
use std::time;

enum GameState {
    Rects,
    Circles,
    Texts,
    Images
}

impl GameState {

    fn print_result(&self, score: usize) {

        match self {
            GameState::Rects => {
                println!("Drawing rectangles: scored {}", score);
            }
            GameState::Circles => {
                println!("Drawing circles: scored {}", score);
            }
            GameState::Texts => {
                println!("Drawing texts: scored {}", score);
            }
            GameState::Images => {
                println!("Drawing images: scored {}", score);
            }
        }

    }

    fn iter(&mut self) {
        match self {
            GameState::Rects => {
                *self = GameState::Circles;
            }
            GameState::Circles => {
                *self = GameState::Texts;
            }
            GameState::Texts => {
                *self = GameState::Images;
            }
            GameState::Images => {
                *self = GameState::Rects;
            }
        }
    }

}

pub struct Game {
    state: GameState,
    draw_count: usize,
    frame_count: usize,
    frame_start: time::Instant,
    halt: bool
}

impl Game {

    pub fn new() -> Self {
        Game {
            state: GameState::Rects,
            draw_count: 16,
            frame_count: 0,
            frame_start: time::Instant::now(),
            halt: false
        }
    }

}

impl Context for Game {
    fn frame(mut self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {

        if self.frame_count == 1 {
            self.frame_start = time::Instant::now();
        }

        let mut graphics = Vec::with_capacity(self.draw_count);

        if self.frame_count == 72 || self.halt {

            if time::Instant::now().duration_since(self.frame_start.clone()).as_millis() > 3000 || self.halt {

                // if the program fails at the first iteration, that's probably due to another issue other than this benchmark
                if self.draw_count > 16 || self.halt {
                    self.state.print_result(self.draw_count);
                    self.state.iter();
                }

                self.frame_count = 0;
                self.draw_count = 16;
                self.halt = false;

                return (self, vec![], vec![]);
            }

            else {
                self.draw_count /= 4;
                self.draw_count *= 5;
                self.frame_count = 0;
            }

        }

        self.halt = false;
        self.frame_count += 1;

        if inputs.key_pressed[KEY_SPACE] {
            self.halt = true;
        }

        match self.state {
            GameState::Rects => {

                for i in 0..self.draw_count {
                    let seed = (self.frame_count + i * 30) * 8;
                    let (x, y, w, h, r, g, b, a) = (
                        (random(seed) % 720) as f32,
                        (random(seed + 1) % 720) as f32,
                        (random(seed + 2) % 200) as f32,
                        (random(seed + 3) % 200) as f32,
                        (random(seed + 4) % 256) as u8,
                        (random(seed + 5) % 256) as u8,
                        (random(seed + 6) % 256) as u8,
                        (random(seed + 7) % 256) as u8,
                    );
                    graphics.push(Graphic::new_rect(x, y, w, h, -1.0, Color::new(r, g, b, a)));
                }

            }
            GameState::Circles => {

                for i in 0..self.draw_count {
                    let seed = (self.frame_count + i * 30) * 7;
                    let (x, y, rad, r, g, b, a) = (
                        (random(seed) % 720) as f32,
                        (random(seed + 1) % 720) as f32,
                        (random(seed + 2) % 200) as f32,
                        (random(seed + 3) % 256) as u8,
                        (random(seed + 4) % 256) as u8,
                        (random(seed + 5) % 256) as u8,
                        (random(seed + 6) % 256) as u8,
                    );
                    graphics.push(Graphic::new_circle(x, y, rad, -1.0, Color::new(r, g, b, a)));
                }

            }
            GameState::Texts => {

                for i in 0..self.draw_count {
                    let seed = (self.frame_count + i * 30) * 8;
                    let (x, y, text, size, r, g, b, a) = (
                        (random(seed) % 720) as f32,
                        (random(seed + 1) % 720) as f32,
                        (random(seed + 2) % 26 + 65) as u8,
                        (random(seed + 3) % 24 + 16) as u16,
                        (random(seed + 4) % 256) as u8,
                        (random(seed + 5) % 256) as u8,
                        (random(seed + 6) % 256) as u8,
                        (random(seed + 7) % 256) as u8,
                    );
                    graphics.push(Graphic::new_text(x, y, 0, String::from(text as char), size, Color::new(r, g, b, a)));
                }

            }
            GameState::Images => {

                for i in 0..self.draw_count {
                    let seed = (self.frame_count + i * 30) * 7;
                    let (x, y, index, r, g, b, a) = (
                        (random(seed) % 720) as f32,
                        (random(seed + 1) % 720) as f32,
                        (random(seed + 2) % 8) as usize,
                        (random(seed + 3) % 256) as u8,
                        (random(seed + 4) % 256) as u8,
                        (random(seed + 5) % 256) as u8,
                        (random(seed + 6) % 256) as u8,
                    );
                    graphics.push(Graphic::new_image(x, y, index, Color::new(r, g, b, a)));
                }

            }
        }

        // prevents photosensitive epilepsy
        graphics.push(Graphic::new_rect(0.0, 0.0, 960.0, 960.0, 0.0, Color::new(0, 0, 0, 192)));

        (self, graphics, vec![])
    }
}

fn random(mut seed: usize) -> u32 {

    if seed < 1024 {
        seed *= seed;
    }

    let mut seed = seed as u32;

    for _ in 0..3 {
        seed = ((seed % 21 + seed % 23 + seed % 25) * 821 + (seed % 27 + seed % 29 + seed % 31) * 823 + (seed % 33 + seed % 35 + seed % 37) * 827 + (seed % 39 + seed % 41 + seed % 43) * 829) % 65536;
    }

    seed
}
