use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

use crate::logic::Game;

impl Game {
    pub fn hud(&self) {
        let transparent = root_ui().style_builder().background(Image::empty()).build();
        let skin = Skin {
            window_style: transparent,
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&skin);

        root_ui().window(hash!(), vec2(0., 0.), vec2(250., 50.), |ui| {
            ui.label(vec2(10., 10.), "Health");
            let player_heath = self.player.props.heath;
            let rect = Rect::new(60., 10., player_heath, 30.);
            ui.canvas().rect(rect, RED, GREEN)
        });
    }
}
