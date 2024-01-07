use std::collections::HashMap;

use macroquad::prelude::*;

use crate::camera::{draw_tiles, render_text, Utils};
use crate::logic::Game;
use crate::logic::*;

#[derive(Clone)]
pub struct Button {
    hitbox: Rect,
}
impl Button {
    fn is_clicked(&self, mouse_pos: Vec2) -> bool {
        if !is_mouse_button_down(MouseButton::Left) {
            return false;
        }
        if !self.hitbox.contains(mouse_pos) {
            return false;
        }
        true
    }

    fn size(size: Vec2) -> Self {
        let hitbox = Rect::new(0., 0., size.x, size.y);
        Self { hitbox }
    }

    fn center_on(self, pos: Vec2) -> Self {
        let x = pos.x - self.hitbox.w / 2.;
        let y = pos.y - self.hitbox.h / 2.;
        let hitbox = Rect::new(x, y, self.hitbox.w, self.hitbox.h);
        Self { hitbox }
    }
}

#[derive(Clone)]
pub struct MainMenu {
    buttons: HashMap<String, Button>,
}

impl MainMenu {
    fn update(&mut self, screen_box: Rect) {
        let size = vec2(5., 2.) * TILE;
        let mut pos = screen_box.center();
        pos.y -= TILE;
        let play_button = Button::size(size).center_on(pos);
        pos.y += 2. * TILE;
        let quit_button = Button::size(size).center_on(pos);
        self.buttons.insert("play".to_string(), play_button);
        self.buttons.insert("quit".to_string(), quit_button);
    }

    pub fn draw_background(&self, bg: &Texture2D, screen_box: Rect) {
        let dest_size = Some(screen_box.size());
        let params = DrawTextureParams {
            dest_size,
            ..Default::default()
        };

        draw_texture_ex(bg, screen_box.x, screen_box.y, WHITE, params)
    }

    pub fn draw_buttons(&self, texture: &Texture2D, font: &Font) {
        let mesh = button_mesh();
        for (name, button) in self.buttons.iter() {
            draw_tiles(&mesh, button.hitbox.point(), texture, None, TILE_SIZE);
            let params = TextParams {
                font: Some(font),
                font_size: 60,
                color: BLACK,
                ..Default::default()
            };
            render_text(button.hitbox.shift(-TILE * 1.75, -TILE/ 2.), name, params)
        }
    }
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
        }
    }
}

impl Game {
    fn tick_main_menu(&mut self, menu: &mut MainMenu) {
        menu.update(self.cam_box());
        let mouse_pos = self.get_mouse_pos();
        if menu.buttons["play"].is_clicked(mouse_pos) {
            self.state = GameState::Normal
        }
        if menu.buttons["quit"].is_clicked(mouse_pos) {
            self.quit = true
        }
    }

    pub fn tick_gui(&mut self, mut gui: GUIType) {
        match &mut gui {
            GUIType::Inventory => self.tick_inv(),
            GUIType::MainMenu(main_menu) => self.tick_main_menu(main_menu),
        }
        if let GameState::GUI(_) = self.state {
            self.state = GameState::GUI(gui)
        }
    }
}

fn button_mesh() -> Vec<Vec<u16>> {
    vec![vec![10, 11, 11, 11, 12], vec![22, 23, 23, 23, 24]]
}
