use macroquad::prelude::*;

pub mod chests;
pub use chests::*;

use crate::ui::items::ItemEntity;

#[derive(Clone)]
pub enum GameSignal {
    SpawnItem(ItemEntity),
}

pub trait Interactables {
    fn activate(&mut self, search_box: &Rect) -> Option<GameSignal>;
    fn draw(&self, texture: &Texture2D);
    fn draw_overlay(&self, texture: &Texture2D);
    fn hitbox(&self) -> Rect;
}
