use crate::logic::{STANDARD_SQUARE, TILE_SIZE};
use macroquad::{
    experimental::animation::{AnimatedSprite, Animation},
    prelude::*,
};

use crate::player::Player;

const MONSTER_VELOCITY: f32 = 100.;

#[derive(Clone)]
pub struct Monster {
    pub damage: f32,
    pub health: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub animation: AnimatedSprite,
    pub is_moving: bool,

}

impl Monster {
    pub fn slime(x: f32, y: f32) -> Self {
        Monster {
            animation: animation(),
            damage: 10.,
            health: 50.,
            pos_x: x,
            pos_y: y,
            is_moving: false
        }
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(self.pos_x, self.pos_y, STANDARD_SQUARE, STANDARD_SQUARE)
    }

    pub fn move_to_player(&mut self, player: &Player, delta_time: &f32) {
        let dist = ((player.pos_x - self.pos_x).powi(2) + (player.pos_y - self.pos_y).powi(2)).sqrt();
        if dist > 5. * STANDARD_SQUARE {
            self.is_moving = false;
            return;
        }
        self.is_moving = true;
        let movement_vector =
            vec2(self.pos_x - player.pos_x, self.pos_y - player.pos_y).normalize();

        self.pos_x -= MONSTER_VELOCITY * movement_vector.x * delta_time;
        self.pos_y -= MONSTER_VELOCITY * movement_vector.y * delta_time;
    }

    pub fn damage_player(&self, player: &mut Player, delta_time: &f32) {
        if let Some(_) = self.hitbox().intersect(player.hitbox()) {
            player.heath -= self.damage * delta_time
        }
    }

    pub fn update_anim(&mut self) {
        if self.is_moving {
            self.animation.set_animation(1)
        } else {
            self.animation.set_animation(0)
        }
        self.animation.update()
    }
}

fn animation() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[make_anim("idle", 0, 4), make_anim("moving", 1, 6)],
        true,
    )
}

fn make_anim(name: &str, row: u32, frames: u32) -> Animation {
    Animation {
        name: name.to_string(),
        row,
        frames,
        fps: 12,
    }
}
