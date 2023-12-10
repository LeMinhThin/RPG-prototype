use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin, Ui};

use crate::player::Player;

const COL: f32 = 9.;
const ROW: f32 = 2.;
const SIZE: f32 = 80.;
#[allow(dead_code)]
impl Player {
    pub fn show_inv(&mut self, texture: &Texture2D) {
        let skin = Skin {
            ..root_ui().default_skin()
        };

        root_ui().push_skin(&skin);

        let size = vec2(800., 500.);
        let inv_pos_x = screen_width() / 2. - size.x / 2.;
        let inv_pos_y = screen_height() / 2. - size.y / 2.;
        let position = vec2(inv_pos_x, inv_pos_y);

        let _margin_x = (size.x - (COL * SIZE)) / (COL + 1.);
        root_ui().window(hash!(), position, size, |ui| {
            ui.texture(texture.clone(), SIZE * 10., SIZE * 10.);
            widgets::Label::new("Hello World")
                .size(vec2(SIZE, SIZE))
                .ui(ui)
            //self.draw_inv(ui, position, margin_x);
        });

        root_ui().pop_skin();
    }
    fn draw_inv(&mut self, ui: &mut Ui, origin: Vec2, margin: f32) {
        let mut col = 0.;
        let mut row = 0.;
        let mut index = 0;
        let mut last_hover = vec![];
        let inv = &self.inventory.mouse_over;

        // So uhm basicly the last_hover_method only returns a bool, so I had to buffer the output
        // of the method in a vector, that why the code looks so messy.

        while row < ROW {
            while col < COL {
                let x = origin.x + (margin * (col + 1.) + SIZE * col);
                let y = origin.y + (margin * (row + 1.) + SIZE * row);

                let rect = Rect::new(x, y, SIZE, SIZE);
                if inv[index] {
                    ui.canvas().rect(rect, GREEN, GREEN);
                } else {
                    ui.canvas().rect(rect, RED, RED);
                }

                last_hover.push(ui.last_item_hovered());
                col += 1.;
                index += 1;
            }
            col = 0.;
            row += 1.
        }
        self.inventory.mouse_over = last_hover;
    }
}
