use crate::logic::Game;
use macroquad::prelude::*;

const COL: f32 = 4.;
const ROW: f32 = 3.;
const SIZE: f32 = 120.;

impl Game {
    pub fn show_inv(&self) {
        let screen_center = self.cam_box().center();
        let width = 700.;
        let height = 1200.;
        let margin = 50.;

        let (left_box, right_box) = dual_box(screen_center, width, height, margin);

        draw_rectangle(left_box.x, left_box.y, left_box.w, left_box.h, LIGHTGRAY);
        draw_rectangle(
            right_box.x,
            right_box.y,
            right_box.w,
            right_box.h,
            LIGHTGRAY,
        );
        draw_slots(right_box, self.get_mouse_pos());
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

fn draw_slots(container: Rect, mouse_pos: Vec2) {
    let margin_x = (container.w - (COL * SIZE)) / (COL + 1.);
    let margin_y = (container.h - (ROW * SIZE)) / (ROW + 1.) / 2.;
    let margin = min(margin_x, margin_y);

    let max_row = ROW as u8;
    let max_col = COL as u8;

    let starting_pos = vec2(container.right(), container.bottom());

    for row in 0..max_row {
        let row = row as f32;
        for col in 0..max_col {
            let col = col as f32;

            let rect = Rect::new(
                starting_pos.x - col * (margin + 1.) - (col + 1.) * SIZE - margin,
                starting_pos.y - row * (margin + 1.) - (row + 1.) * SIZE - margin,
                SIZE,
                SIZE,
            );

            if rect.contains(mouse_pos) {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, GREEN)
            } else {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, RED)
            }
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

// IDK why there isn't any min/max function for f32
fn min<T: std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x < y {
        return x;
    } else {
        return y;
    }
}
