#![allow(dead_code)]

use macroquad::audio::{Sound, PlaySoundParams, play_sound, play_sound_once, stop_sound};

#[derive(Copy, Clone)]
pub enum SoundAction {
    PlayOnce(usize),  // if it's already being played, the previous one stops
    PlayLoop(usize),
    Stop(usize),
    MuteAll,
    UnmuteAll,
    StopAll
}

pub struct SoundManager {
    is_looping: Vec<bool>,
    muted: bool,
    pub sounds: Vec<Sound>
}

impl SoundManager {

    pub fn new(sounds: Vec<Sound>) -> SoundManager {
        let is_looping = vec![false;sounds.len()];

        SoundManager {
            sounds, is_looping, muted: false
        }
    }

    pub fn stop_all(&mut self) {

        for snd in self.sounds.iter() {
            stop_sound(*snd);
        }

        self.is_looping = vec![false;self.is_looping.len()];
    }

    pub fn frame(&mut self, actions: Vec<SoundAction>) {

        for action in actions.iter() {

            if self.muted {
                match action {
                    SoundAction::UnmuteAll => {
                        self.muted = false;
                    }
                    _ => {}
                }

                continue;
            }

            match action {
                SoundAction::PlayOnce(sound) => {
                    play_sound_once(self.sounds[*sound]);
                    self.is_looping[*sound] = false;
                }
                SoundAction::PlayLoop(sound) => {

                    if !self.is_looping[*sound] {
                        play_sound(self.sounds[*sound], PlaySoundParams {looped: true, ..PlaySoundParams::default()});
                        self.is_looping[*sound] = true;
                    }

                }
                SoundAction::Stop(sound) => {

                    if self.is_looping[*sound] {
                        stop_sound(self.sounds[*sound]);
                        self.is_looping[*sound] = false;
                    }

                }
                SoundAction::StopAll => {

                    for snd in self.sounds.iter() {
                        stop_sound(*snd);
                    }

                    self.is_looping = vec![false;self.is_looping.len()];
                }
                SoundAction::MuteAll => {
                    self.muted = true;
                }
                SoundAction::UnmuteAll => {
                    self.muted = false;
                }
            }
        }

    }

}
