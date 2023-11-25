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
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let textures = load_textures().await;
    let mut game_state = Game::new(textures);
    loop {
        let delta_time = get_frame_time();
        game_state.tick(&delta_time);
        game_state.draw();
        next_frame().await;

        // game over if health < 0
        if game_state.player.heath < 0. {
            break;
        }
    }
}

async fn load_textures() -> Textures {
    let player_textures: Texture2D = load_texture("res/player.png").await.unwrap();
    let terrain_textures: Texture2D = load_texture("res/terrain.png").await.unwrap();
    let slime_textures: Texture2D = load_texture("res/slime.png").await.unwrap();

    let mut textures = vec![player_textures, terrain_textures, slime_textures];
    for texture in textures.iter_mut() {
        texture.set_filter(FilterMode::Nearest)
    }
    pack_texture(textures)
}
