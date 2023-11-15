use crate::logic::STANDARD_SQUARE;
use macroquad::prelude::*;

use crate::player::Player;

const MONSTER_VELOCITY: f32 = 100.;

#[derive(Clone, Debug)]
pub struct Monster {
    pub damage: f32,
    pub health: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Monster {
    pub fn cube(x: f32, y: f32) -> Self {
        Monster {
            damage: 10.,
            health: 50.,
            pos_x: x,
            pos_y: y,
        }
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(self.pos_x, self.pos_y, STANDARD_SQUARE, STANDARD_SQUARE)
    }

    pub fn move_to_player(&mut self, player: &Player, delta_time: &f32) {
        let dist_x = player.pos_x - self.pos_x;
        let dist_y = player.pos_y - self.pos_y;
        if dist_x == 0. && dist_y == 0. {
            return;
        }
        let coefficent;
        if dist_x < 0. {
            coefficent = -1.
        } else {
            coefficent = 1.
        }
        let angle = (dist_y / dist_x).atan();
        self.pos_x += MONSTER_VELOCITY * angle.cos() * coefficent * delta_time;
        self.pos_y += MONSTER_VELOCITY * angle.sin() * coefficent * delta_time;
    }

    pub fn damage_player(&self, player: &mut Player, delta_time: &f32) {
        if let Some(_) = self.hitbox().intersect(player.hitbox()) {
            player.heath -= self.damage * delta_time
        }
    }
}
