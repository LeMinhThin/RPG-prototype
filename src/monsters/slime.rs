use crate::logic::*;
use crate::player::{Collidable, Player, Props, INVUL_TIME};
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

use super::{Entity, IsAMonster};

const SLIME_HEATH: f32 = 50.;
const SLIME_SPEED: f32 = 150.;
const SLIME_MAX_TRACKING: f32 = 500.;

#[derive(Clone)]
pub struct Slime {
    pub props: Props,
    damage: f32,
}
impl IsAMonster for Slime {
    fn tick(&mut self, player: &mut Player, walls: &[Rect]) {
        let player_pos = player.props.get_pos();

        self.move_to(player_pos);
        self.damage_player(player);
        self.props.animation.update();
        self.props.new_pos();
        self.wall_collsion(walls);

        self.change_anim()
    }

    fn move_to(&mut self, player_pos: Vec2) {
        // May looks dawnting but it's just the Pythagoras theorem
        let dist =
            ((self.props.x - player_pos.x).powi(2) + (self.props.y - player_pos.y).powi(2)).sqrt();

        if dist > SLIME_MAX_TRACKING {
            return;
        }
        self.props.move_to(player_pos, SLIME_SPEED)
    }

    fn damage_player(&self, player: &mut Player) {
        if player.invul_time > 0. {
            return;
        }
        let self_hitbox = self.hitbox();
        let player_hitbox = player.hitbox();

        if let Some(_) = self_hitbox.intersect(player_hitbox) {
            player.props.heath -= self.damage;
            player.invul_time = INVUL_TIME
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
        let speed = self.props.movement_vector.length();

        if speed < 1. {
            self.props.animation.set_animation(0);
        } else {
            self.props.animation.set_animation(1);
        }
    }

    fn get_props(&self) -> &Props {
        &self.props
    }

    fn get_mut_props(&mut self) -> &mut Props {
        &mut self.props
    }
}

impl Slime {
    pub fn from(x: f32, y: f32) -> Self {
        let animation = slime_animations();
        let props = Props::from(x, y, SLIME_HEATH, animation);
        Slime { props, damage: 10. }
    }
}

impl Collidable for Slime {
    fn pos(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.props.x, &mut self.props.y)
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

impl Entity for Slime {}

fn slime_animations() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[make_anim("idle", 0, 4, 8), make_anim("moving", 1, 6, 8)],
        true,
    )
}
