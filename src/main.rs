mod game;
mod engine;

mod ai;
mod dfs;
mod player;
mod popup;
mod mouse_trace;

use macroquad::prelude::*;
use engine::context::Context;
use engine::global::{EnvMessage, GLOBAL_ENV};
use std::{time, thread};

// configure the game window here
fn conf() -> Conf {
    Conf {
        window_height: 760,
        window_width: 1160,
        window_resizable: true,
        window_title: "Test".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {

    let mut sound_manager = engine::sound::SoundManager::new(vec![]);
    let mut fonts = vec![];
    let mut textures = vec![];

    #[cfg(feature = "profile")]
    let mut profiler = engine::profile::Profiler::new();

    #[cfg(feature = "profile")]
    unsafe {engine::profile::Profiler::init_function_profilers();}

    match engine::loader::load_all().await {
        Ok((fonts_, textures_, sounds)) => {
            fonts = fonts_;
            textures = textures_;
            sound_manager = engine::sound::SoundManager::new(sounds);
        }
        Err(err) => {
            unsafe { GLOBAL_ENV.raise_error(&err); }
        }
    }

    if fonts.len() > 0 {
        // it assumes that the fonts are not dynamically loaded
        unsafe{ GLOBAL_ENV.is_font_available = true; }
    }

    else {  // if the game does not load any font, it tries to load the default font

        match engine::loader::default_font() {

            Err(err) => {
                unsafe { GLOBAL_ENV.raise_error(&err); }
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

        let inputs = engine::inputs::Inputs::poll(is_screen_size_changed);

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
                    },
                    EnvMessage::Error(message) => {
                        curr_game = Box::new(engine::error::ErrorHandler::new(message));
                        continue 'game_loop;
                    },
                    EnvMessage::ShowCursor(show) => {
                        show_mouse(show);
                    },
                    EnvMessage::GrabCursor(grab) => {
                        set_cursor_grab(grab);
                    },
                    EnvMessage::LoadImage(path, index) => {
                        if index >= textures.len() {
                            GLOBAL_ENV.messages.push(EnvMessage::Error(
                                format!("Index error when dynamically loading an image: `index` is {}, but `textures.len()` is {}", index, textures.len())
                            ));
                        }

                        else {
                            match engine::loader::load_image_from_file(&path) {
                                Err(message) => {
                                    GLOBAL_ENV.messages.push(EnvMessage::Error(message));
                                }
                                Ok(image) => {
                                    textures[index] = image;
                                }
                            }
                        }
                    },
                    EnvMessage::LoadSound(path, index) => {
                        if index >= sound_manager.sounds.len() {
                            GLOBAL_ENV.messages.push(EnvMessage::Error(
                                format!("Index error when dynamically loading a sound: `index` is {}, but `sounds.len()` is {}", index, sound_manager.sounds.len())
                            ));
                        }

                        else {
                            match engine::loader::load_sound_from_file(&path).await {
                                Err(message) => {
                                    GLOBAL_ENV.messages.push(EnvMessage::Error(message));
                                }
                                Ok(sound) => {
                                    sound_manager.sounds[index] = sound;
                                }
                            }
                        }
                    },
                }
            }
        }

        // play sounds
        sound_manager.frame(sound_actions);

        #[cfg(feature = "profile")]
        let graphics = vec![graphics, profiler.rendered.clone()].concat();

        #[cfg(feature = "profile")]
        for graphic in graphics.iter() {
            profiler.count_graphic(graphic);
        }

        engine::graphic::render(graphics, &textures, &fonts);

        // fps is set to 40 by default
        // but, with the macroquad backend, the fps is controlled by the backend, not by this engine
        while time::Instant::now().duration_since(frame_begin.clone()).as_millis() < 25 {
            thread::sleep(time::Duration::new(0, 1_000_000_000u32 / 600));
        }

        next_frame().await;
    }
}
