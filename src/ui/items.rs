use macroquad::prelude::*;
use std::rc::Rc;

use super::inventory::source_rect;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Item {
    // Because a guy named Logan Smith told me so
    name: Rc<str>,
    description: Rc<str>,
    value: u32,
}

#[derive(Debug)]
pub struct ItemEntity {
    pub item: Item,
    pub hitbox: Rect,
    pub should_delete: bool,
}

impl Item {
    pub fn slime() -> Self {
        Item {
            name: "Slime".into(),
            value: 5,
            description: "It's quite slimy".into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
impl ItemEntity {
    pub fn new(item: Item, pos: Vec2) -> Self {
        Self {
            item,
            hitbox: Rect::new(pos.x, pos.y, 100., 100.),
            should_delete: false,
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        let dest_size = Some(vec2(self.hitbox.w, self.hitbox.h));
        let source = source_rect(Some(&self.item));

        let params = DrawTextureParams {
            dest_size,
            source,
            ..Default::default()
        };

        draw_texture_ex(texture, self.hitbox.x, self.hitbox.y, WHITE, params)
    }
}
