use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::rc::Rc;

use crate::interactables::GameSignal;
use crate::map::*;
use crate::player::*;
use crate::ui::*;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 24.;
pub const SCALE_FACTOR: f32 = 6.;
pub const TILE: f32 = TILE_SIZE * SCALE_FACTOR;
pub const KNOCKBACK: f32 = 5000.;

pub type Textures = HashMap<Rc<str>, Texture2D>;
pub type Maps = HashMap<Rc<str>, Area>;

pub struct Game {
    pub player: Player,
    pub maps: Maps,
    pub current_map: Rc<str>,
    pub cam_offset: Vec2,
    pub textures: Textures,
    pub state: GameState,
    pub tasks: Vec<GameSignal>, // This is kind of a hack
    pub font: Font,
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub timer: Timer,
    moved: bool,
    pos: Vec2,
    map: Rc<str>,
}

#[derive(Clone, Debug)]
pub enum GUIType {
    Inventory,
    MainMenu(MainMenu),
    DeathScreen(DeathScreen),
}

#[derive(Clone, Debug)]
pub enum GameState {
    Normal,
    Quit,
    GUI(GUIType),
    Talking(usize, usize),
    Transition(Transition),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Timer {
    pub time: f32,
    pub duration: f32,
}

impl Transition {
    pub fn new(pos: Vec2, map: Rc<str>) -> Self {
        Self {
            timer: Timer::new(0.7),
            pos,
            map,
            moved: false,
        }
    }
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

    pub fn progress(&self) -> f32 {
        1. - self.time / self.duration
    }
}

impl Game {
    pub fn new(textures: Textures, font: Font) -> Self {
        let mut area: Maps = HashMap::new();
        // TODO unhardcode this value
        let current_map = "Village".into();

        let map_list = get_path("assets/maps/", ".json");
        for map in map_list {
            let json_string = read_to_string(map).unwrap();
            let map_content = Area::from(&json_string);
            area.insert(map_content.0, map_content.1);
        }
        let state = GameState::GUI(GUIType::MainMenu(MainMenu::new()));

        Game {
            player: Player::new(),
            tasks: vec![],
            maps: area,
            current_map,
            textures,
            cam_offset: vec2(0., 0.),
            state,
            font,
        }
    }

    fn key_event_handler(&mut self) {
        if is_key_pressed(KeyCode::E) {
            self.state = GameState::GUI(GUIType::Inventory)
        }

        if is_key_pressed(KeyCode::Escape) {
            let gui = match &self.state {
                GameState::GUI(gui) => gui,
                _ => return,
            };
            match gui {
                GUIType::Inventory => self.state = GameState::Normal,
                _ => return,
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
                    if let PlayerState::Attacking(..) = self.player.state {
                        return;
                    }
                    // If the player has no weapon
                    if let None = self.player.inventory.content[12] {
                        return;
                    }
                    let mouse_pos = self.get_mouse_pos();
                    self.player.face(mouse_pos);
                    self.player.attack(mouse_pos);
                }
                _ => return,
            }
        }
    }

