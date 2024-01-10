use std::collections::HashMap;

use macroquad::prelude::*;

use super::Button;
use crate::camera::{draw_tiles, render_text, Utils};
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
        self.buttons.insert("play".to_string(), play_button);
        self.buttons.insert("quit".to_string(), quit_button);
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
            render_text(button.hitbox.shift(-TILE * 1.9, -TILE / 2.), name, params)
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
