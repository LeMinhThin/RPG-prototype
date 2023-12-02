use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

use crate::logic::Game;

impl Game {
    pub fn show_ui(&self) {
        root_ui().window(hash!(), vec2(0., 0.), vec2(250., 50.), |ui| {
            let player_heath = self.player.props.heath;
            ui.label(vec2(10., 10.), &format!("Health: {player_heath}/100"))
        });
    }
}
