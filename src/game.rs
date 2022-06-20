mod save_data;
mod graphic;

use crate::context::Context;
use crate::inputs::Inputs;
use crate::graphic::Graphic;
use crate::global::GLOBAL_ENV;
use crate::widget::{
    textbox::TextBox,
    align::Alignment,
    button::Button
};
use crate::sound::SoundAction;
use crate::color::Color;
use crate::player::Player;
use crate::popup::Popup;
use save_data::GameSaveData;
use std::time;

pub struct Game {
    state: GameState,
    pub player1: Player,
    pub player2: Player,
    last_clock_tick: time::Instant,
    pub clock: f32,
    pub vertical_walls: Vec<Vec<bool>>,
    pub horizontal_walls: Vec<Vec<bool>>,
    pub cross_walls: Vec<Vec<bool>>,
    curr_popup: Popup,
    pub player1_turn: bool,
    last_turn_data: GameSaveData,
    last_state: GameState,  // state to transit from `ScreenTooSmall`
    buttons: Vec<Button>
}

#[derive(Copy, Clone, PartialEq)]
enum GameState {
    ScreenTooSmall,
    Playing,
    GameOver
}

impl Game {

    pub fn new() -> Self {

        let restart_button = Button::new(0.0, 0.0, "Restart");
        let undo_button = Button::new(0.0, 0.0, "Undo");
        let quit_button = Button::new(0.0, 0.0, "Quit");

        Game {
            state: GameState::Playing,
            player1: Player::new(true),
            player2: Player::new(false),
            last_clock_tick: time::Instant::now(),
            vertical_walls: vec![vec![false; 9]; 10],
            horizontal_walls: vec![vec![false; 9]; 10],
            cross_walls: vec![vec![false; 10]; 10],
            clock: 0.0,
            curr_popup: Popup::dummy(),
            player1_turn: true,
            last_turn_data: GameSaveData::dummy(),
            last_state: GameState::Playing,
            buttons: vec![
                restart_button, undo_button, quit_button
            ]
        }
    }

    fn locate_buttons(&mut self) {

        let (screen_w, _) = unsafe {GLOBAL_ENV.screen_size};
        let x = screen_w - 150.0;
        let mut curr_y = 30.0;

        for button in self.buttons.iter_mut() {
            button.move_to(x, curr_y);
            curr_y += 60.0;
        }

    }

    fn undo(&mut self) {
        self.player1 = self.last_turn_data.player1.clone();
        self.player2 = self.last_turn_data.player2.clone();
        self.clock = self.last_turn_data.clock;
        self.vertical_walls = self.last_turn_data.vertical_walls.clone();
        self.horizontal_walls = self.last_turn_data.horizontal_walls.clone();
        self.cross_walls = self.last_turn_data.cross_walls.clone();
        self.player1_turn = self.last_turn_data.player1_turn;

        self.last_clock_tick = time::Instant::now();
    }

    fn restart(&mut self) {
        self.player1 = Player::new(true);
        self.player2 = Player::new(false);
        self.clock = 0.0;
        self.vertical_walls = vec![vec![false; 9]; 10];
        self.horizontal_walls = vec![vec![false; 9]; 10];
        self.cross_walls = vec![vec![false; 10]; 10];
        self.player1_turn = true;
        self.last_turn_data = GameSaveData::dummy();

        self.last_clock_tick = time::Instant::now();
    }

    fn get_valid_moves(&self) -> Vec<(i32, i32)> {

        let mut result = Vec::with_capacity(4);

        let ((x, y), (another_x, another_y)) = if self.player1_turn {
            (self.player1.position, self.player2.position)
        } else {
            (self.player2.position, self.player1.position)
        };

        let mut possible_moves = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        while possible_moves.len() > 0 {
            let (dx, dy) = possible_moves.pop().unwrap();
            let (new_x, new_y) = (x + dx, y + dy);

            if (new_x < 0 || new_x > 8 || new_y < 0 || new_y > 8) || !self.is_movable_at((x, y), (dx, dy)) {}

            else if (new_x, new_y) == (another_x, another_y) {
                possible_moves.push((dx * 2, dy * 2));
            }

            else {
                result.push((new_x, new_y));
            }

        }

        result
    }

