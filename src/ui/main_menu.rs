use std::collections::HashMap;

use macroquad::prelude::*;

use super::{Button, PIXEL};
use crate::camera::draw_tiles;
use crate::logic::*;

#[derive(Clone, Debug)]
pub struct MainMenu {
    pub buttons: HashMap<String, Button>,
}

impl MainMenu {
    pub fn update(&mut self, screen_box: Rect) {
        let size = vec2(5., 2.) * TILE;
        let mut pos = screen_box.center();
        pos.y -= TILE;
        let play_button = Button::size(size).center_on(pos);
        pos.y += 2. * TILE;
        let quit_button = Button::size(size).center_on(pos);
        self.buttons.insert("Chơi".to_string(), play_button);
        self.buttons.insert("Thoát".to_string(), quit_button);
    }

    pub fn draw_background(&self, bg: &Texture2D, screen_box: Rect) {
        let dest_size = Some(screen_box.size());
        let params = DrawTextureParams {
            dest_size,
            ..Default::default()
        };

        draw_texture_ex(bg, screen_box.x, screen_box.y, WHITE, params)
    }

    pub fn draw_buttons(&self, texture: &Texture2D, font: &Font) {
        let mesh = button_mesh();
        for (name, button) in self.buttons.iter() {
            draw_tiles(&mesh, button.hitbox.point(), texture, None, TILE_SIZE);
            let params = TextParams {
                font: Some(font),
                font_size: 60,
                color: BLACK,
                ..Default::default()
            };
            let rect = measure_text(name, Some(font), params.font_size, 1.);
            let delta_x = (button.hitbox.w - rect.width) / 2.;
            let delta_y = (button.hitbox.h - rect.height) / 2.;
            let text_box = vec2(button.hitbox.x + delta_x, button.hitbox.y + delta_y);
            draw_text_ex(
                name,
                text_box.x,
                button.hitbox.center().y + 3. * PIXEL,
                params,
            )
        }
    }
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
        }
    }
}

pub fn button_mesh() -> Vec<Vec<u16>> {
    vec![vec![10, 11, 11, 11, 12], vec![22, 23, 23, 23, 24]]
}
