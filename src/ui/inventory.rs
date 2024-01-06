use crate::camera::{draw_tiles, render_text, Utils};
use crate::logic::{Game, TILE, TILE_SIZE};
use crate::player::{Player, PIXEL};
use macroquad::prelude::*;

use super::items::{Item, ItemType};

const ROW: u8 = 3;
const COL: f32 = 4.;
const SIZE: f32 = 140.;

#[derive(Clone)]
pub struct Inventory {
    pub content: [Option<Item>; 13],
    slot_hitboxes: [Rect; 13],
    holding: Option<Item>,
}

impl Inventory {
    pub fn empty() -> Self {
        #[rustfmt::skip]
        let content = [Some(Item::rusty_sword()), None, None, None, None, None, None, None, None, None, None, None, None]; // Idk
        Self {
            content,
            slot_hitboxes: [Rect::new(0., 0., 0., 0.); 13],
            holding: None,
        }
    }

    pub fn first_of_type(&self, item: &Item) -> Option<usize> {
        let mut index = 0;
        for slot in &self.content {
            if let Some(content) = slot {
                if content.name() == item.name() {
                    return Some(index);
                }
            } else {
                return Some(index);
            }
            index += 1;
        }
        return None;
    }

    pub fn append(&mut self, item: Item) {
        if let Some(slot) = self.first_of_type(&item) {
            if self.content[slot].is_none() {
                self.content[slot] = Some(item);
                return;
            }
            self.content[slot].as_mut().unwrap().count += 1
        }
    }
}

impl Game {
    pub fn show_inv(&mut self) {
        let screen_center = self.cam_box().center();
        let width = 720.;
        let height = 1296.;
        let margin = 50.;

        let (l_box, r_box) = dual_box(screen_center, width, height, margin);

        #[rustfmt::skip]
        let mesh = window_texture();
        draw_tiles(&mesh, l_box.point(), &self.textures["ui"], None, TILE_SIZE);
        draw_tiles(&mesh, r_box.point(), &self.textures["ui"], None, TILE_SIZE);
        self.player.update_inv(r_box, l_box);
        self.draw_slots();
        self.render_items();
        self.draw_description();
        self.inv_click_detection();
        self.draw_held_item()
    }

    pub fn get_mouse_pos(&self) -> Vec2 {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let offset_x = -self.cam_offset.x * screen_width;
        let offset_y = self.cam_offset.y * screen_height;
        let mouse = mouse_position_local();

        vec2(
            mouse.x * screen_width + offset_x,
            mouse.y * screen_height + offset_y,
        )
    }

    // I'm .... not exactly proud of this one
    fn draw_description(&self) {
        let mouse_pos = self.get_mouse_pos();
        let player_inv = &self.player.inventory;
        let mut index = 0;
        let mut diag_rect = Rect::new(
            mouse_pos.x + 4. * PIXEL,
            mouse_pos.y,
            4. * TILE,
            2. * TILE,
        );
        for slot in player_inv.slot_hitboxes {
            if !slot.contains(mouse_pos) {
                index += 1;
                continue;
            }
            let item = &player_inv.content[index];
            let item = match item {
                Some(item) => item,
                _ => return,
            };
            draw_tiles(
                &desc_diag(),
                mouse_pos,
                &self.textures["ui"],
                None,
                TILE_SIZE,
            );
            let name = item.name();
            let mut param = TextParams {
                color: BLACK,
                font: Some(&self.font),
                font_size: 48,
                ..Default::default()
            };
            render_text(diag_rect, name, param.clone());
            param.font_size = 28;
            let desc = item.description();
            diag_rect.y += 10. * PIXEL;
            render_text(diag_rect, desc, param);
            index += 1;
        }
    }

    fn inv_click_detection(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }
        let mouse_pos = self.get_mouse_pos();
        let player_inv = &mut self.player.inventory;

        let mut index = 0;
        while index < 12 {
            let slot = player_inv.slot_hitboxes[index];
            if !slot.contains(mouse_pos) {
                index += 1;
                continue;
            }
            // If an item is being held by the cursor
            if let Some(holding) = player_inv.holding.as_mut() {
                let slot = player_inv.content[index].as_mut();
                if let Some(slot) = slot {
                    if slot.is_same_type(holding) {
                        slot.count += holding.count;
                        player_inv.holding = None;
                        return;
                    }
                    (*slot, *holding) = (holding.clone(), slot.clone());
                    return;
                }
                player_inv.content[index] = Some(holding.clone());
                player_inv.holding = None;
                return;
            } else {
                player_inv.holding = player_inv.content[index].clone();
                player_inv.content[index] = None
            }
            index += 1;
        }