    fn is_vertical_wall_ok_at(&self, x: usize, y: usize) -> bool {
        (x < 9 && y < 8 && x > 0)
        && !(self.vertical_walls[x][y] || self.vertical_walls[x][y + 1])
        && !self.cross_walls[x][y + 1]
    }

    fn is_horizontal_wall_ok_at(&self, x: usize, y: usize) -> bool {
        (y < 9 && x < 8 && y > 0)
        && !(self.horizontal_walls[y][x] || self.horizontal_walls[y][x + 1])
        && !self.cross_walls[x + 1][y]
    }

    pub fn is_movable_at(&self, pos: (i32, i32), dir: (i32, i32)) -> bool {

        let result = if dir.0 == 0 {

            if dir.1 < 0 {
                !self.horizontal_walls[pos.1 as usize][pos.0 as usize]
            }

            else if dir.1 > 0 {
                !self.horizontal_walls[pos.1 as usize + 1][pos.0 as usize]
            }

            else {
                // stationary? that's illegal!
                unsafe { GLOBAL_ENV.raise_error(format!("Something went wrong: file `game.rs`, func `is_movable_at`, dir: {:?}", dir)); }
                true
            }

        }

        else if dir.1 == 0 {

            if dir.0 < 0 {
                !self.vertical_walls[pos.0 as usize][pos.1 as usize]
            }

            else if dir.0 > 0 {
                !self.vertical_walls[pos.0 as usize + 1][pos.1 as usize]
            }

            else {
                // stationary? that's illegal!
                unsafe { GLOBAL_ENV.raise_error(format!("Something went wrong: file `game.rs`, func `is_movable_at`, dir: {:?}", dir)); }
                true
            }

        }

        else {
            // moving in diagonal way? that's illegal!
            unsafe { GLOBAL_ENV.raise_error(format!("Something went wrong: file `game.rs`, func `is_movable_at`, dir: {:?}", dir)); }
            true
        };

        if dir.0.abs() == 2 || dir.1.abs() == 2 {
            result && self.is_movable_at((pos.0 + dir.0 / 2, pos.1 + dir.1 / 2), (dir.0 / 2, dir.1 / 2))
        }

        else {
            result
        }

    }

    fn next_turn(&mut self) {
        self.player1_turn = !self.player1_turn;
    }

    fn did_player1_win(&self) -> bool {
        self.player1.position.0 == 8
    }

    fn did_player2_win(&self) -> bool {
        self.player2.position.0 == 0
    }

}

