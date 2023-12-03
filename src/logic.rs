use std::collections::HashMap;

use crate::map::*;
use crate::monsters::Monster;
use crate::player::*;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

pub const TILE_SIZE: f32 = 24.;
pub const SCALE_FACTOR: f32 = 6.;
pub const STANDARD_SQUARE: f32 = TILE_SIZE * SCALE_FACTOR;
const KNOCKBACK: f32 = 10000.;

pub struct Game {
    pub player: Player,
    pub maps: HashMap<String, Area>,
    pub current_map: String,
    pub cam_offset_x: f32,
    pub cam_offset_y: f32,
    pub textures: Textures,
}

pub struct Textures {
    pub player: Texture2D,
    pub terrain: Texture2D,
    pub slime: Texture2D,
    pub mushroom: Texture2D,
}

impl Game {
    pub fn new(textures: Textures) -> Self {
        let mut area: HashMap<String, Area> = HashMap::new();
        // TODO unhardcode this value
        let current_map = "Village".to_string();
        let map_list = get_map_list();
        for map in map_list {
            let json_string = read_to_string(map).unwrap();
            let map_content = Area::from(&json_string);
            area.insert(map_content.0, map_content.1);
        }

        Game {
            player: Player::new(),
            maps: area,
            current_map,
            textures,
            cam_offset_x: 0.,
            cam_offset_y: 0.,
        }
    }

    fn key_event_handler(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            if self.player.attack_cooldown <= 0. {
                self.player.attack_cooldown = self.player.held_weapon.cooldown;
                self.damage_monster();
            }
        }
    }

    pub fn tick(&mut self) {
        self.new_camera_offset();
        self.key_event_handler();
        self.player.tick();
        self.wall_collision();
        self.move_through_gate();
        // self.go_through_gate();
        let current_map = self.maps.get_mut(&self.current_map).unwrap();

        for monster in current_map.enemies.iter_mut() {
            monster.get_mut().tick(&mut self.player);
        }

        for spawner in current_map.spawners.iter_mut() {
            spawner.tick(&mut current_map.enemies)
        }

        current_map.clean_up();
    }

    fn wall_collision(&mut self) {
        let walls = &self.maps[&self.current_map].walls;
        for wall in walls {
            let player_hitbox = self.player.hitbox();
            if let Some(rect) = wall.intersect(player_hitbox) {
                if rect.w < rect.h {
                    if player_hitbox.right() > wall.right() {
                        self.player.props.x += rect.w
                    } else {
                        self.player.props.x -= rect.w
                    }
                } else {
                    if player_hitbox.bottom() > wall.bottom() {
                        self.player.props.y += rect.h
                    } else {
                        self.player.props.y -= rect.h
                    }
                }
            }
        }
    }

    fn move_through_gate(&mut self) {
        let gates = self.maps[&self.current_map].gates.clone();
        for gate in gates {
            if let Some(_) = self.player.hitbox().intersect(gate.hitbox()) {
                self.move_map(&gate.command)
            }
        }
    }

    fn get_monster_list(&mut self) -> &mut Vec<Monster> {
        &mut self.maps.get_mut(&self.current_map).unwrap().enemies
    }

    fn damage_monster(&mut self) {
        let damage_zone = self.player.weapon_hitbox();
        let damage = self.player.held_weapon.base_damage;
        let player_pos = &self.player.props.get_pos();

        for monster in self.get_monster_list() {
            if let Some(_) = damage_zone.intersect(monster.get().hitbox()) {
                let monster_props = monster.get_mut().get_mut_props();
                let knockback = vec2(
                    monster_props.x - player_pos.x,
                    monster_props.y - player_pos.y,
                )
                .normalize()
                    * KNOCKBACK;

                monster_props.movement_vector = knockback;
                monster_props.heath -= damage
            }
        }
    }

    fn move_map(&mut self, command: &str) {
        let commands: Vec<&str> = command.split_whitespace().map(|x| x.trim()).collect();

        self.current_map = commands[0].to_string();
        self.player.props.x = commands[1].parse::<f32>().unwrap() * STANDARD_SQUARE;
        self.player.props.y = commands[2].parse::<f32>().unwrap() * STANDARD_SQUARE
    }
}

fn get_map_list() -> Vec<PathBuf> {
    let map_path = PathBuf::from("./assets/maps/");
    let maps = read_dir(map_path).unwrap();

    let mut return_vec: Vec<PathBuf> = vec![];
    for map in maps {
        let to_add: PathBuf = map.unwrap().path();
        if !to_add.to_str().unwrap().contains(".json") {
            continue;
        }
        return_vec.push(to_add)
    }
    return_vec
}

pub fn make_anim(name: &str, row: u32, frames: u32, fps: u32) -> Animation {
    Animation {
        name: name.to_string(),
        row,
        frames,
        fps,
    }
}
