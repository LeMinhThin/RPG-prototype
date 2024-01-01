use crate::logic::*;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use serde_json::Value;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::rc::Rc;

pub struct NPC {
    pub name: Rc<str>,
    pub dialogs: Vec<String>,
    pub hitbox: Rect,
    pub anim: AnimatedSprite,
    pub is_talking: bool,
}

impl NPC {
    pub fn new(name: &str, diag_path: &str, hitbox: Rect) -> Self {
        // Because cross platform lol
        let path: PathBuf = diag_path.replace("..", "assets").into();
        let dialogs: Vec<String> = make_dialog(path).unwrap();

        let anim = npc_anim();

        NPC {
            name: name.into(),
            dialogs,
            anim,
            hitbox,
            is_talking: false,
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        let dest_size = Some(self.anim.frame().dest_size * SCALE_FACTOR);
        let source = Some(self.anim.frame().source_rect);
        let draw_param = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        let pos = npc_draw_pos(self.hitbox);

        draw_texture_ex(texture, pos.x, pos.y, WHITE, draw_param)
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

        let pos = npc_draw_pos(self.hitbox);

        draw_texture_ex(texture, pos.x, pos.y - STANDARD_SQUARE, WHITE, draw_param);
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

fn make_dialog(path: PathBuf) -> Option<Vec<String>> {
    let mut dialog = vec![];

    let json_string = read_to_string(path).unwrap();
    let parsed: Value = serde_json::from_str(&json_string).unwrap();
    let arr = parsed["dialog"].as_array()?;

    for item in arr {
        dialog.push(item.as_str()?.to_string())
    }

    Some(dialog)
}

fn npc_draw_pos(rect: Rect) -> Vec2 {
    let x = rect.center().x - STANDARD_SQUARE / 2.;
    let y = rect.bottom() - STANDARD_SQUARE;
    vec2(x, y)
}
