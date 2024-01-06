use macroquad::prelude::*;

use crate::logic::{Game, TILE, TILE_SIZE};
use crate::player::{PIXEL, PLAYER_HEALTH};

pub mod inventory;
pub mod items;

impl Game {
    pub fn hud(&self) {
        self.draw_health_bar();
    }

    fn draw_health_bar(&self) {
        let screen = self.cam_box();

        let health_percentage = self.player.props.health / PLAYER_HEALTH;
        draw_rectangle(
            screen.x + 3. * PIXEL,
            screen.y + 3. * PIXEL,
            66. * PIXEL * health_percentage,
            4. * PIXEL,
            RED,
        );

        let texture = &self.textures["ui"];
        let dest_size = Some(vec2(TILE * 3., TILE));
        let source = Some(Rect::new(0., TILE_SIZE, TILE_SIZE * 3., TILE_SIZE));
        let params = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(texture, screen.x, screen.y, WHITE, params)
    }
}
