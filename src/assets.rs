use macroquad::prelude::*;

use macroquad::audio::{
    PlaySoundParams, Sound, load_sound, load_sound_from_bytes, play_sound, play_sound_once,
    stop_sound,
};
use macroquad::file::load_file;
use macroquad::rand::gen_range;

use crate::gameplay_config::FONT_PATH;

const SFX_CLICK_PATH: &str = "assets/audio/Click.wav";
const SFX_HIT_ANIMAL_PATH: &str = "assets/audio/HitAnimal.wav";
const SFX_EATING_PATH: &str = "assets/audio/Eating.wav";
const SFX_NOMAD_SELECTION_1_PATH: &str = "assets/audio/NomadSelection1.wav";
const SFX_NOMAD_SELECTION_2_PATH: &str = "assets/audio/NomadSelection2.wav";
const SFX_WIN_PATH: &str = "assets/audio/Win.wav";
const SFX_GAME_OVER_PATH: &str = "assets/audio/GameOver.wav";

const BGM_GAME_THEME_PATH: &str = "assets/audio/GameTheme.wav";
const BGM_GAME_THEME_VOLUME: f32 = 0.5;

const SFX_NOMAD_SELECTION_VOLUME: f32 = 0.5;

pub struct Assets {
    pub main_font: Option<Font>,

    pub sfx_click: Option<Sound>,
    pub sfx_hit_animal: Option<Sound>,
    pub sfx_eating: Option<Sound>,
    pub sfx_nomad_selection_1: Option<Sound>,
    pub sfx_nomad_selection_2: Option<Sound>,
    pub sfx_win: Option<Sound>,
    pub sfx_game_over: Option<Sound>,

    pub bgm_game_theme: Option<Sound>,
    pub bgm_game_theme_seconds: Option<f32>,
}

impl Assets {
    fn wav_duration_seconds(bytes: &[u8]) -> Option<f32> {
        // Minimal RIFF/WAVE duration parser. Reads the `fmt ` and `data` chunks.
        if bytes.len() < 12 {
            return None;
        }
        if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return None;
        }

        let mut i = 12;
        let mut channels: Option<u16> = None;
        let mut sample_rate: Option<u32> = None;
        let mut bits_per_sample: Option<u16> = None;
        let mut data_len: Option<u32> = None;

        while i + 8 <= bytes.len() {
            let id = &bytes[i..i + 4];
            let size = u32::from_le_bytes(bytes[i + 4..i + 8].try_into().ok()?) as usize;
            i += 8;

            if i + size > bytes.len() {
                return None;
            }

            if id == b"fmt " {
                if size < 16 {
                    return None;
                }

                channels = Some(u16::from_le_bytes(bytes[i + 2..i + 4].try_into().ok()?));
                sample_rate = Some(u32::from_le_bytes(bytes[i + 4..i + 8].try_into().ok()?));
                bits_per_sample =
                    Some(u16::from_le_bytes(bytes[i + 14..i + 16].try_into().ok()?));
            } else if id == b"data" {
                data_len = Some(size as u32);
            }

            // Chunks are padded to even size.
            i += size + (size % 2);

            if data_len.is_some()
                && channels.is_some()
                && sample_rate.is_some()
                && bits_per_sample.is_some()
            {
                break;
            }
        }

        let channels = channels? as u32;
        let sample_rate = sample_rate?;
        let bits_per_sample = bits_per_sample? as u32;
        let data_len = data_len? as u32;

        let bytes_per_sample = (bits_per_sample + 7) / 8;
        let bytes_per_second = sample_rate
            .saturating_mul(channels)
            .saturating_mul(bytes_per_sample);
        if bytes_per_second == 0 {
            return None;
        }

