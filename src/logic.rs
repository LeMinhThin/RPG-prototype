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
pub const KNOCKBACK: f32 = 5000.;

pub type Textures = HashMap<String, Texture2D>;
pub type Maps = HashMap<String, Area>;

pub struct Game {
    pub player: Player,
    pub maps: Maps,
    pub current_map: String,
    pub cam_offset: Vec2,
    pub textures: Textures,
    pub state: GameState,
    pub font: Font,
}

#[derive(PartialEq)]
pub enum GameState {
    Normal,
    GUI,
    Talking(usize, usize),
    Transition(Timer, bool),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Timer {
    pub time: f32,
    pub duration: f32,
}

impl Timer {
    pub fn new(dur: f32) -> Self {
        Timer {
            time: dur,
            duration: dur,
        }
    }

    pub fn tick(&mut self) {
        self.time -= get_frame_time();
    }

    pub fn is_done(&self) -> bool {
        self.time < 0.
    }

    pub fn repeat(&mut self) {
        self.time = self.duration
    }
}

impl Game {
    pub fn new(textures: Textures, font: Font) -> Self {
        let mut area: Maps = HashMap::new();
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
            cam_offset: vec2(0., 0.),
            state: GameState::Normal,
            font,
        }
    }

    fn key_event_handler(&mut self) {
        if is_key_pressed(KeyCode::E) {
            self.state = GameState::GUI
        }

        if is_key_pressed(KeyCode::Escape) {
            if self.state == GameState::GUI {
                self.state = GameState::Normal
            }
        }

        if is_key_pressed(KeyCode::R) {
            if let GameState::Talking(..) = self.state {
                return;
            }
            self.talk_to_npc();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            match self.state {
                GameState::Talking(mut line, mut char) => {
                    let npcs = &self.maps[&self.current_map].npcs;
                    let npc = npcs.iter().find(|npc| npc.is_talking).unwrap();

                    let max_char = npc.dialogs[line].len() - 1;
                    if char >= max_char {
                        line += 1;
                        char = 0;
                    } else {
                        char = max_char;
                    }
                    self.state = GameState::Talking(line, char)
                }
                GameState::Normal => {
                    self.player.face(self.get_mouse_pos());
                    self.damage_monster();
                    self.player.attack()
                }
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
                self.state = GameState::Talking(0, 0);
                npc.is_talking = true
            }
        }
    }

    // I can not think of a better name for the love of god
    fn conversation(&mut self) {
        let (line, mut char) = match self.state {
            GameState::Talking(line, char) => (line, char),
            _ => return,
        };
        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        let talking_npc = current_map
            .npcs
            .iter_mut()
            .find(|npc| npc.is_talking)
            .unwrap();

        match talking_npc.dialogs.get(line) {
            Some(_) => {
                if talking_npc.dialogs[line].len() <= char {
                    char = talking_npc.dialogs[line].len() - 1;
                } else {
                    char += 1;
                }
                self.state = GameState::Talking(line, char)
            }
            None => {
                self.state = GameState::Normal;
                talking_npc.is_talking = false
            }
        }
    }

    pub fn tick(&mut self) {
        self.new_camera_offset();
        self.key_event_handler();
        self.anim_tick();
        match self.state {
            GameState::Talking(..) => {
                self.player.change_anim(false);
                self.conversation();
                return;
            }
            GameState::Transition(mut timer, mut moved) => {
                self.player.change_anim(false);
                self.timer_progress(&mut timer, &mut moved);
                return;
            }
            GameState::GUI => {
                self.player.change_anim(false);
                return;
            }
            GameState::Normal => (),
        }
        self.is_touching_gate();

        self.tick_player();
        let current_map = self.maps.get_mut(&self.current_map).unwrap();

        for projectile in current_map.projectiles.iter_mut() {
            projectile.tick(&mut current_map.enemies);
        }
        for monster in current_map.enemies.iter_mut() {
            monster.get_mut().tick(&mut self.player, &current_map.walls);
        }
        for spawner in current_map.spawners.iter_mut() {
            spawner.tick(&mut current_map.enemies)
        }
        for item in current_map.items.iter_mut() {
            if !item.hitbox.overlaps(&self.player.hitbox()) {
                continue;
            }
            if let Some(slot) = self.player.inventory.has_empty_slot() {
                self.player.inventory.content[slot] = Some(item.item.clone());
                item.should_delete = true
            }
        }

        current_map.clean_up();
    }

