use macroquad::rand::{gen_range, rand};

use super::Monster;
use super::slime::Slime;

// The term "radius" being used here is technically wrong as it is more of a square than a circle but
// it would take an additional layer of complexity to fix this tiny bug, which I'm not very fond of
// dealing with so "radius" remains for now.
struct Spawner {
    kind: SpawnerType,
    spawn_radius: f32,
    max_mob: u32,
    cooldown: f32,
    x: f32,
    y: f32,

}

enum SpawnerType {
    Slime
}

impl Spawner {
    fn new(kind: SpawnerType, spawn_radius: f32, max_mob: u32, cooldown: f32, x: f32, y: f32) -> Self {
        Spawner {
            kind,
            spawn_radius,
            max_mob,
            cooldown,
            x,
            y
        }
    }

    fn spawn_mob(&mut self, monsters: &mut Vec<Monster>) {
        if self.cooldown > 0. {
            return;
        }
        if self.count_mob(&monsters) > self.max_mob {
            return;
        }
        let num_mobs = rand() % self.max_mob;

        for _ in 0..=num_mobs {
            let x_offset = gen_range(-self.spawn_radius, self.spawn_radius);
            let y_offset = gen_range(-self.spawn_radius, self.spawn_radius);
            
            match self.kind {
                SpawnerType::Slime => {
                    let new_mob = Monster::Slime(Slime::from(x_offset, y_offset));
                    monsters.push(new_mob);
                }
            }
        }
    }

    fn count_mob(&self, mobs: &Vec<Monster>) -> u32 {
        let mut num_mobs = 0;
        for mob in mobs {
            let mob_pos = mob.get_hitbox().center();

            let dist = ((self.x - mob_pos.x).powi(2) + (self.y - mob_pos.y).powi(2)).sqrt();
            if dist < self.spawn_radius {
                num_mobs += 1;
            }
        }
        num_mobs
    }
}
