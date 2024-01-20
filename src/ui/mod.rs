use macroquad::prelude::*;

use crate::logic::*;
use crate::player::*;
use crate::{GUIType, GameState};

pub mod death_screen;
pub mod inventory;
pub mod items;
pub mod main_menu;

pub use death_screen::*;
pub use main_menu::*;

#[derive(Clone, Debug)]
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

impl Game {
    pub fn hud(&self) {
        self.draw_health_bar();
    }

    fn draw_health_bar(&self) {
        let screen = self.cam_box();

        let health_percentage = self.player.props.health / PLAYER_HEALTH;
        draw_rectangle(
            screen.x + 3. * PIXEL,
            screen.y + 3. * PIXEL,
            66. * PIXEL * health_percentage,
            4. * PIXEL,
            RED,
        );

        let texture = &self.textures["ui"];
        let dest_size = Some(vec2(TILE * 3., TILE));
        let source = Some(Rect::new(0., TILE_SIZE, TILE_SIZE * 3., TILE_SIZE));
        let params = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(texture, screen.x, screen.y, WHITE, params)
    }

    fn tick_main_menu(&mut self, mut menu: MainMenu) -> GameState {
        menu.update(self.cam_box());
        let mouse_pos = self.get_mouse_pos();
        if menu.buttons["play"].is_clicked(mouse_pos) {
            return GameState::Normal;
        }
        if menu.buttons["quit"].is_clicked(mouse_pos) {
            return GameState::Quit;
        }
        GameState::GUI(GUIType::MainMenu(menu))
    }

    fn tick_death_screen(&mut self, mut menu: DeathScreen) -> GameState {
        menu.update(self.cam_box());
        let mouse_pos = self.get_mouse_pos();

        if menu.buttons["respawn"].is_clicked(mouse_pos) {
            let pos = self.player.spawn_loc.location;
            let map = self.player.spawn_loc.map.clone();

            let state = GameState::Transition(Transition::new(pos, map));
            self.player.state = PlayerState::Transition;
            self.player.props.health = PLAYER_HEALTH;
            return state;
        }
        if menu.buttons["menu"].is_clicked(mouse_pos) {
            return GameState::GUI(GUIType::MainMenu(MainMenu::new()));
        }
        return GameState::GUI(GUIType::DeathScreen(menu));
    }

    pub fn tick_gui(&mut self) {
        let gui = match &self.state {
            GameState::GUI(gui) => gui,
            _ => return,
        };
        self.state = match gui {
            GUIType::Inventory => {
                self.tick_inv();
                return;
            }
            GUIType::MainMenu(main_menu) => self.tick_main_menu(main_menu.clone()),
            GUIType::DeathScreen(death_screen) => self.tick_death_screen(death_screen.clone()),
        }
    }
}