        // Weapon slot
        let weapon_slot = player_inv.slot_hitboxes[12];
        if !weapon_slot.contains(mouse_pos) {
            return;
        }
        if let Some(item) = &player_inv.holding {
            if item.class != ItemType::Weapon {
                return;
            }
            player_inv.content[12] = player_inv.holding.clone();
            player_inv.holding = None;
            return;
        }
        player_inv.holding = player_inv.content[12].clone();
        player_inv.content[12] = None;
    }

    fn draw_held_item(&self) {
        if let Some(item) = &self.player.inventory.holding {
            let mouse_pos = self.get_mouse_pos();
            let source = source_rect(Some(item));
            if source == None {
                error!("Like how even?")
            }
            let dest_size = vec2(SIZE, SIZE) * 0.8;
            let padd_x = (SIZE - dest_size.x) / 2.;
            let padd_y = (SIZE - dest_size.y) / 2.;

            let params = DrawTextureParams {
                source,
                dest_size: Some(dest_size),
                ..Default::default()
            };

            draw_texture_ex(
                &self.textures["ui"],
                mouse_pos.x + padd_x,
                mouse_pos.y + padd_y,
                WHITE,
                params,
            );
        }
    }

    fn render_items(&self) {
        let mut index = 0;
        let player_inv = &self.player.inventory;
        for slot in player_inv.slot_hitboxes {
            let item = &player_inv.content[index];
            if item.is_none() {
                index += 1;
                continue;
            }
            let source = source_rect(item.as_ref());
            let dest_size = vec2(SIZE, SIZE) * 0.8f32;
            let padd_x = (SIZE - dest_size.x) / 2.;
            let padd_y = (SIZE - dest_size.y) / 2.;

            let params = DrawTextureParams {
                source,
                dest_size: Some(dest_size),
                ..Default::default()
            };

            draw_texture_ex(
                &self.textures["ui"],
                slot.x + padd_x,
                slot.y + padd_y,
                WHITE,
                params,
            );

            index += 1;
            let item = item.as_ref().unwrap();
            if item.count == 1 {
                continue;
            }

            let params = TextParams {
                font: Some(&self.font),
                font_size: 24,
                ..Default::default()
            };

            render_text(slot.shift(-24., 0.), &format!("{}", item.count), params);
        }
    }

    fn draw_slots(&self) {
        for slot in self.player.inventory.slot_hitboxes {
            let params = param();
            draw_texture_ex(&self.textures["ui"], slot.x, slot.y, WHITE, params);
        }
    }
}

impl Player {
    fn update_inv(&mut self, right_box: Rect, left_box: Rect) {
        let margin = (right_box.w - (COL * SIZE)) / (COL + 1.);
        let height = right_box.h - ((COL - 1.) * SIZE + COL * margin);
        let max_col = COL as u8;
        let max_row = ROW;

        let mut index = 0;
        let starting_pos = vec2(right_box.left(), right_box.top() + height);

        for row in 0..max_row {
            let row = row as f32;
            for col in 0..max_col {
                let col = col as f32;
                let hitbox = Rect::new(
                    starting_pos.x + (col + 1.) * margin + col * SIZE,
                    starting_pos.y + (row + 1.) * margin + row * SIZE,
                    SIZE,
                    SIZE,
                );
                self.inventory.slot_hitboxes[index] = hitbox;
                index += 1
            }
        }

        let pos_x = left_box.left() + left_box.w / 2. - SIZE / 2.;
        let pos_y = left_box.top() + left_box.w * 1.3;
        self.inventory.slot_hitboxes[12] = Rect::new(pos_x, pos_y, SIZE, SIZE);
    }
}

fn dual_box(screen_center: Vec2, width: f32, height: f32, margin: f32) -> (Rect, Rect) {
    let ui_box = Rect::new(
        screen_center.x - margin / 2. - width,
        screen_center.y - height / 2.,
        width * 2. + margin,
        height,
    );

    let left_box = Rect::new(ui_box.left(), ui_box.top(), width, height);
    let right_box = Rect::new(ui_box.right() - width, ui_box.top(), width, height);

    (left_box, right_box)
}

fn param() -> DrawTextureParams {
    let dest_size = Some(vec2(SIZE, SIZE));
    let source = Some(Rect::new(0., 0., TILE_SIZE, TILE_SIZE));

    DrawTextureParams {
        dest_size,
        source,
        ..Default::default()
    }
}

pub fn source_rect(item: Option<&Item>) -> Option<Rect> {
    return match item?.name() {
        "Slime" => Some(Rect::new(0., TILE_SIZE * 2., TILE_SIZE, TILE_SIZE)),
        "Mushroom" => Some(Rect::new(TILE_SIZE, TILE_SIZE * 2., TILE_SIZE, TILE_SIZE)),
        "Rusty sword" => Some(Rect::new(0., 3. * TILE_SIZE, TILE_SIZE, TILE_SIZE)),
        _ => None,
    };
}

#[rustfmt::skip]
fn window_texture() -> Vec<Vec<u16>> {
    vec![
        vec![7 ,  8,  8,  8,  9],
        vec![19, 20, 20, 20, 21],
        vec![19, 20, 20, 20, 21],
        vec![19, 20, 20, 20, 21],
        vec![19, 20, 20, 20, 21],
        vec![19, 20, 20, 20, 21],
        vec![19, 20, 20, 20, 21],
        vec![19, 20, 20, 20, 21],
        vec![31, 32, 32, 32, 33],
    ]
}

#[rustfmt::skip]
fn desc_diag() -> Vec<Vec<u16>> {
    vec![
        vec![7 ,  8, 8,  9],
        vec![31, 32, 32, 33],
    ]
}