    fn tick_player(&mut self) {
        if self.state != GameState::Normal {
            self.player.change_anim(false);
        }

        match self.state {
            GameState::Talking(..) => self.conversation(),
            GameState::Transition(mut timer, mut moved) => {
                self.timer_progress(&mut timer, &mut moved)
            }
            GameState::GUI => (),
            GameState::Normal => (),
        }
        let mouse_pos = self.get_mouse_pos();
        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        self.player.tick(mouse_pos);
        self.player.wall_collsion(&current_map.walls);

        if is_mouse_button_released(MouseButton::Right) {
            current_map
                .projectiles
                .push(self.player.current_projectile(mouse_pos))
        }
    }

    fn is_touching_gate(&mut self) {
        let gates = &self.maps[&self.current_map].gates;
        let player_hitbox = self.player.hitbox();

        for gate in gates {
            if gate.hitbox().overlaps(&player_hitbox) {
                let timer = Timer::new(0.7);
                self.state = GameState::Transition(timer, false);
                self.player.state = PlayerState::Transition;
            }
        }
    }

    fn transition(&mut self, timer: &Timer, moved: &mut bool) {
        if should_move(timer, self.cam_box()) && !*moved {
            *moved = true;
            let gates = self.maps[&self.current_map].gates.clone();
            let player_hitbox = self.player.hitbox();
            let gate = gates
                .iter()
                .find(|gate| gate.hitbox().overlaps(&player_hitbox))
                .unwrap();

            self.move_map(&gate.command);
        }
    }

    fn timer_progress(&mut self, timer: &mut Timer, moved: &mut bool) {
        timer.tick();
        self.transition(timer, moved);

        if timer.is_done() {
            self.player.state = PlayerState::Normal;
            self.state = GameState::Normal;
        } else {
            self.state = GameState::Transition(*timer, *moved)
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
        let damage_zone = self.player.weapon_hitbox();
        let damage = self.player.held_weapon.base_damage;
        let player_pos = &self.player.pos();

        for monster in self.get_monster_list() {
            if damage_zone.overlaps(&monster.get().hitbox()) {
                let monster = monster.get_mut().get_mut_props();
                let knockback = vec2(monster.pos.x - player_pos.x, monster.pos.y - player_pos.y)
                    .normalize()
                    * KNOCKBACK;

                monster.health -= damage;
                monster.knockback(knockback);
            }
        }
    }

    fn move_map(&mut self, command: &str) {
        let commands: Vec<&str> = command.split_whitespace().map(|x| x.trim()).collect();

        let pos_x = commands[1].parse::<f32>().unwrap() * STANDARD_SQUARE;
        let pos_y = commands[2].parse::<f32>().unwrap() * STANDARD_SQUARE;

        //self.cam_offset.x = -pos_x / screen_width();
        //self.cam_offset.y = pos_y / screen_height();

        self.player.props.pos.x = pos_x;
        self.player.props.pos.y = pos_y;

        self.current_map = commands[0].to_string();
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

fn should_move(timer: &Timer, screen: Rect) -> bool {
    let top_right = vec2(screen.right(), screen.top());
    let timer_progress: f32 = (timer.time / timer.duration) * 2. - 1.;
    let new_width = screen.w * 2.0;
    let rect = Rect::new(
        screen.left() - new_width * timer_progress,
        screen.top(),
        new_width,
        screen.h,
    );

    rect.contains(top_right)
}
