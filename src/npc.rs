use crate::logic::*;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct NPC {
    pub name: String,
    pub dialogs: Vec<String>,
    pub hitbox: Rect,
    pub anim: AnimatedSprite,
    pub is_talking: bool,
}

impl NPC {
    pub fn new(name: &str, diag_path: &str, hitbox: Rect) -> Self {
        // Because cross platform lol
        let path: PathBuf = diag_path.replace("..", "assets").into();
        let dialog = read_to_string(path).unwrap();
        let dialogs: Vec<String> = dialog.split("|").map(|str| str.to_string()).collect();

        let anim = npc_anim();

        NPC {
            name: name.to_string(),
            dialogs,
            anim,
            hitbox,
            is_talking: false,
        }
    }

    /*
    pub fn tick(&mut self) {
    }
    */

    pub fn draw(&self, texture: &Texture2D) {
        let dest_size = Some(self.anim.frame().dest_size * SCALE_FACTOR);
        let source = Some(self.anim.frame().source_rect);
        let draw_param = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        let pos = self.hitbox.center();

        draw_texture_ex(
            texture,
            pos.x - STANDARD_SQUARE,
            pos.y - STANDARD_SQUARE,
            WHITE,
            draw_param,
        )
    }

    pub fn pos(&self) -> Vec2 {
        self.hitbox.center()
    }

    pub fn draw_overlay(&self, texture: &Texture2D) {
        let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
        let source = Some(Rect::new(TILE_SIZE, 0., TILE_SIZE, TILE_SIZE));

        let draw_param = DrawTextureParams {
            dest_size,
            source,
            ..Default::default()
        };

        let pos = self.hitbox.center();

        draw_texture_ex(
            texture,
            pos.x - STANDARD_SQUARE,
            pos.y - 2. * STANDARD_SQUARE,
            WHITE,
            draw_param,
        );
    }
}

fn npc_anim() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[
            make_anim("down", 0, 6, 12),
            make_anim("left", 1, 6, 12),
            make_anim("up", 2, 6, 12),
            make_anim("right", 3, 6, 12),
        ],
        true,
    )
}
