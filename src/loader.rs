use macroquad::{text::{Font, load_ttf_font_from_bytes}, texture::Texture2D, audio::{Sound, load_sound_from_bytes}};
use crate::file_io::read_bytes;

/*
DO NOT call these functions inside the game loop!!!

The return types of the functions are dependent on its backend, which is subject to change.

All you have to care is the body of the `load_all` function.
*/

pub async fn load_all() -> Result<(Vec<Font>, Vec<Texture2D>, Vec<Sound>), String> {

    // images = vec![load_image(include_bytes!("...")), load_image_from_file("...")?, ...];
    let images = vec![];

    // fonts = vec![load_font(include_bytes!("..."))?, load_font_from_file("...")?, ...];
    let fonts = vec![];

    // sounds = vec![load_sound(include_bytes!("..."))?, load_sound_from_file("...")?, ...];
    let sounds = vec![];

    Ok((fonts, images, sounds))
}


fn load_image(bytes: &[u8]) -> Texture2D {
    Texture2D::from_file_with_format(bytes, None)
}


async fn load_sound(bytes: &[u8]) -> Result<Sound, String> {

    match load_sound_from_bytes(bytes).await {
        Ok(s) => Ok(s),
        Err(_) => Err("failed to load sound!".to_string())
    }

}


fn load_font(bytes: &[u8]) -> Result<Font, String> {
    match load_ttf_font_from_bytes(bytes) {
        Ok(s) => Ok(s),
        Err(_) => Err("failed to load sound!".to_string())
    }
}

pub fn load_image_from_file(path: &str) -> Result<Texture2D, String> {

    match read_bytes(path) {
        Err(_) => Err(format!("Failed to open file: {}", path)),
        Ok(v) => Ok(load_image(&v))
    }

}

pub async fn load_sound_from_file(path: &str) -> Result<Sound, String> {

    match read_bytes(path) {
        Err(_) => Err(format!("Failed to open file: {}", path)),
        Ok(v) => load_sound(&v).await
    }

}

fn load_font_from_file(path: &str) -> Result<Font, String> {

    match read_bytes(path) {
        Err(_) => Err(format!("Failed to open file: {}", path)),
        Ok(v) => load_font(&v)
    }

}

pub fn default_font() -> Result<Font, String> {
    // it almost never fails on the runtime
    match load_font(include_bytes!("font.ttf")) {
        Ok(f) => Ok(f),
        Err(_) => {Err("failed to load the default font!".to_string())}
    }
}