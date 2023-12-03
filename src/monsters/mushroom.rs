use crate::logic::*;
use crate::player::*;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;

use super::IsAMonster;

const MUSHROOM_HEALTH: f32 = 20.;
const MUSHROOM_MAX_TRACK: f32 = 200.;
const MUSHROOM_SPEED: f32 = 100.;

#[derive(Clone)]
pub struct Mushroom {
    pub props: Props,
    damage: f32,
}

impl IsAMonster for Mushroom {
    fn tick(&mut self, player: &mut Player) {
        let player_pos = player.props.get_pos();
        self.move_to(player_pos);
        self.damage_player(player);
        self.props.animation.update();
        self.props.new_pos();

        self.change_anim();
    }

    fn move_to(&mut self, player_pos: Vec2) {
        let dist = self.props.get_pos().distance(player_pos);
        if dist > MUSHROOM_MAX_TRACK {
            return;
        }

        self.props.move_to(player_pos, MUSHROOM_SPEED);
    }

    fn damage_player(&self, player: &mut Player) {
        if player.invul_time > 0. {
            return;
        }

        if let Some(_) = self.hitbox().intersect(player.hitbox()) {
            player.props.heath -= self.damage;
            player.invul_time = INVUL_TIME;
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

    fn draw(&self, texture: &Texture2D) {
        let dest_size = Some(self.props.animation.frame().dest_size * SCALE_FACTOR);
        let draw_param = DrawTextureParams {
            source: Some(self.props.animation.frame().source_rect),
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(&texture, self.props.x, self.props.y, WHITE, draw_param);
    }

    fn change_anim(&mut self) {
        let movement_vector = self.props.movement_vector;

        if movement_vector.length() < 1. {
            self.props.animation.set_animation(1)
        } else if movement_vector.x >= 0. {
            self.props.animation.set_animation(0)
        } else {
            self.props.animation.set_animation(2)
        }
    }

    fn get_props(&self) -> &Props {
        &self.props
    }

    fn get_mut_props(&mut self) -> &mut Props {
        &mut self.props
    }
}

impl Mushroom {
    pub fn from(x: f32, y: f32) -> Self {
        let animation = mushroom_animation();
        let props = Props::from(x, y, MUSHROOM_HEALTH, animation);

        Mushroom { props, damage: 5. }
    }
}

fn mushroom_animation() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[
            make_anim("walk right", 0, 2, 4),
            make_anim("idle", 1, 2, 4),
            make_anim("walk left", 2, 2, 4),
        ],
        true,
    )
}