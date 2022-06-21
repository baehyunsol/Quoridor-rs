use super::{Game, BOARD_SIZE};
use crate::graphic::Graphic;
use crate::color::Color;
use crate::widget::{textbox::TextBox, align::Alignment};

impl Game {

    pub fn draw_ui(&mut self, box_x: f32, box_y: f32) -> Vec<Graphic> {
        let mut result = Vec::with_capacity(1000);
        let timer = TextBox::new(
            &format!("{}", self.clock as usize),
            box_x + BOARD_SIZE / 2.0 - 105.0,
            box_y - 60.0,
            210.0,
            60.0,
            51.0
        ).set_color(Color::ui()).set_horizontal_align(Alignment::Center).set_vertical_align(Alignment::Center).render();

        for i in 0..self.player1.walls {
            result.push(Graphic::new_rect(box_x - 80.0, (i * 72) as f32 + box_y, 60.0, 18.0, 0.0, Color::wall()));
        }

        for i in 0..self.player2.walls {
            result.push(Graphic::new_rect(box_x + BOARD_SIZE + 20.0, (i * 72) as f32 + box_y, 60.0, 18.0, 0.0, Color::wall()));
        }

        if self.player1_turn {
            result.push(Graphic::new_triangle(box_x, box_y - 40.0, box_x + 90.0, box_y - 70.0, box_x + 90.0, box_y - 10.0, 0.0, Color::player1_normal()));
            result.push(Graphic::new_triangle(box_x + BOARD_SIZE - 20.0, box_y - 40.0, box_x + BOARD_SIZE - 60.0, box_y - 50.0, box_x + BOARD_SIZE - 60.0, box_y - 30.0, 0.0, Color::player2_normal()));
        }

        else {
            result.push(Graphic::new_triangle(box_x + 20.0, box_y - 40.0, box_x + 60.0, box_y - 50.0, box_x + 60.0, box_y - 30.0, 0.0, Color::player1_normal()));
            result.push(Graphic::new_triangle(box_x + BOARD_SIZE, box_y - 40.0, box_x + BOARD_SIZE - 90.0, box_y - 70.0, box_x + BOARD_SIZE - 90.0, box_y - 10.0, 0.0, Color::player2_normal()));
        }

        let mut buttons = vec![];

        for button in self.buttons.iter_mut() {
            buttons.push(button.render());
        }

        vec![result, timer, buttons.concat()].concat()
    }

    pub fn draw_board(&self, box_x: f32, box_y: f32) -> Vec<Graphic> {
        let mut board_graphics = Vec::with_capacity(100);

        board_graphics.push(Graphic::new_rect(box_x, box_y, BOARD_SIZE, BOARD_SIZE, 0.0, Color::board_normal()));

        for x in 0..9 {

            for y in 0..9 {
                board_graphics.push(
                    Graphic::new_rect(box_x + (x * 72) as f32 + 18.0, box_y + (y * 72) as f32 + 18.0, 54.0, 54.0, 0.0, Color::box_normal())
                );
            }

        }

        for (y, row) in self.horizontal_walls.iter().enumerate() {

            for (x, wall) in row.iter().enumerate() {

                if *wall {
                    board_graphics.push(
                        Graphic::new_rect(box_x + (x * 72) as f32 + 24.0, box_y + (y * 72) as f32 + 5.0, 42.0, 8.0, 0.0, Color::wall())
                    );
                }

            }

        }

        for (x, column) in self.vertical_walls.iter().enumerate() {

            for (y, wall) in column.iter().enumerate() {

                if *wall {
                    board_graphics.push(
                        Graphic::new_rect(box_x + (x * 72) as f32 + 5.0, box_y + (y * 72) as f32 + 24.0, 8.0, 42.0, 0.0, Color::wall())
                    );
                }

            }

        }

        for (x, row) in self.cross_walls.iter().enumerate() {

            for (y, wall) in row.iter().enumerate() {

                if *wall {
                    board_graphics.push(
                        Graphic::new_rect(box_x + (x * 72) as f32 + 5.0, box_y + (y * 72) as f32 + 5.0, 8.0, 8.0, 0.0, Color::wall())
                    );
                }

            }

        }

        board_graphics
    }

    pub fn draw_player(&self, box_x: f32, box_y: f32) -> Vec<Graphic> {
        let mut result = vec![
            Graphic::new_circle(
                box_x + (self.player1.position.0 * 72) as f32 + 45.0,
                box_y + (self.player1.position.1 * 72) as f32 + 45.0,
                16.0, 0.0,
                Color::player1_normal()
            ),
            Graphic::new_circle(
                box_x + (self.player2.position.0 * 72) as f32 + 45.0,
                box_y + (self.player2.position.1 * 72) as f32 + 45.0,
                16.0, 0.0,
                Color::player2_normal()
            ),
        ];

        let curr_color = if self.player1_turn {
            Color::player1_trans()
        } else {
            Color::player2_trans()
        };

        for (x, y) in self.get_valid_moves() {
            result.push(
                Graphic::new_circle(
                    box_x + (x * 72) as f32 + 45.0,
                    box_y + (y * 72) as f32 + 45.0,
                    9.0, 0.0,
                    curr_color.clone()
                ),
            );
        }

        result
    }

}