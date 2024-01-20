use macroquad::prelude::*;

use super::inventory::source_rect;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Item {
    value: u32,
    pub kind: ItemID,
    pub class: ItemType,
    pub count: u8,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum ItemID {
    Slime,
    Mushroom,
    RustySword,
    BlackSword,
}

impl Item {
    pub fn slime(count: u8) -> Self {
        Item {
            kind: ItemID::Slime,
            value: 5,
            count,
            class: ItemType::RegularItem,
        }
    }

    pub fn mushroom(count: u8) -> Self {
        Item {
            kind: ItemID::Mushroom,
            value: 5,
            count,
            class: ItemType::RegularItem,
        }
    }

    pub fn rusty_sword() -> Self {
        Self {
            kind: ItemID::RustySword,
            count: 1,
            class: ItemType::Weapon,
            value: 10,
        }
    }

    pub fn black_sword() -> Self {
        Self {
            kind: ItemID::BlackSword,
            count: 1,
            class: ItemType::Weapon,
            value: 20,
        }
    }

    pub fn name(&self) -> &str {
        match self.kind {
            ItemID::Mushroom => "Nấm Đỏ",
            ItemID::Slime => "Chất nhầy",
            ItemID::RustySword => "Kiếm rỉ sét",
            ItemID::BlackSword => "Hắc kiếm",
        }
    }

    pub fn description(&self) -> &str {
        match self.kind {
            ItemID::Mushroom => "Trái với một tựa game nổi tiếng nào đó, việc tiêu thụ loại nấm này sẽ không làm bạn cao lên",
            ItemID::Slime => "Nó khá nhầy nhụa",
            ItemID::BlackSword => "Một thanh kiếm với màu đen huyền bí",
            ItemID::RustySword => "Một thanh kiếm đã bị rỉ sét"
        }
    }

    pub fn is_same_type(&self, item: &Item) -> bool {
        self.kind == item.kind
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
