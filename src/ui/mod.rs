use macroquad::prelude::*;

use crate::logic::{Game, STANDARD_SQUARE, TILE_SIZE};
use crate::player::{ONE_PIXEL, PLAYER_HEALTH};

mod inventory;

impl Game {
    pub fn hud(&self) {
        self.draw_health_bar();
    }

    fn draw_health_bar(&self) {
        let screen = self.cam_box();

        let health_percentage = self.player.props.health / PLAYER_HEALTH;
        draw_rectangle(
            screen.x + 3. * ONE_PIXEL,
            screen.y + 3. * ONE_PIXEL,
            66. * ONE_PIXEL * health_percentage,
            4. * ONE_PIXEL,
            RED,
        );

        let texture = &self.textures["ui"];
        let dest_size = Some(vec2(STANDARD_SQUARE * 3., STANDARD_SQUARE));
        let source = Some(Rect::new(0., TILE_SIZE, TILE_SIZE * 3., TILE_SIZE));
        let params = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(texture, screen.x, screen.y, WHITE, params)
    }
}
