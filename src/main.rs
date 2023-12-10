use std::{collections::HashMap, path::PathBuf};

use logic::*;
use macroquad::prelude::*;

mod camera;
mod logic;
mod map;
mod monsters;
mod npc;
mod player;
mod ui;
mod weapons;

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
        game_state.tick();
        game_state.draw();
        next_frame().await;

        // game over if health <= 0
        if game_state.player.props.heath <= 0. {
            break;
        }
    }
}

async fn load_textures() -> HashMap<String, Texture2D> {
    let mut textures: HashMap<String, Texture2D> = HashMap::new();
    let paths = get_path("res/", ".png");
    for path in paths {
        let texture = load_texture(path.to_str().unwrap()).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let name = to_name(&path);
        textures.insert(name, texture);
    }
    textures
}

fn to_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split_once(".")
        .unwrap()
        .0
        .to_string()
}
