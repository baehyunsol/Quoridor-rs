mod color;
mod context;
mod inputs;
mod global;
mod graphic;
mod loader;
mod sound;
mod game;
mod keys;
mod file_io;
mod error;
mod widget;
mod player;
mod popup;
mod dfs;

#[cfg(feature = "profile")]
mod profile;

use macroquad::prelude::*;
use context::Context;
use global::{EnvMessage, GLOBAL_ENV};
use std::{time, thread};
use graphic::Graphic;

// configure the game window here
fn conf() -> Conf {
    Conf {
        window_height: 720,
        window_width: 720,
        window_resizable: true,
        window_title: "Quoridor".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {

    let mut sound_manager = sound::SoundManager::new(vec![]);
    let mut fonts = vec![];
    let mut textures = vec![];

    #[cfg(feature = "profile")]
    let mut profiler = profile::Profiler::new();

    #[cfg(feature = "profile")]
    unsafe {profile::Profiler::init_function_profilers();}

    match loader::load_all().await {
        Ok((fonts_, textures_, sounds)) => {
            fonts = fonts_;
            textures = textures_;
            sound_manager = sound::SoundManager::new(sounds);
        }
        Err(err) => {
            unsafe { GLOBAL_ENV.raise_error(err); }
        }
    }

    if fonts.len() > 0 {
        // it assumes that the fonts are not dynamically loaded
        unsafe{ GLOBAL_ENV.is_font_available = true; }
    }

    else {  // if the game does not load any font, it tries to load the default font

        match loader::default_font() {

            Err(err) => {
                unsafe { GLOBAL_ENV.raise_error(err); }
            }
            Ok(f) => {
                unsafe { GLOBAL_ENV.is_font_available = true; }
                fonts.push(f);
            }

        }

    }

    unsafe { GLOBAL_ENV.screen_size = (macroquad::window::screen_width(), macroquad::window::screen_height()); }

    let mut curr_game: Box<dyn Context> = Box::new(game::Game::new());

    'game_loop: loop {
        let frame_begin = time::Instant::now();

        #[cfg(feature = "profile")]
        profiler.new_frame(frame_begin.clone());

        let mut is_screen_size_changed = false;

        // update GLOBAL_ENV.screen_size every frame
        unsafe {
            let new_screen_size = (macroquad::window::screen_width(), macroquad::window::screen_height());

            if new_screen_size != GLOBAL_ENV.screen_size {
                is_screen_size_changed = true;
            }

            GLOBAL_ENV.screen_size = new_screen_size;
        }

        let inputs = inputs::Inputs::poll(is_screen_size_changed);

        // run a frame
        let (g, graphics, sound_actions) = curr_game.frame(inputs);
        curr_game = g;

        // handle GLOBAL_ENV.messages
        unsafe {

            while GLOBAL_ENV.messages.len() > 0 {

                match GLOBAL_ENV.messages.pop().unwrap() {
                    EnvMessage::Quit => {
                        sound_manager.stop_all();
                        break 'game_loop;
                    }
                    EnvMessage::Error(message) => {
                        curr_game = Box::new(error::ErrorHandler::new(message));
                        continue 'game_loop;
                    }
                    EnvMessage::ShowCursor(show) => {
                        show_mouse(show);
                    }
                    EnvMessage::GrabCursor(grab) => {
                        set_cursor_grab(grab);
                    }
                    EnvMessage::LoadImage(path, index) => {

                        if index >= textures.len() {
                            GLOBAL_ENV.messages.push(EnvMessage::Error(
                                format!("Index error when dynamically loading an image: `index` is {}, but `textures.len()` is {}", index, textures.len())
                            ));
                        }

                        else {
                            textures[index].delete();

                            match loader::load_image_from_file(&path) {
                                Err(message) => {
                                    GLOBAL_ENV.messages.push(EnvMessage::Error(message));
                                }
                                Ok(image) => {
                                    textures[index] = image;
                                }
                            }

                        }

                    }
                    EnvMessage::LoadSound(path, index) => {

                        if index >= sound_manager.sounds.len() {
                            GLOBAL_ENV.messages.push(EnvMessage::Error(
                                format!("Index error when dynamically loading a sound: `index` is {}, but `sounds.len()` is {}", index, sound_manager.sounds.len())
                            ));
                        }

                        else {
                            match loader::load_sound_from_file(&path).await {
                                Err(message) => {
                                    GLOBAL_ENV.messages.push(EnvMessage::Error(message));
                                }
                                Ok(sound) => {
                                    sound_manager.sounds[index] = sound;
                                }
                            }
                        }

                    }
                }

            }

        }

        // play sounds
        sound_manager.frame(sound_actions);

        #[cfg(feature = "profile")]
        let graphics = vec![graphics, profiler.rendered.clone()].concat();

        // draw graphics
        for graphic in graphics.into_iter() {

            #[cfg(feature = "profile")]
            profiler.count_graphic(&graphic);

            match graphic {
                Graphic::Rect {x, y, w, h, thickness, color} => {

                    if thickness > 0.0 {
                        draw_rectangle_lines(
                            x, y, w, h, thickness, Color::from_rgba(color.r, color.g, color.b, color.a)
                        );
                    }

                    else {
                        draw_rectangle(
                            x, y, w, h, Color::from_rgba(color.r, color.g, color.b, color.a)
                        );
                    }

                }
                Graphic::Circle {x, y, r, thickness, color} => {
                    let sides = if r < 60.0 { 15 } else if r < 180.0 { 24 } else { 30 };

                    if thickness > 0.0 {
                        draw_poly_lines(
                            x, y, sides, r, 0.0, thickness, Color::from_rgba(color.r, color.g, color.b, color.a)
                        );
                    }

                    else {
                        draw_poly(
                            x, y, sides, r, 0.0, Color::from_rgba(color.r, color.g, color.b, color.a)
                        );
                    }
                
                }
                Graphic::Line {x1, y1, x2, y2, thickness, color} => {
                    draw_line(x1, y1, x2, y2, thickness, Color::from_rgba(color.r, color.g, color.b, color.a));
                }
                Graphic::Triangle {x1, y1, x2, y2, x3, y3, thickness, color} => {

                    if thickness > 0.0 {
                        draw_triangle_lines(
                            macroquad::math::Vec2::new(x1, y1), macroquad::math::Vec2::new(x2, y2), macroquad::math::Vec2::new(x3, y3), thickness, Color::from_rgba(color.r, color.g, color.b, color.a)
                        );
                    }

                    else {
                        draw_triangle(
                            macroquad::math::Vec2::new(x1, y1), macroquad::math::Vec2::new(x2, y2), macroquad::math::Vec2::new(x3, y3), Color::from_rgba(color.r, color.g, color.b, color.a)
                        );
                    }

                }
                Graphic::Text {x, y, size, string, font, color} => {

                    #[cfg(feature = "profile")]
                    if font >= fonts.len() {
                        unsafe { GLOBAL_ENV.raise_error("Uninitialized font used!".to_string()); }
                        break;
                    }

                    draw_text_ex(
                        &string, x, y, TextParams {
                            font_size: size,
                            color: Color::from_rgba(color.r, color.g, color.b, color.a),
                            font: fonts[font],
                            ..Default::default()
                        }
                    );
                }
                Graphic::Image {x, y, image_index, color} => {

                    #[cfg(feature = "profile")]
                    if image_index >= textures.len() {
                        unsafe { GLOBAL_ENV.raise_error("Uninitialized image used!".to_string()); }
                        break;
                    }

                    draw_texture(
                        textures[image_index], x, y, Color::from_rgba(color.r, color.g, color.b, color.a),
                    );
                }

            }
        }

        // fps is set to 40 by default
        // but, with the macroquad backend, the fps is controlled by the backend, not by this engine
        while time::Instant::now().duration_since(frame_begin.clone()).as_millis() < 25 {
            thread::sleep(time::Duration::new(0, 1_000_000_000u32 / 600));
        }

        next_frame().await;
    }

}