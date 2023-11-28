use crate::player::Player;
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
        let player_pos = player.props.get_pos();
        match self {
            Monster::Slime(slime) => {
                slime.move_to_player(player_pos);
                slime.damage_player(player);
                slime.props.animation.update();
                slime.props.new_pos();
            }
        }
    }

    pub fn get_heath(&self) -> f32 {
        match self {
            Monster::Slime(slime) => {
                return slime.props.heath;
            }
        }
    }

    pub fn get_mut_heath(&mut self) -> &mut f32 {
        match self {
            Monster::Slime(slime) => {
                return &mut slime.props.heath;
            }
        }
    }

    pub fn get_hitbox(&self) -> Rect {
        match self {
            Monster::Slime(slime) => {
                return slime.hitbox();
            }
        }
    }

    pub fn slime() -> Self {
        Monster::Slime(Slime::from(300., 300.))
    }
}
