use crate::player::{Player, Props};
use macroquad::{
    math::{Rect, Vec2},
    texture::Texture2D,
};
use mushroom::Mushroom;
use slime::Slime;

pub mod mushroom;
pub mod slime;
pub mod spawner;

#[derive(Clone)]
pub enum Monster {
    Slime(Slime),
    Mushroom(Mushroom),
}

impl Monster {
    pub fn get(&self) -> Box<&dyn IsAMonster> {
        return match self {
            Monster::Slime(slime) => Box::new(slime),
            Monster::Mushroom(mushroom) => Box::new(mushroom),
        };
    }
    pub fn get_mut(&mut self) -> Box<&mut dyn IsAMonster> {
        return match self {
            Monster::Slime(slime) => Box::new(slime),
            Monster::Mushroom(mushroom) => Box::new(mushroom),
        };
    }
}

pub trait IsAMonster {
    fn tick(&mut self, player: &mut Player);
    fn damage_player(&self, player: &mut Player);
    fn move_to(&mut self, player_pos: Vec2);
    fn hitbox(&self) -> Rect;
    fn draw(&self, texture: &Texture2D);
    fn change_anim(&mut self);
    fn get_props(&self) -> &Props;
    fn get_mut_props(&mut self) -> &mut Props;
}
