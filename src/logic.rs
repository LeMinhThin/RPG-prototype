use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use crate::map::*;
use crate::monsters::Monster;
use crate::player::*;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 24.;
pub const SCALE_FACTOR: f32 = 6.;
pub const STANDARD_SQUARE: f32 = TILE_SIZE * SCALE_FACTOR;
const KNOCKBACK: f32 = 10000.;

type Textures = HashMap<String, Texture2D>;

pub struct Game {
    pub player: Player,
    pub maps: HashMap<String, Area>,
    pub current_map: String,
    pub cam_offset_x: f32,
    pub cam_offset_y: f32,
    pub textures: HashMap<String, Texture2D>,
    pub current_state: GameState,
    pub font: Font,
}

#[derive(PartialEq)]
pub enum GameState {
    Normal,
    GUI,
    Talking(usize),
}

impl Game {
    pub fn new(textures: Textures, font: Font) -> Self {
        let mut area: HashMap<String, Area> = HashMap::new();
        // TODO unhardcode this value
        let current_map = "Village".to_string();

        let map_list = get_path("assets/maps/", ".json");
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
            current_state: GameState::Normal,
            font,
        }
    }

    fn key_event_handler(&mut self) {
        if is_key_pressed(KeyCode::E) {
            self.current_state = GameState::GUI
        }

        if is_key_pressed(KeyCode::Escape) {
            if self.current_state == GameState::GUI {
                self.current_state = GameState::Normal
            }
        }

        if is_key_pressed(KeyCode::R) {
            if let GameState::Talking(_) = self.current_state {
                return;
            }
            self.talk_to_npc();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.current_state = match self.current_state {
                GameState::Talking(x) => GameState::Talking(x + 1),
                _ => return,
            }
        }
    }

    // This could use a better name
    fn talk_to_npc(&mut self) {
        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        let npcs = &mut current_map.npcs;
        let player_pos = self.player.pos();

        for npc in npcs.iter_mut() {
            if npc.pos().distance(player_pos) < STANDARD_SQUARE {
                self.current_state = GameState::Talking(0);
                npc.is_talking = true
            }
        }
    }

    // I can not think of a better name for the love of god
    fn conversation(&mut self) {
        let index = match self.current_state {
            GameState::Talking(x) => x,
            _ => return,
        };
        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        let talking_npc = current_map
            .npcs
            .iter_mut()
            .find(|npc| npc.is_talking)
            .unwrap();

        match talking_npc.dialogs.get(index) {
            Some(_) => return,
            None => {
                self.current_state = GameState::Normal;
                talking_npc.is_talking = false
            }
        }
    }

    pub fn tick(&mut self) {
        self.new_camera_offset();
        self.key_event_handler();
        self.anim_tick();
        match self.current_state {
            GameState::Talking(_) => {
                self.conversation();
                return;
            }
            GameState::GUI => return,
            GameState::Normal => (),
        }
        self.move_through_gate();

        self.damage_monster();
        let current_map = self.maps.get_mut(&self.current_map).unwrap();

        self.player.tick(&current_map.walls);
        for monster in current_map.enemies.iter_mut() {
            monster.get_mut().tick(&mut self.player, &current_map.walls);
        }
        for spawner in current_map.spawners.iter_mut() {
            spawner.tick(&mut current_map.enemies)
        }

        current_map.clean_up();
    }

    fn move_through_gate(&mut self) {
        let gates = self.maps[&self.current_map].gates.clone();
        for gate in gates {
            if let Some(_) = self.player.hitbox().intersect(gate.hitbox()) {
                self.move_map(&gate.command)
            }
        }
    }

    fn anim_tick(&mut self) {
        self.player.props.animation.update();

        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        for monster in current_map.enemies.iter_mut() {
            monster.get_mut().tick_anim();
        }

        for npc in current_map.npcs.iter_mut() {
            npc.anim.update()
        }
    }

    fn get_monster_list(&mut self) -> &mut [Monster] {
        &mut self.maps.get_mut(&self.current_map).unwrap().enemies
    }

    fn damage_monster(&mut self) {
        if !self.player.should_attack() {
            return;
        }
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

pub fn get_path(dir: &str, file_type: &str) -> Vec<PathBuf> {
    let maps = read_dir(dir).unwrap();

    let mut return_vec: Vec<PathBuf> = vec![];
    for map in maps {
        let to_add: PathBuf = map.unwrap().path();
        if !to_add.to_str().unwrap().contains(file_type) {
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