impl Context for Game {
    fn frame(mut self: Box<Self>, inputs: Inputs) -> (Box<dyn Context>, Vec<Graphic>, Vec<SoundAction>) {
        let graphics;
        let (screen_w, screen_h) = unsafe {GLOBAL_ENV.screen_size};

        if inputs.is_screen_size_changed {
            self.locate_buttons();

            if self.state == GameState::ScreenTooSmall && screen_w >= 960.0 && screen_h >= 720.0 {
                self.state = self.last_state;
            }

        }

        if screen_w < 960.0 || screen_h < 720.0 {
            self.state = GameState::ScreenTooSmall;
            self.last_clock_tick = time::Instant::now();
        }

        match self.state {
            GameState::ScreenTooSmall => {
                let mut textbox = TextBox::new(
                    "The game screen is too small to play this game. Please resize the window and try again.",
                    20.0, 20.0, screen_w - 40.0, screen_h - 40.0, 28.0
                ).set_color(Color::new(192, 64, 64, 255))
                .set_horizontal_align(Alignment::Center)
                .set_vertical_align(Alignment::Center)
                .to_owned();
                graphics = textbox.render();

                (self, graphics, vec![])
            }
            GameState::GameOver => {

                for button in self.buttons.iter_mut() {
                    button.check_mouse(inputs.mouse_pos);
                }

                if inputs.mouse_pressed[0] {

                    if self.buttons[0].check_mouse(inputs.mouse_pos) {
                        self.curr_popup = Popup::new("Restart");
                        self.restart();
                    }

                    else if self.buttons[1].check_mouse(inputs.mouse_pos) {
                        self.curr_popup = Popup::new("You cannot undo it. The game is over.");
                    }

                    else if self.buttons[2].check_mouse(inputs.mouse_pos) {
                        unsafe {GLOBAL_ENV.quit()}
                    }

                }

                let (box_x, box_y) = ((screen_w - BOARD_SIZE) / 2.0, (screen_h - BOARD_SIZE) / 1.2);

                graphics = vec![
                    self.draw_board(box_x, box_y),
                    self.draw_ui(box_x, box_y),
                    self.player1.show_trace(box_x, box_y),
                    self.player2.show_trace(box_x, box_y),
                    self.curr_popup.render()
                ].concat();

                (self, graphics, vec![])
            }
            GameState::Playing => {
                let clock_check = time::Instant::now().duration_since(self.last_clock_tick.clone()).as_millis();

                if clock_check > 100 {
                    self.clock += clock_check as f32 / 1000.0;
                    self.last_clock_tick = time::Instant::now();
                }

                for button in self.buttons.iter_mut() {
                    button.check_mouse(inputs.mouse_pos);
                }

                let (box_x, box_y) = ((screen_w - BOARD_SIZE) / 2.0, (screen_h - BOARD_SIZE) / 1.2);

                let (mouse_x, mouse_y) = inputs.mouse_pos;
                let mouse_index = get_cursor_index(mouse_x, mouse_y, box_x, box_y);

                let mut board_graphics = self.draw_board(box_x, box_y);

                match mouse_index {
                    Index::Box(x, y) => {
                        board_graphics.push(
                            Graphic::new_rect(box_x + (x * 72) as f32 + 18.0, box_y + (y * 72) as f32 + 18.0, 54.0, 54.0, 0.0, Color::selection_mask())
                        );
                    }
                    Index::Vertical(x, y) if x < 9 && y < 8 && x > 0 => {
                        board_graphics.push(
                            Graphic::new_rect(box_x + (x * 72) as f32, box_y + (y * 72) as f32 + 18.0, 18.0, 126.0, 0.0, Color::selection_mask())
                        );
                    }
                    Index::Horizontal(x, y) if y < 9 && x < 8 && y > 0 => {
                        board_graphics.push(
                            Graphic::new_rect(box_x + (x * 72) as f32 + 18.0, box_y + (y * 72) as f32, 126.0, 18.0, 0.0, Color::selection_mask())
                        );
                    }
                    _ => {}
                }

                if inputs.mouse_pressed[0] {
                    let mut new_wall_placed = false;

                    match mouse_index {
                        Index::Box(x, y) => {
                            let (x, y) = (x as i32, y as i32);
                            let mut is_invalid_move = true;

                            for next_move in self.get_valid_moves() {

                                if (x, y) == next_move {
                                    self.last_turn_data = GameSaveData::from_game(&self);

                                    if self.player1_turn {
                                        self.player1.move_to(x, y);
                                    }

                                    else {
                                        self.player2.move_to(x, y);
                                    }

                                    self.next_turn();
                                    is_invalid_move = false;
                                    break;
                                }

                            }

                            if is_invalid_move {
                                self.curr_popup = Popup::new("Invalid Move!");
                            }

                        }
                        Index::Vertical(x, y) => {

                            if (self.player1_turn && self.player1.walls == 0) || (!self.player1_turn && self.player2.walls == 0) {
                                self.curr_popup = Popup::new("No walls to place!");
                            }

                            else if self.is_vertical_wall_ok_at(x, y) {
                                self.last_turn_data = GameSaveData::from_game(&self);

                                self.vertical_walls[x][y] = true;
                                self.vertical_walls[x][y + 1] = true;
                                self.cross_walls[x][y + 1] = true;

                                if self.player1_turn {
                                    self.player1.walls -= 1;
                                }

                                else {
                                    self.player2.walls -= 1;
                                }

                                new_wall_placed = true;
                                self.next_turn();
                            }

                            else {
                                self.curr_popup = Popup::new("Cannot place a wall there!");
                            }

                        }
                        Index::Horizontal(x, y) => {

                            if (self.player1_turn && self.player1.walls == 0) || (!self.player1_turn && self.player2.walls == 0) {
                                self.curr_popup = Popup::new("No walls to place!");
                            }

                            else if self.is_horizontal_wall_ok_at(x, y) {
                                self.last_turn_data = GameSaveData::from_game(&self);

                                self.horizontal_walls[y][x] = true;
                                self.horizontal_walls[y][x + 1] = true;
                                self.cross_walls[x + 1][y] = true;

                                if self.player1_turn {
                                    self.player1.walls -= 1;
                                }

                                else {
                                    self.player2.walls -= 1;
                                }

                                new_wall_placed = true;
                                self.next_turn();
                            }

                            else {
                                self.curr_popup = Popup::new("Cannot place a wall there!");
                            }

                        }
                        Index::None => {}
                    }

                    if new_wall_placed {
                        let visited = vec![vec![false; 10]; 10];

                        if !self.dfs(self.player1.position, 8, 1, &mut visited.clone()) || !self.dfs(self.player2.position, 0, -1, &mut visited.clone()) {
                            self.curr_popup = Popup::new("You may not trap a player!");
                            self.undo();
                        }

                    }

                    if self.buttons[0].check_mouse(inputs.mouse_pos) {
                        self.curr_popup = Popup::new("Restart");
                        self.restart();
                    }

                    else if self.buttons[1].check_mouse(inputs.mouse_pos) {
                        self.curr_popup = Popup::new("Undo");
                        self.undo();
                    }

                    else if self.buttons[2].check_mouse(inputs.mouse_pos) {
                        unsafe {GLOBAL_ENV.quit()}
                    }

                }

                if self.did_player1_win() {
                    self.curr_popup = Popup::new("Player 1 Won!");
                    self.state = GameState::GameOver;
                    self.last_state = GameState::GameOver;
                }

                else if self.did_player2_win() {
                    self.curr_popup = Popup::new("Player 2 Won!");
                    self.state = GameState::GameOver;
                    self.last_state = GameState::GameOver;
                }

                graphics = vec![
                    board_graphics,
                    self.draw_player(box_x, box_y),
                    self.draw_ui(box_x, box_y),
                    self.curr_popup.render()
                ].concat();

                (self, graphics, vec![])
            }
        }

    }
}

fn get_cursor_index(mouse_x: f32, mouse_y: f32, box_x: f32, box_y: f32) -> Index {

    if mouse_x <= box_x || mouse_x >= box_x + BOARD_SIZE || mouse_y <= box_y || mouse_y >= box_y + BOARD_SIZE {
        Index::None
    }

    else {
        let (mouse_index_x, mouse_index_y) = ((mouse_x - box_x) as usize / 72, (mouse_y - box_y) as usize / 72);
        let (mouse_rem_x, mouse_rem_y) = ((mouse_x - box_x) as usize % 72, (mouse_y - box_y) as usize % 72);

        if mouse_rem_x > 18 && mouse_rem_y > 18 {
            Index::Box(mouse_index_x, mouse_index_y)
        }

        else if mouse_rem_y <= 18 {
            Index::Horizontal(mouse_index_x, mouse_index_y)
        }

        else {
            Index::Vertical(mouse_index_x, mouse_index_y)
        }

    }

}

enum Index {
    None,
    Box(usize, usize),
    Horizontal(usize, usize),
    Vertical(usize, usize)
}

const BOARD_SIZE: f32 = 666.0;