        Some(data_len as f32 / bytes_per_second as f32)
    }

    async fn load_sound_opt(path: &str) -> Option<Sound> {
        match load_sound(path).await {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Error loading '{path}': {:?}", e);
                None
            }
        }
    }

    pub async fn load() -> Self {
        let result = load_ttf_font(FONT_PATH).await;
        let main_font = match result {
            Ok(font) => Some(font),
            Err(e) => {
                println!("Error loading '{FONT_PATH}': {:?}", e);
                None
            }
        };

        let sfx_click = Self::load_sound_opt(SFX_CLICK_PATH).await;
        let sfx_hit_animal = Self::load_sound_opt(SFX_HIT_ANIMAL_PATH).await;
        let sfx_eating = Self::load_sound_opt(SFX_EATING_PATH).await;
        let sfx_nomad_selection_1 = Self::load_sound_opt(SFX_NOMAD_SELECTION_1_PATH).await;
        let sfx_nomad_selection_2 = Self::load_sound_opt(SFX_NOMAD_SELECTION_2_PATH).await;
        let sfx_win = Self::load_sound_opt(SFX_WIN_PATH).await;
        let sfx_game_over = Self::load_sound_opt(SFX_GAME_OVER_PATH).await;

        // Load theme via bytes so we can derive its duration.
        let (bgm_game_theme, bgm_game_theme_seconds) = match load_file(BGM_GAME_THEME_PATH).await {
            Ok(bytes) => {
                let seconds = Self::wav_duration_seconds(&bytes);
                let sound = match load_sound_from_bytes(&bytes).await {
                    Ok(s) => Some(s),
                    Err(e) => {
                        println!("Error loading '{BGM_GAME_THEME_PATH}': {:?}", e);
                        None
                    }
                };
                (sound, seconds)
            }
            Err(e) => {
                println!("Error loading '{BGM_GAME_THEME_PATH}': {:?}", e);
                (None, None)
            }
        };

        Assets {
            main_font,

            sfx_click,
            sfx_hit_animal,
            sfx_eating,
            sfx_nomad_selection_1,
            sfx_nomad_selection_2,
            sfx_win,
            sfx_game_over,

            bgm_game_theme,
            bgm_game_theme_seconds,
        }
    }

    pub fn play_click(&self) {
        if let Some(s) = &self.sfx_click {
            play_sound_once(s);
        }
    }

    pub fn play_hit_animal(&self) {
        if let Some(s) = &self.sfx_hit_animal {
            play_sound_once(s);
        }
    }

    pub fn play_eating(&self) {
        if let Some(s) = &self.sfx_eating {
            play_sound_once(s);
        }
    }

    pub fn play_nomad_selection(&self) {
        let play = |s: &Sound| {
            play_sound(
                s,
                PlaySoundParams {
                    looped: false,
                    volume: SFX_NOMAD_SELECTION_VOLUME,
                },
            );
        };

        match (&self.sfx_nomad_selection_1, &self.sfx_nomad_selection_2) {
            (Some(a), Some(b)) => {
                if gen_range(0, 2) == 0 {
                    play(a);
                } else {
                    play(b);
                }
            }
            (Some(a), None) => play(a),
            (None, Some(b)) => play(b),
            (None, None) => {}
        }
    }

    pub fn play_win(&self) {
        if let Some(s) = &self.sfx_win {
            play_sound_once(s);
        }
    }

    pub fn play_game_over(&self) {
        if let Some(s) = &self.sfx_game_over {
            play_sound_once(s);
        }
    }

    pub fn play_game_theme_once(&self) {
        let Some(s) = &self.bgm_game_theme else {
            return;
        };

        stop_sound(s);
        play_sound(
            s,
            PlaySoundParams {
                looped: false,
                volume: BGM_GAME_THEME_VOLUME,
            },
        );
    }

    pub fn play_game_theme_looped(&self) {
        // Fallback behavior when we can't time the track duration.
        let Some(s) = &self.bgm_game_theme else {
            return;
        };

        stop_sound(s);
        play_sound(
            s,
            PlaySoundParams {
                looped: true,
                volume: BGM_GAME_THEME_VOLUME,
            },
        );
    }

    pub fn stop_game_theme(&self) {
        if let Some(s) = &self.bgm_game_theme {
            stop_sound(s);
        }
    }
}
