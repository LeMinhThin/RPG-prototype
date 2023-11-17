use std::collections::HashMap;

use crate::map::*;
use crate::player::*;
use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 24.;
pub const SCALE_FACTOR: f32 = 6.;
pub const STANDARD_SQUARE: f32 = TILE_SIZE * SCALE_FACTOR;

pub struct Game {
    pub player: Player,
    pub maps: HashMap<String, Area>,
    pub current_map: String,
    pub cam_offset_x: f32,
    pub cam_offset_y: f32,
    pub textures: Textures,
}

pub struct Textures {
    pub player: Texture2D
}

impl Game {
    pub fn new(textures: Texture2D) -> Self {
        let mut maps = HashMap::new();
        let map_path = get_map_list();

        // TODO unhardcode this value
        let current_map = "Village".to_string();
        for map in map_path {
            let map_content = Area::from(map);
            maps.insert(map_content.0, map_content.1);
        }

        let textures = pack_texture(textures);

        Game {
            player: Player::new(),
            maps,
            current_map,
            textures,
            cam_offset_x: 0.,
            cam_offset_y: 0.,
        }
    }

    fn key_event_handler(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            if self.player.attack_cooldown == 0 {
                self.player.attack_cooldown = self.player.held_weapon.cooldown;
                self.damage_monster();
            }
        }
    }

    pub fn tick(&mut self, delta_time: &f32) {
        self.new_camera_offset();
        self.key_event_handler();
        self.player.tick();
        self.wall_collision();
        self.move_through_gate();
        // self.go_through_gate();
        for monster in self
            .maps
            .get_mut(&self.current_map)
            .unwrap()
            .enemies
            .iter_mut()
        {
            monster.move_to_player(&self.player, delta_time);
            monster.damage_player(&mut self.player, delta_time);
        }

        self.kill_monster();
    }

    fn kill_monster(&mut self) {
        self.maps.get_mut(&self.current_map).unwrap().enemies = self.maps[&self.current_map]
            .enemies
            .iter()
            .filter(|x| x.health > 0.)
            .cloned()
            .collect()
    }

    fn wall_collision(&mut self) {
        let walls = self.maps[&self.current_map].walls.clone();
        for wall in walls {
            if let Some(rect) = wall.intersect(self.player.hitbox()) {
                // Top or bottom platform_collision
                let player_size = TILE_SIZE * SCALE_FACTOR;
                if rect.h < rect.w {
                    if wall.y + wall.h > self.player.pos_y + player_size {
                        self.player.pos_y = wall.y - player_size;
                    } else {
                        self.player.pos_y = wall.y + wall.h;
                    }
                } else {
                    if wall.x + wall.w > self.player.pos_x + player_size {
                        self.player.pos_x = wall.x - player_size;
                    } else {
                        self.player.pos_x = wall.x + wall.w
                    }
                }
            }
        }
    }

    fn move_through_gate(&mut self) {
        let gates = self.maps[&self.current_map].gates.clone();
        for gate in gates {
            if let Some(_) = self.player.hitbox().intersect(gate.hitbox()) {
                self.run_command(&gate.command)
            }
        }
    }

    fn damage_monster(&mut self) {
        let damage_zone = self.player.weapon_hitbox();
        let damage = self.player.held_weapon.base_damage;
        for monster in self
            .maps
            .get_mut(&self.current_map)
            .unwrap()
            .enemies
            .iter_mut()
        {
            if let Some(_) = damage_zone.intersect(monster.hitbox()) {
                monster.health -= damage;
            }
        }
    }

    fn run_command(&mut self, command: &str) {
        let commands: Vec<&str> = command.split_whitespace().collect();
        match commands[0] {
            "move" => match commands[1] {
                "player" => self.move_map(commands[2], (commands[3], commands[4])),
                _ => (),
            },
            _ => (),
        }
    }

    fn move_map(&mut self, to: &str, location: (&str, &str)) {
        self.current_map = to.to_string();
        self.player.pos_x = location.0.parse::<f32>().unwrap() * STANDARD_SQUARE;
        self.player.pos_y = location.1.parse::<f32>().unwrap() * STANDARD_SQUARE
    }
}

fn pack_texture(texture: Texture2D) -> Textures {
    Textures {
        player: texture
    }
}
