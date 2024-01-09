use crate::camera::TERRAIN_TILE_SIZE;
use crate::logic::*;
use crate::npc::overlay_pos;
use crate::player::PIXEL;
use crate::ui::items::{Item, ItemEntity};
use macroquad::prelude::*;

use super::{GameSignal, Interactables};

pub enum ChestState {
    Closed,
    Opening(Timer),
    Opened,
}
pub struct Chest {
    content: Item,
    pos: Vec2,
    pub state: ChestState,
}

impl Chest {
    pub fn new(pos: Vec2, item: Item) -> Self {
        Self {
            content: item,
            pos,
            state: ChestState::Closed,
        }
    }

    fn texture(&self) -> Rect {
        let mut rect = Rect::new(0., 0., TERRAIN_TILE_SIZE, TERRAIN_TILE_SIZE);
        match self.state {
            ChestState::Closed => (),
            ChestState::Opening(timer) => {
                if timer.progress() < 0.5 {
                    rect.x = TERRAIN_TILE_SIZE;
                } else {
                    rect.x = 2. * TERRAIN_TILE_SIZE;
                }
            }
            ChestState::Opened => rect.x = 3. * TERRAIN_TILE_SIZE,
        }
        rect
    }

    fn state_management(&mut self, search_box: &Rect) {
        match self.state {
            ChestState::Opened | ChestState::Opening(_) => return,
            _ => (),
        }
        if !is_key_pressed(KeyCode::R) {
            return;
        }
        if !search_box.overlaps(&self.hitbox()) {
            return;
        }
        self.state = ChestState::Opening(Timer::new(0.5));
    }
}

impl Interactables for Chest {
    fn activate(&mut self, search_box: &Rect) -> Option<GameSignal> {
        self.state_management(search_box);
        let mut timer = match self.state {
            ChestState::Opening(timer) => timer,
            ChestState::Closed => return None,
            ChestState::Opened => return None,
        };
        timer.tick();

        if !timer.is_done() {
            self.state = ChestState::Opening(timer);
            return None;
        }
        self.state = ChestState::Opened;
        let item = ItemEntity::new(
            self.content.clone(),
            vec2(self.pos.x, self.pos.y + 10. * PIXEL),
        );
        return Some(GameSignal::SpawnItem(item));
    }

    fn draw(&self, texture: &Texture2D) {
        let hitbox = self.hitbox();
        let source = Some(self.texture());
        let dest_size = Some(vec2(TILE, TILE) * 0.8);

        let params = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };

        draw_texture_ex(texture, hitbox.x, hitbox.y, WHITE, params)
    }

    fn draw_overlay(&self, texture: &Texture2D) {
        if let ChestState::Opened = self.state {
            return;
        }
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

    fn hitbox(&self) -> Rect {
        Rect::new(
            self.pos.x + PIXEL,
            self.pos.y + PIXEL,
            18. * PIXEL,
            17. * PIXEL,
        )
    }
}
