mod camera;
mod logic;
mod map;
mod monsters;
mod player;
mod weapons;

use std::vec;

use logic::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let player_textures: Texture2D = load_texture("res/player.png").await.unwrap();
    player_textures.set_filter(FilterMode::Nearest);
    let terrain_textures: Texture2D = load_texture("res/terrain.png").await.unwrap();
    terrain_textures.set_filter(FilterMode::Nearest);
    let texture = vec![player_textures, terrain_textures];
    let textures = pack_texture(texture);
    let mut game_state = Game::new(textures);
    loop {
        let delta_time = get_frame_time();
        game_state.tick(&delta_time);
        game_state.draw();
        next_frame().await;
    }
}
