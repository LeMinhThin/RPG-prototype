use crate::logic::{Game, TILE_SIZE};
use crate::player::Player;
use macroquad::prelude::*;

use super::items::Item;

const ROW: u8 = 3;
const COL: f32 = 4.;
const SIZE: f32 = 144.;

#[derive(Clone)]
pub struct Inventory {
    content: [Option<Item>; 13],
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
                None,
            ],
        }
    }
}

impl Game {
    pub fn show_inv(&self) {
        let screen_center = self.cam_box().center();
        let width = 700.;
        let height = 1200.;
        let margin = 50.;

        let (l_box, r_box) = dual_box(screen_center, width, height, margin);

        #[rustfmt::skip]
        draw_rectangle(l_box.x, l_box.y, l_box.w, l_box.h, LIGHTGRAY);
        draw_rectangle(r_box.x, r_box.y, r_box.w, r_box.h, LIGHTGRAY);
        draw_slots(r_box, &self.textures["ui"]);
        self.player.draw_items(r_box, &self.textures["ui"])
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
}

impl Player {
    fn draw_items(&self, window: Rect, texture: &Texture2D) {
        let margin = (window.w - (COL * SIZE)) / (COL + 1.);
        let height = window.h - ((COL - 1.) * SIZE + COL * margin);
        let starting_pos = vec2(window.left(), window.top() + height);

        let max_col = COL as u8;
        let max_row = ROW;
        let mut item = 0;

        for col in 0..max_col {
            let col = col as f32;
            for row in 0..max_row {
                let row = row as f32;
                let dest_size = vec2(SIZE, SIZE) * 0.8;
                let source = source_rect(self.inventory.content[item].as_ref());
                if source == None {
                    item += 1;
                    continue;
                }
                let params = DrawTextureParams {
                    dest_size: Some(dest_size),
                    source,
                    ..Default::default()
                };
                let pos = vec2(
                    starting_pos.x + (col + 1.) * margin + col * SIZE + dest_size.x / 8.,
                    starting_pos.y + (row + 1.) * margin + row * SIZE + dest_size.y / 8.,
                );
                draw_texture_ex(texture, pos.x, pos.y, WHITE, params);

                item += 1
            }
        }
    }
}

fn draw_slots(window: Rect, texture: &Texture2D) {
    let margin = (window.w - (COL * SIZE)) / (COL + 1.);
    let height = window.h - ((COL - 1.) * SIZE + COL * margin); // What
    let starting_pos = vec2(window.left(), window.top() + height);
    let max_col = COL as u8;
    let params = param();

    for col in 0..max_col {
        let col = col as f32;
        for row in 0..ROW {
            let row = row as f32;

            let pos = vec2(
                starting_pos.x + margin * (col + 1.) + col * SIZE,
                starting_pos.y + margin * (row + 1.) + row * SIZE,
            );

            draw_texture_ex(texture, pos.x, pos.y, WHITE, params.clone())
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
    return match item?.name().as_str() {
        "slime" => Some(Rect::new(0., TILE_SIZE * 2., TILE_SIZE, TILE_SIZE)),
        _ => None,
    };
}

/*
fn single_box(screen_center: Vec2, width: f32, height: f32) -> Rect {
    Rect::new(
        screen_center.x - width / 2.,
        screen_center.y - height / 2.,
        width,
        height,
    )
}
*/
