use logic::*;
use macroquad::prelude::*;

mod camera;
mod logic;
mod map;
mod monsters;
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

        // game over if health < 0
        if game_state.player.props.heath <= 0. {
            break;
        }
    }
}

async fn load_textures() -> Textures {
    let player_textures: Texture2D = load_texture("res/player.png").await.unwrap();
    let terrain_textures: Texture2D = load_texture("res/terrain.png").await.unwrap();
    let slime_textures: Texture2D = load_texture("res/slime.png").await.unwrap();
    let mushroom_textures: Texture2D = load_texture("res/mushroom.png").await.unwrap();

    let textures = vec![
        player_textures,
        terrain_textures,
        slime_textures,
        mushroom_textures,
    ];
    for texture in textures.iter() {
        texture.set_filter(FilterMode::Nearest)
    }
    pack_texture(textures)
}

fn pack_texture(texture: Vec<Texture2D>) -> Textures {
    Textures {
        player: texture[0].clone(),
        terrain: texture[1].clone(),
        slime: texture[2].clone(),
        mushroom: texture[3].clone(),
    }
}
