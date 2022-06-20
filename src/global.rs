#![allow(dead_code)]

pub struct GlobalEnv {
    pub messages: Vec<EnvMessage>,
    pub screen_size: (f32, f32),
    pub is_font_available: bool,
}

pub static mut GLOBAL_ENV: GlobalEnv = GlobalEnv::new();

impl GlobalEnv {

    pub const fn new() -> Self {
        GlobalEnv {
            messages: vec![],
            is_font_available: false,
            screen_size: (0.0, 0.0)
        }
    }

    pub fn is_font_available(&self) -> bool {
        self.is_font_available
    }

    pub fn quit(&mut self) {
        self.messages.push(EnvMessage::Quit);
    }

    // for unrecoverable errors
    pub fn raise_error(&mut self, message: String) {
        self.messages.push(EnvMessage::Error(message));
    }

    // load new image at image_index
    pub fn load_image(&mut self, path: &str, image_index: usize) {
        self.messages.push(EnvMessage::LoadImage(path.to_string(), image_index));
    }

    // load new sound at sound_index
    pub fn load_sound(&mut self, path: &str, sound_index: usize) {
        self.messages.push(EnvMessage::LoadSound(path.to_string(), sound_index));
    }

    pub fn show_cursor(&mut self, show: bool) {
        self.messages.push(EnvMessage::ShowCursor(show));
    }

    pub fn grab_cursor(&mut self, grab: bool) {
        self.messages.push(EnvMessage::GrabCursor(grab));
    }

}

pub enum EnvMessage {
    Quit,
    Error(String),
    LoadImage(String, usize),  // path, index
    LoadSound(String, usize),  // path, index
    ShowCursor(bool),
    GrabCursor(bool),
}