use logic::*;
use macroquad::prelude::*;
use std::rc::Rc;
use std::{collections::HashMap, path::PathBuf};

mod camera;
pub mod interactables;
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
    let font = load_font().await;
    let textures = load_textures().await;
    let mut game_state = Game::new(textures, font);
    loop {
        game_state.tick();
        game_state.draw();
        next_frame().await;

        // game over if health <= 0
        if game_state.quit {
            break;
        }
    }
}

async fn load_textures() -> HashMap<Rc<str>, Texture2D> {
    let mut textures: HashMap<Rc<str>, Texture2D> = HashMap::new();
    let paths = get_path("res/", ".png");
    for path in paths {
        let texture = load_texture(path.to_str().unwrap()).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let name = to_name(&path);
        textures.insert(name, texture);
    }
    textures
}

async fn load_font() -> Font {
    load_ttf_font("assets/font/Monocraft.otf").await.unwrap()
}

fn to_name(path: &PathBuf) -> Rc<str> {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split_once(".")
        .unwrap()
        .0
        .into()
}
