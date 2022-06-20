use crate::game::Game;
use crate::graphic::Graphic;
use crate::color::Color;

impl Game {

    pub fn dfs(&self, from: (i32, i32), destination: i32, desired_direction: i32, visited: &mut Vec<Vec<bool>>) -> bool {

        if from.0 == -1 || from.1 == -1 || from.0 == 9 || from.1 == 9 {
            false
        }

        else if from.0 == destination {
            visited[from.0 as usize][from.1 as usize] = true;
            true
        }

        else {
            visited[from.0 as usize][from.1 as usize] = true;
            let dirs = vec![
                (desired_direction, 0),
                (-desired_direction, 0),
                (0, 1),
                (0, -1)
            ];
            let mut result = false;

            for dir in dirs.iter() {

                if self.is_movable_at(from, *dir) && from.0 + dir.0 >= 0 && from.1 + dir.1 >= 0 && !visited[(from.0 + dir.0) as usize][(from.1 + dir.1) as usize] {
                    result |= self.dfs((from.0 + dir.0, from.1 + dir.1), destination, desired_direction, visited);

                    if result {
                        break;
                    }

                }

            }

            result
        }

    }

    pub fn vis_dfs(&self, box_x: f32, box_y: f32) -> Vec<Graphic> {

        let mut visited = vec![vec![false; 10]; 10];

        self.dfs(self.player1.position, 8, 1, &mut visited);

        let mut result = Vec::with_capacity(100);

        for x in 0..10 {

            for y in 0..10 {

                if visited[x][y] {
                    result.push(Graphic::new_rect(
                        box_x + (x * 72) as f32 + 18.0,
                        box_y + (y * 72) as f32 + 18.0,
                        54.0, 54.0, 0.0,
                        Color::new(192, 64, 64, 128)
                    ));
                }

            }

        }

        result
    }

}