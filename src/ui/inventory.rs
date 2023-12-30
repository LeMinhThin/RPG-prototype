use crate::camera::{draw_tiles, render_text};
use crate::logic::{Game, STANDARD_SQUARE, TILE_SIZE};
use crate::player::{Player, PIXEL};
use macroquad::prelude::*;

use super::items::Item;

const ROW: u8 = 3;
const COL: f32 = 4.;
const SIZE: f32 = 140.;

#[derive(Clone)]
pub struct Inventory {
    content: [Option<Item>; 12],
    slot_hitboxes: [Rect; 12],
}

impl Inventory {
    pub fn empty() -> Self {
        Self {
            // Idk man
            content: [
                Some(Item::slime()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            slot_hitboxes: [Rect::new(0., 0., 0., 0.); 12],
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
        draw_rectangle(l_box.x, l_box.y, l_box.w, l_box.h, LIGHTGRAY);
        draw_tiles(&mesh, r_box.point(), &self.textures["ui"], None, TILE_SIZE);
        //draw_slots(r_box, &self.textures["ui"]);
        self.player.update_inv(r_box);
        self.player.draw_slots(&self.textures["ui"]);
        self.player.draw_items(&self.textures["ui"]);
        self.draw_description()
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
            mouse_pos.x + 8. * PIXEL,
            mouse_pos.y,
            3. * STANDARD_SQUARE,
            2. * STANDARD_SQUARE,
        );
        for slot in player_inv.slot_hitboxes {
            if !slot.contains(mouse_pos) {
                index += 1;
                continue;
            }
            let item = &player_inv.content[index];
            if let Some(item) = item {
                draw_tiles(
                    &desc_diag(),
                    mouse_pos,
                    &self.textures["ui"],
                    None,
                    TILE_SIZE,
                );
                let name = item.name();
                let mut param = TextParams {
                    color: WHITE,
                    font: Some(&self.font),
                    font_size: 48,
                    ..Default::default()
                };
                render_text(diag_rect, name, param.clone());
                param.font_size = 32;
                let desc = item.description();
                diag_rect.y += 10. * PIXEL;
                diag_rect.x -= 4. * PIXEL;
                render_text(diag_rect, desc, param);

            }
        }
    }
}

impl Player {
    fn update_inv(&mut self, window: Rect) {
        let margin = (window.w - (COL * SIZE)) / (COL + 1.);
        let height = window.h - ((COL - 1.) * SIZE + COL * margin);
        let max_col = COL as u8;
        let max_row = ROW;

        let mut index = 0;
        let starting_pos = vec2(window.left(), window.top() + height);

        for col in 0..max_col {
            let col = col as f32;
            for row in 0..max_row {
                let row = row as f32;
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
    }

    fn draw_items(&self, texture: &Texture2D) {
        let mut index = 0;
        for slot in self.inventory.slot_hitboxes {
            let source = source_rect(self.inventory.content[index].as_ref());
            if source == None {
                index += 1;
                continue;
            }
            let dest_size = vec2(SIZE, SIZE) * 0.8f32;
            let padd_x = (SIZE - dest_size.x) / 2.;
            let padd_y = (SIZE - dest_size.y) / 2.;

            let params = DrawTextureParams {
                source,
                dest_size: Some(dest_size),
                ..Default::default()
            };

            draw_texture_ex(texture, slot.x + padd_x, slot.y + padd_y, WHITE, params);
            index += 1;
        }
    }

    fn draw_slots(&self, texture: &Texture2D) {
        for slot in self.inventory.slot_hitboxes {
            let params = param();
            draw_texture_ex(texture, slot.x, slot.y, WHITE, params);
        }
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

fn source_rect(item: Option<&Item>) -> Option<Rect> {
    return match item?.name() {
        "Slime" => Some(Rect::new(0., TILE_SIZE * 2., TILE_SIZE, TILE_SIZE)),
        _ => None,
    };
}

#[rustfmt::skip]
fn window_texture() -> Vec<Vec<u16>> {
    vec![
        vec![4 ,  5,  5,  5,  6],
        vec![16, 17, 17, 17, 18],
        vec![16, 17, 17, 17, 18],
        vec![16, 17, 17, 17, 18],
        vec![16, 17, 17, 17, 18],
        vec![16, 17, 17, 17, 18],
        vec![16, 17, 17, 17, 18],
        vec![16, 17, 17, 17, 18],
        vec![28, 29, 29, 29, 30],
    ]
}

#[rustfmt::skip]
fn desc_diag() -> Vec<Vec<u16>> {
    vec![
        vec![7 ,  8,  9],
        vec![31, 32, 33],
    ]
}
