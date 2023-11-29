use macroquad::rand::{gen_range, rand};
use macroquad::time::get_frame_time;
use macroquad::math::Rect;

use super::slime::Slime;
use super::Monster;

// The term "radius" being used here is technically wrong as it is more of a square than a circle but
// it would take an additional layer of complexity to fix this tiny bug, which I'm not very fond of
// dealing with so "radius" remains for now.
pub struct Spawner {
    kind: SpawnerType,
    pub spawn_radius: f32,
    max_mob: u32,
    cooldown: f32,
    max_cooldown: f32,
    pub x: f32,
    pub y: f32,
}

pub enum SpawnerType {
    Slime,
}

impl Spawner {
    pub fn new(
        kind: SpawnerType,
        spawn_radius: f32,
        max_mob: u32,
        max_cooldown: f32,
        x: f32,
        y: f32,
    ) -> Self {
        Spawner {
            kind,
            spawn_radius,
            max_mob,
            max_cooldown,
            cooldown: max_cooldown,
            x,
            y,
        }
    }

    pub fn tick(&mut self, monsters: &mut Vec<Monster>) {
        if self.cooldown > 0. {
            self.cooldown -= get_frame_time();
            return;
        }
        self.cooldown = self.max_cooldown;
        if self.count_mob(&monsters) > self.max_mob {
            return;
        }
        let num_mobs = rand() % self.max_mob;

        for _ in 0..=num_mobs {
            let x_offset = gen_range(-self.spawn_radius, self.spawn_radius);
            let y_offset = gen_range(-self.spawn_radius, self.spawn_radius);

            match self.kind {
                SpawnerType::Slime => {
                    let new_mob = Monster::Slime(Slime::from(self.x + x_offset, self.y + y_offset));
                    monsters.push(new_mob);
                }
            }
        }
    }

    fn count_mob(&self, mobs: &Vec<Monster>) -> u32 {
        let spawner_detect_box = Rect::new(self.x - self.spawn_radius, self.y - self.spawn_radius, self.spawn_radius * 2., self.spawn_radius * 2.);
        let mut num_mobs = 0;
        for mob in mobs {
            let mob_hitbox = mob.get_hitbox();

            if let Some(_) = spawner_detect_box.intersect(mob_hitbox) {
                num_mobs += 1;
            }
        }
        num_mobs
    }
}
