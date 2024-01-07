use crate::camera::TERRAIN_TILE_SIZE;
use crate::logic::{Timer, TILE, TILE_SIZE};
use crate::npc::overlay_pos;
use crate::player::PIXEL;
use crate::ui::items::{Item, ItemEntity};
use macroquad::prelude::*;

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

    fn open(&mut self, search_box: Rect) {
        if !is_key_pressed(KeyCode::R) {
            return;
        }
        if !self.hitbox().overlaps(&search_box) {
            return;
        }
        match self.state {
            ChestState::Closed => (),
            _ => return,
        }
        self.state = ChestState::Opening(Timer::new(0.5));
    }

    pub fn tick(&mut self, search_box: Rect) -> Option<ItemEntity> {
        self.open(search_box);
        if let ChestState::Opening(mut timer) = self.state {
            if !timer.is_done() {
                timer.tick();
                self.state = ChestState::Opening(timer);
                return None;
            }
            let mut pos = self.pos;
            pos.y += 10. * PIXEL;
            let item = ItemEntity::new(self.content.clone(), pos);
            self.state = ChestState::Opened;
            return Some(item);
        }
        None
    }

    pub fn draw(&self, texture: &Texture2D) {
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

    pub fn hitbox(&self) -> Rect {
        Rect::new(
            self.pos.x + PIXEL,
            self.pos.y + PIXEL,
            18. * PIXEL,
            17. * PIXEL,
        )
    }

    pub fn draw_overlay(&self, texture: &Texture2D) {
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
}
