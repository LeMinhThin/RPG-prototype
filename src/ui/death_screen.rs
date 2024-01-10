use crate::{
    camera::{draw_tiles, render_text, Utils},
    logic::TILE,
};
use std::collections::HashMap;

use super::*;

#[derive(Clone, Debug)]
pub struct DeathScreen {
    pub buttons: HashMap<String, Button>,
}

impl DeathScreen {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
        }
    }

    pub fn update(&mut self, screen_box: Rect) {
        let button_size = vec2(5., 2.) * TILE;
        let mut pos = screen_box.center();

        pos.y += TILE;
        let respawn_button = Button::size(button_size).center_on(pos);
        pos.y += 2. * TILE;
        let main_menu_button = Button::size(button_size).center_on(pos);
        self.buttons.insert("respawn".to_string(), respawn_button);
        self.buttons.insert("menu".to_string(), main_menu_button);
    }

    pub fn draw_buttons(&self, texture: &Texture2D, font: &Font) {
        let mesh = button_mesh();

        for (name, button) in self.buttons.iter() {
            draw_tiles(&mesh, button.hitbox.point(), texture, None, TILE_SIZE);
            let params = TextParams {
                font: Some(font),
                font_size: 120,
                color: BLACK,
                ..Default::default()
            };
            render_text(button.hitbox.shift(-TILE, -TILE / 2.), name, params);
        }
    }
}
