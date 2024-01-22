use super::*;
use crate::npc::overlay_pos;
use crate::player::PIXEL;
use crate::Rc;
use crate::{TILE, TILE_SIZE};

pub struct Door {
    pub hitbox: Rect,
    pub map: Rc<str>,
    pub location: Vec2,
}

impl Door {
    pub fn new(hitbox: Rect, map: &str, location: Vec2) -> Self {
        Self {
            hitbox,
            map: map.into(),
            location,
        }
    }
}

impl Interactables for Door {
    fn draw(&self, _texture: &Texture2D) {}

    fn hitbox(&self) -> Rect {
        self.hitbox
    }

    fn activate(&mut self, search_box: &Rect) -> Option<GameSignal> {
        if !is_key_pressed(KeyCode::R) {
            return None;
        }

        if !search_box.overlaps(&self.hitbox) {
            return None;
        }

        let trans = Transition::new(self.location, self.map.clone());
        return Some(GameSignal::MovePlayer(trans));
    }

    fn draw_overlay(&self, texture: &Texture2D) {
        let dest_size = Some(vec2(TILE, TILE));
        let source = Some(Rect::new(TILE_SIZE, 0., TILE_SIZE, TILE_SIZE));

        let draw_param = DrawTextureParams {
            dest_size,
            source,
            ..Default::default()
        };
        let pos = overlay_pos(self.hitbox());

        draw_texture_ex(texture, pos.x + 3. * PIXEL, pos.y - TILE, WHITE, draw_param);
    }
}
