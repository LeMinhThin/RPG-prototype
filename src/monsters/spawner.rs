use macroquad::prelude::*;
use macroquad::rand::*;

use crate::logic::Timer;
use crate::map::Monster;

use super::mushroom::Mushroom;
use super::slime::Slime;

// The term "radius" being used here is technically incorrect as it is more of a square than a circle but
// it would take an additional layer of complexity to fix this tiny bug, which I'm not very fond of
// dealing with so "radius" remains for now.
#[derive(Debug)]
pub struct Spawner {
    kind: MobType,
    pub spawn_radius: f32,
    max_mob: u32,
    timer: Timer,
    pos: Vec2,
}

#[derive(PartialEq, Debug)]
pub enum MobType {
    Slime,
    Mushroom,
}

impl Spawner {
    pub fn new(
        kind: MobType,
        spawn_radius: f32,
        max_mob: u32,
        max_cooldown: f32,
        pos: Vec2,
    ) -> Self {
        let mut timer = Timer::new(max_cooldown);
        timer.time = 0.;
        Spawner {
            kind,
            spawn_radius,
            max_mob,
            timer,
            pos,
        }
    }

    pub fn tick(&mut self, monsters: &mut Vec<Monster>) {
        self.timer.tick();
        if !self.timer.is_done() {
            return;
        }
        self.timer.repeat();

        if self.count_mob(&monsters) > self.max_mob {
            return;
        }

        let num_mobs = rand() % self.max_mob;

        for _ in 0..num_mobs {
            let x_offset = gen_range(-self.spawn_radius, self.spawn_radius);
            let y_offset = gen_range(-self.spawn_radius, self.spawn_radius);

            match self.kind {
                MobType::Slime => {
                    let slime = Slime::from(vec2(self.pos.x + x_offset, self.pos.y + y_offset));
                    monsters.push(Box::new(slime));
                }
                MobType::Mushroom => {
                    let mushroom =
                        Mushroom::from(vec2(self.pos.x + x_offset, self.pos.y + y_offset));
                    monsters.push(Box::new(mushroom))
                }
            }
        }
    }

    fn count_mob(&self, mobs: &[Monster]) -> u32 {
        let spawner_detect_box = Rect::new(
            self.pos.x - self.spawn_radius,
            self.pos.y - self.spawn_radius,
            self.spawn_radius * 2.,
            self.spawn_radius * 2.,
        );
        let mut num_mobs = 0;
        for mob in mobs {
            let mob_hitbox = mob.hitbox();

            if !self.is_same_type(mob) {
                continue;
            }

            if spawner_detect_box.overlaps(&mob_hitbox) {
                num_mobs += 1;
            }
        }
        num_mobs
    }

    fn is_same_type(&self, mob: &Monster) -> bool {
        let mob_type = mob.get_type();
        mob_type == self.kind
    }
}