    // This could use a better name
    fn talk_to_npc(&mut self) {
        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        let npcs = &mut current_map.npcs;
        let search_box = self.player.search_box();

        for npc in npcs.iter_mut() {
            if !npc.hitbox.overlaps(&search_box) {
                continue;
            }
            self.state = GameState::Talking(0, 0);
            npc.is_talking = true;
            npc.face(self.player.pos())
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
        match self.state.clone() {
            GameState::Talking(..) => {
                self.player.change_anim(false);
                self.conversation();
                return;
            }
            GameState::Transition(mut transition) => {
                self.player.change_anim(false);
                self.timer_progress(&mut transition);
                return;
            }
            GameState::GUI(_) => {
                self.player.change_anim(false);
                self.tick_gui();
                return;
            }
            GameState::Normal | GameState::Quit => (),
        }

        self.tick_player();
        self.tick_map();
        self.do_task();
    }

    fn do_task(&mut self) {
        let tasks = self.tasks.clone();
        tasks
            .iter()
            .for_each(|task| self.handle_signals(task));
        self.tasks.clear()
    }

    fn tick_map(&mut self) {
        let current_map = self.maps.get_mut(&self.current_map).unwrap();

        for projectile in current_map.projectiles.iter_mut() {
            projectile.tick(&mut current_map.enemies);
        }
        for monster in current_map.enemies.iter_mut() {
            monster.tick(&mut self.player, &current_map.walls);
        }
        for spawner in current_map.spawners.iter_mut() {
            spawner.tick(&mut current_map.enemies)
        }
        for item in current_map.items.iter_mut() {
            if !item.hitbox.overlaps(&self.player.hitbox()) {
                continue;
            }
            self.player.inventory.append(item.item.clone());
            item.should_delete = true
        }
        let search_box = self.player.search_box();

        for interactable in current_map.interactables.iter_mut() {
            let signal = interactable.activate(&search_box);
            if let Some(signal) = signal {
                self.tasks.push(signal)
            }
        }

        let player_hitbox = self.player.hitbox();
        for gate in &current_map.gates {
            if !gate.hitbox().overlaps(&player_hitbox) {
                continue;
            }
            self.state = GameState::Transition(gate.get_transition());
            self.player.state = PlayerState::Transition;
        }

        current_map.clean_up();
    }

    fn handle_signals(&mut self, signal: &GameSignal) {
        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        match signal {
            GameSignal::SpawnItem(item) => current_map.items.push(item.clone()),
            GameSignal::MovePlayer(trans) => self.state = GameState::Transition(trans.clone())
        }
    }

    fn tick_player(&mut self) {
        match self.state.clone() {
            GameState::Talking(..) => {
                self.conversation();
                self.player.change_anim(false)
            }
            GameState::Transition(mut transition) => {
                self.timer_progress(&mut transition);
                self.player.change_anim(false)
            }
            GameState::GUI(_) => {
                self.player.change_anim(false);
            }
            GameState::Normal | GameState::Quit => (),
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
        if let PlayerState::Attacking(mut attack) = self.player.state {
            self.damage_monster(&mut attack);
            self.player.state = PlayerState::Attacking(attack)
        }

        if self.player.props.health <= 0. {
            self.state = GameState::GUI(GUIType::DeathScreen(DeathScreen::new()))
        }
    }

    fn transition(&mut self, transition: &mut Transition) {
        let timer = transition.timer;
        let moved = &mut transition.moved;
        if should_move(&timer, self.cam_box()) && !*moved {
            *moved = true;
            self.move_map(transition.pos, transition.map.clone());
        }
    }

    fn timer_progress(&mut self, transition: &mut Transition) {
        transition.timer.tick();
        self.transition(transition);

        if transition.timer.is_done() {
            self.player.state = PlayerState::Normal;
            self.state = GameState::Normal;
        } else {
            self.state = GameState::Transition(transition.clone())
        }
    }

    fn anim_tick(&mut self) {
        self.player.props.animation.update();

        let current_map = self.maps.get_mut(&self.current_map).unwrap();
        for monster in current_map.enemies.iter_mut() {
            monster.tick_anim();
        }

        for npc in current_map.npcs.iter_mut() {
            npc.update_anim();
            npc.anim.update();
        }
    }

    fn get_monster_list(&mut self) -> &mut [Monster] {
        &mut self.maps.get_mut(&self.current_map).unwrap().enemies
    }

    fn damage_monster(&mut self, attack: &mut Attack) {
        let prog = attack.timer.progress();
        if prog < 0.5 {
            return;
        }
        if attack.attacked {
            return;
        }
        let damage_zone = self.player.weapon_hitbox();
        let damage = self.player.held_weapon.base_damage;
        let player_pos = self.player.pos();

        for monster in self.get_monster_list() {
            if !damage_zone.overlaps(&monster.hitbox()) {
                continue;
            }
            let monster = monster.get_mut_props();
            let knockback = vec2(monster.pos.x - player_pos.x, monster.pos.y - player_pos.y)
                .normalize()
                * KNOCKBACK;

            monster.health -= damage;
            monster.knockback(knockback);
        }

        attack.attacked = true;

        // A bit unrelated since this will move the player toward the mouse
        let vector = (attack.mouse_pos - player_pos).normalize() * 8. * TILE;
        self.player.props.velocity += vector;
    }

    fn move_map(&mut self, pos: Vec2, map: Rc<str>) {
        self.player.props.pos.x = pos.x;
        self.player.props.pos.y = pos.y;

        self.current_map = map
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
