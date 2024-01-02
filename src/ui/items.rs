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
    pub class: ItemType,
    pub count: u8,
}

#[derive(Debug)]
pub struct ItemEntity {
    pub item: Item,
    pub hitbox: Rect,
    pub should_delete: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    RegularItem,
    Weapon,
}

impl Item {
    pub fn slime(count: u8) -> Self {
        Item {
            name: "Slime".into(),
            value: 5,
            description: "It's quite slimy".into(),
            count,
            class: ItemType::RegularItem,
        }
    }

    pub fn mushroom(count: u8) -> Self {
        Item {
            name: "Mushroom".into(),
            value: 5,
            description: "Contrary to popular belief, eating this will not make you grow bigger"
                .into(),
            count,
            class: ItemType::RegularItem,
        }
    }

    pub fn rusty_sword() -> Self {
        Self {
            name: "Rusty sword".into(),
            description: "It is not the best sword out there".into(),
            count: 1,
            class: ItemType::Weapon,
            value: 10,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_same_type(&self, item: &Item) -> bool {
        self.name() == item.name()
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
