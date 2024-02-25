#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}


impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {r, g, b, a}
    }

    pub fn box_normal() -> Self {
        Color::new(0, 0, 0, 255)
    }

    pub fn board_normal() -> Self {
        Color::new(160, 48, 48, 255)
    }

    pub fn selection_mask() -> Self {
        Color::new(255, 255, 255, 128)
    }

    pub fn wall() -> Self {
        Color::new(255, 208, 32, 255)
    }

    pub fn ui() -> Self {
        Color::new(255, 255, 255, 255)
    }

    pub fn player1_normal() -> Self {
        Color::new(0, 128, 0, 255)
    }

    pub fn player1_trans() -> Self {
        Color::new(0, 64, 0, 255)
    }

    pub fn player1_trace() -> Self {
        Color::new(0, 128, 0, 128)
    }

    pub fn player2_normal() -> Self {
        Color::new(192, 128, 255, 255)
    }

    pub fn player2_trans() -> Self {
        Color::new(96, 64, 128, 255)
    }

    pub fn player2_trace() -> Self {
        Color::new(192, 128, 255, 128)
    }
}
