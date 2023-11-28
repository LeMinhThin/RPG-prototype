use crate::logic::{make_anim, STANDARD_SQUARE, TILE_SIZE};
use crate::player::{Player, Props};
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

const SLIME_HEATH: f32 = 50.;
const SLIME_SPEED: f32 = 150.;

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

#[derive(Clone)]
pub struct Slime {
    pub props: Props,
    damage: f32,
}

impl Slime {
    fn from(x: f32, y: f32) -> Self {
        let animation = slime_animations();
        let props = Props::from(x, y, SLIME_HEATH, animation);
        Slime { props, damage: 10. }
    }

    fn move_to_player(&mut self, player_pos: Vec2) {
        self.props.move_to(player_pos, SLIME_SPEED)
    }

    fn damage_player(&self, player: &mut Player) {
        let self_hitbox = self.hitbox();
        let player_hitbox = player.hitbox();

        if let Some(_) = self_hitbox.intersect(player_hitbox) {
            player.props.heath -= self.damage * get_frame_time()
        }
    }

    fn hitbox(&self) -> Rect {
        Rect {
            x: self.props.x,
            y: self.props.y,
            w: STANDARD_SQUARE,
            h: STANDARD_SQUARE,
        }
    }
}

fn slime_animations() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[make_anim("idle", 0, 4), make_anim("moving", 1, 6)],
        true,
    )
}
