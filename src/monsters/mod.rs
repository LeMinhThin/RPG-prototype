use crate::player::{Player, Props};
use macroquad::math::Rect;
use slime::Slime;

pub mod slime;
pub mod spawner;

#[derive(Clone)]
pub enum Monster {
    Slime(Slime),
}

impl Monster {
    pub fn tick(&mut self, player: &mut Player) {
        match self {
            Monster::Slime(slime) => slime.tick(player),
        }
    }

    pub fn get_props(&self) -> &Props {
        return match self {
            Monster::Slime(slime) => &slime.props,
        };
    }

    pub fn get_mut_props(&mut self) -> &mut Props {
        return match self {
            Monster::Slime(slime) => &mut slime.props,
        };
    }

    pub fn slime() -> Self {
        Monster::Slime(Slime::from(300., 300.))
    }

    pub fn get_hitbox(&self) -> Rect {
        match self {
            Monster::Slime(slime) => slime.hitbox(),
        }
    }
}
