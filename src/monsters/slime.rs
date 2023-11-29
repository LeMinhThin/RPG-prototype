use crate::logic::{make_anim, STANDARD_SQUARE, TILE_SIZE};
use crate::player::{Player, Props};
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

const SLIME_HEATH: f32 = 50.;
const SLIME_SPEED: f32 = 150.;
const SLIME_MAX_TRACKING: f32 = 500.;

#[derive(Clone)]
pub struct Slime {
    pub props: Props,
    damage: f32,
}

impl Slime {
    pub fn from(x: f32, y: f32) -> Self {
        let animation = slime_animations();
        let props = Props::from(x, y, SLIME_HEATH, animation);
        Slime { props, damage: 10. }
    }

    pub fn move_to_player(&mut self, player_pos: Vec2) {
        let dist = ((self.props.x - player_pos.x).powi(2) + (self.props.y - player_pos.y).powi(2)).sqrt();

        if dist > SLIME_MAX_TRACKING {
            return;
        }
        self.props.move_to(player_pos, SLIME_SPEED)
    }

    pub fn damage_player(&self, player: &mut Player) {
        let self_hitbox = self.hitbox();
        let player_hitbox = player.hitbox();

        if let Some(_) = self_hitbox.intersect(player_hitbox) {
            player.props.heath -= self.damage * get_frame_time()
        }
    }

    pub fn hitbox(&self) -> Rect {
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
