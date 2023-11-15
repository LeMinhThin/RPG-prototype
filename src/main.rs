mod camera;
mod logic;
mod map;
mod monsters;
mod player;
mod weapons;

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
    let textures: Texture2D = load_texture("res/player.png").await.unwrap();
    textures.set_filter(FilterMode::Nearest);
    let mut game_state = Game::new(textures);
    loop {
        let delta_time = get_frame_time();
        game_state.tick(&delta_time);
        game_state.draw();
        next_frame().await;
    }
}
