use crate::player::{Collidable, Player, Props};
use macroquad::prelude::*;
use spawner::SpawnerType;

pub mod mushroom;
pub mod slime;
pub mod spawner;

pub struct Monster(Box<dyn Entity>);

impl Monster {
    pub fn get(&self) -> &Box<(dyn Entity)> {
        match self {
            Monster(val) => return val,
        }
    }
    pub fn get_mut(&mut self) -> &mut Box<dyn Entity> {
        match self {
            Monster(val) => return val,
        }
    }
}

pub trait IsAMonster {
    fn tick(&mut self, player: &mut Player, walls: &[Rect]);
    fn damage_player(&self, player: &mut Player);
    fn move_to(&mut self, player_pos: Vec2);
    fn draw(&self, texture: &Texture2D);
    fn change_anim(&mut self);
    fn get_props(&self) -> &Props;
    fn get_mut_props(&mut self) -> &mut Props;
    fn tick_anim(&mut self);
    fn get_type(&self) -> SpawnerType;
}

pub trait Entity: IsAMonster + Collidable {}
