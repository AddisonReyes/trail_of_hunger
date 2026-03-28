use macroquad::prelude::*;

use crate::gameplay_config::FONT_PATH;

pub struct Assets {
    pub main_font: Option<Font>,
}

impl Assets {
    pub async fn load() -> Self {
        let result = load_ttf_font(FONT_PATH).await;
        let mut alagard_font: Option<Font> = None;

        match result {
            Ok(font) => {
                alagard_font = Some(font);
            }
            Err(e) => {
                println!("Error loading \'{FONT_PATH}\': {:?}", e);
            }
        }

        return Assets {
            main_font: alagard_font,
        };
    }
}
