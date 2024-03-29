use macroquad::prelude::*;
use serde_json::Value;
use std::rc::Rc;

use crate::camera::TERRAIN_TILE_SIZE;
use crate::interactables::{Chest, Door, Interactables};
use crate::logic::*;
use crate::monsters::*;
use crate::npc::NPC;
use crate::player::PIXEL;
use crate::ui::items::*;
use spawner::*;

pub const RATIO: f32 = TILE / TERRAIN_TILE_SIZE;
const PROJ_SPEED: f32 = 2000.;
pub type Monster = Box<dyn Entity>;
pub type Interactable = Box<dyn Interactables>;

pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Rect>,
    pub gates: Vec<Gate>,
    pub spawners: Vec<Spawner>,
    pub npcs: Vec<NPC>,
    pub projectiles: Vec<Projectile>,
    pub items: Vec<ItemEntity>,
    pub interactables: Vec<Interactable>,
    pub draw_mesh: Meshes,
}

pub struct Projectile {
    pos: Vec2,
    speed: Vec2,
    damage: f32,
    life_time: Timer,
    should_despawn: bool,
}

#[derive(Clone)]
pub struct Gate {
    pub map: Rc<str>,
    pub hitbox: Rect,
    pub location: Vec2,
}

pub struct Meshes {
    pub terrain: Vec<Vec<u16>>,
    pub decorations: Vec<Vec<u16>>,
}

impl Meshes {
    fn new() -> Self {
        let terrain = vec![];
        let decorations = vec![];

        Meshes {
            terrain,
            decorations,
        }
    }
}

impl Projectile {
    pub fn new(pos: Vec2, speed: Vec2) -> Self {
        let speed = vec2(speed.x, -speed.y);
        let pos = vec2(pos.x, pos.y + 6. * PIXEL);
        Self {
            pos,
            life_time: Timer::new(0.8),
            should_despawn: false,
            speed: speed * PROJ_SPEED,
            damage: 10.,
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        let center = self.hitbox().center();
        let dest_size = Some(vec2(TILE, TILE));
        let source = Some(Rect::new(TILE_SIZE * 6., TILE_SIZE, TILE_SIZE, TILE_SIZE));
        let rotation = self.speed.angle_between(vec2(1., 0.));
        let params = DrawTextureParams {
            dest_size,
            source,
            rotation,
            ..Default::default()
        };

        draw_texture_ex(
            texture,
            center.x - TILE / 2.,
            center.y - TILE / 2.,
            WHITE,
            params,
        )
    }

    pub fn tick(&mut self, monsters: &mut Vec<Monster>) {
        self.new_pos();
        self.life_time.tick();
        let hitbox = self.hitbox();

        for monster in monsters.iter_mut() {
            if hitbox.overlaps(&monster.hitbox()) {
                let props = monster.get_mut_props();
                props.health -= self.damage;
                props.knockback(self.speed.normalize() * KNOCKBACK);
                self.should_despawn = true
            }
        }
    }

    fn new_pos(&mut self) {
        let dt = get_frame_time();
        self.pos.x += self.speed.x * dt;
        self.pos.y += self.speed.y * dt;
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 11. * PIXEL, 10. * PIXEL)
    }
}

impl Area {
    pub fn from(json_string: &str) -> (Rc<str>, Self) {
        // Now I could go ahead and handle all of these potential errors like a good developer but
        // instead, I chose to ignore it. Now that the code is a tangled mess, it is quite
        // difficult to handle all of the errors
        let parsed: Value = serde_json::from_str(json_string).unwrap();
        let name = parsed["class"].as_str().unwrap();

        let mut draw_mesh = Meshes::new();
        let mut walls = vec![];
        let mut spawners = vec![];
        let mut gates = vec![];
        let mut npcs = vec![];
        let mut interactables = vec![];

        for layer in parsed["layers"].as_array().unwrap() {
            match layer["name"].as_str().unwrap().to_lowercase().as_str() {
                "terrain" => {
                    draw_mesh.terrain = make_render_mesh(layer).unwrap();
                }
                "walls" => {
                    walls = make_walls(layer);
                }
                "decorations" => {
                    draw_mesh.decorations = make_render_mesh(layer).unwrap();
                }
                "spawners" => {
                    spawners = make_spawners(layer).unwrap();
                }
                "gates" => {
                    gates = make_gates(layer).unwrap();
                }
                "npcs" => {
                    npcs = make_npcs(layer).unwrap();
                }
                "interactables" => {
                    interactables = parse_interactable(layer);
                }
                _ => (),
            }
        }

        for npc in &npcs {
            walls.push(npc.hitbox)
        }
        for chest in &interactables {
            walls.push(chest.hitbox())
        }

        (
            name.into(),
            Area {
                enemies: vec![],
                projectiles: vec![],
                items: vec![],
                spawners,
                draw_mesh,
                gates,
                walls,
                npcs,
                interactables,
            },
        )
    }

    pub fn clean_up(&mut self) {
        let projectiles = &mut self.projectiles;
        let mobs = &mut self.enemies;
        // Spawn loot for every dying mob
        for mob in mobs.iter() {
            if !mob.get_props().should_despawn {
                continue;
            }
            let loot = mob.loot();
            if let Some(loot) = loot {
                self.items.push(ItemEntity::new(loot, mob.pos()))
            }
        }
        let items = &mut self.items;
        mobs.retain(|mob| !mob.get_props().should_despawn);
        projectiles.retain(|proj| !proj.should_despawn && !proj.life_time.is_done());
        items.retain(|item| !item.should_delete);
    }
}
impl Gate {
    fn new(hitbox: Rect, location: Vec2, command: &str) -> Self {
        Gate {
            hitbox,
            location,
            map: command.into(),
        }
    }

    pub fn hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn get_transition(&self) -> Transition {
        let pos = self.location;
        let map = self.map.clone();
        Transition::new(pos, map)
    }
}

// I dont really need these to be of type Option but doing so will alow me to use the ? operator,
// which is shorter than just writing out .unwrap()
fn make_render_mesh(objects: &Value) -> Option<Vec<Vec<u16>>> {
    // Ah yes, functional programming
    let parsed = objects["data"].as_array()?;
    let lenght = objects["width"].as_u64()? as usize;

    let temp: Vec<u16> = parsed
        .iter()
        .map(|elem| elem.as_i64().unwrap() as u16)
        .collect();

    let return_vec = temp.chunks(lenght).map(|elem| elem.into()).collect();

    Some(return_vec)
}

fn make_walls(objects: &Value) -> Vec<Rect> {
    let raw_data;
    // Some thing is telling me I can do better than this
    if let Some(value) = objects.get("objects") {
        if let Some(arr) = value.as_array() {
            raw_data = arr
        } else {
            error!("make_walls [ERROR] field object is not of type array");
            return vec![];
        }
    } else {
        error!("make_walls [ERROR] field objects does not exist");
        return vec![];
    }

    let mut walls: Vec<Rect> = vec![];
    for wall in raw_data {
        let mut wall_cons = Rect::new(0., 0., 0., 0.);
        wall_cons.x = get_pos(wall, "x", "make_walls") * RATIO;
        wall_cons.y = get_pos(wall, "y", "make_walls") * RATIO;
        wall_cons.w = get_pos(wall, "width", "make_walls") * RATIO;
        wall_cons.h = get_pos(wall, "height", "make_walls") * RATIO;

        walls.push(wall_cons);
    }
    walls
}

fn make_spawners(objects: &Value) -> Option<Vec<Spawner>> {
    let mut spawners: Vec<Spawner> = vec![];
    let data = objects["objects"].as_array()?;
    for spawner in data {
        let x = spawner["x"].as_f64()? as f32;
        let y = spawner["y"].as_f64()? as f32;
        let (cooldown, spawn_radius, kind, max_mob) = get_props(spawner)?;

        let spawner = Spawner::new(
            kind,
            spawn_radius,
            max_mob,
            cooldown,
            vec2(x * RATIO, y * RATIO),
        );

        spawners.push(spawner)
    }

    Some(spawners)
}

// f32 cooldown
// f32 spawn_radius
// String kind
// int max_mob

fn get_props(objects: &Value) -> Option<(f32, f32, MobType, u32)> {
    let props = objects["properties"].as_array()?;
    // Default values
    let mut cooldown = 30.;
    let mut spawn_radius = 3. * TILE;
    let mut kind = MobType::Slime;
    let mut max_mob = 3;

    if let Some(mob) = objects["type"].as_str() {
        kind = what_kind(mob)
    }
    for prop in props {
        match prop["name"].as_str()? {
            "cooldown" => cooldown = prop["value"].as_f64()? as f32,
            "max_mob" => max_mob = prop["value"].as_f64()? as u32,
            "spawn_radius" => {
                spawn_radius = prop["value"].as_f64()? as f32 * TILE;
            }
            x => warn!("[WARN] unrecognised field name {}", x),
        }
    }

    Some((cooldown, spawn_radius, kind, max_mob))
}

fn what_kind(name: &str) -> MobType {
    match name {
        "slime" => MobType::Slime,
        "mushroom" => MobType::Mushroom,
        x => {
            warn!("[WARN] unrecognised mob type {}, falling back to slime", x);
            MobType::Slime
        }
    }
}

fn make_gates(objects: &Value) -> Option<Vec<Gate>> {
    let mut gates: Vec<Gate> = vec![];

    let props = objects["objects"].as_array()?;
    for gate in props {
        let x = get_pos(gate, "x", "make_gates") * RATIO;
        let y = get_pos(gate, "y", "make_gates") * RATIO;
        let w = get_pos(gate, "width", "make_gates") * RATIO;
        let h = get_pos(gate, "height", "make_gates") * RATIO;
        let hitbox = Rect::new(x, y, w, h);

        let command = match get_command(&gate["properties"]) {
            Some(command) => command,
            None => {
                dbg!(gate);
                dbg!(objects);
                panic!("shid")
            }
        };
        let commands: Vec<&str> = command.split_whitespace().collect();
        let command = commands[0].trim().into();
        let pos_x = commands[1].trim().parse::<f32>().unwrap() * TILE;
        let pos_y = commands[2].trim().parse::<f32>().unwrap() * TILE;

        gates.push(Gate::new(hitbox, vec2(pos_x, pos_y), command))
    }

    Some(gates)
}

fn get_command(objects: &Value) -> Option<&str> {
    let commands = objects.as_array()?;

    let mut string = "";
    for i in commands {
        if i["name"].as_str()? == "to" {
            string = i["value"].as_str()?;
        }
    }
    if string.is_empty() {
        return None;
    }
    Some(string)
}

fn make_npcs(objects: &Value) -> Option<Vec<NPC>> {
    let mut npcs: Vec<NPC> = vec![];

    let list = objects["objects"].as_array()?;

    for item in list {
        let mut diag_path = "";
        let name = item["name"].as_str()?;

        let x = get_pos(item, "x", "make_npcs");
        let y = get_pos(item, "y", "make npcs");
        let hitbox = Rect::new(x * RATIO, y * RATIO, 100., 50.);

        let props = item["properties"].as_array()?;

        for prop in props {
            if prop["name"].as_str()? == "dialog" {
                diag_path = prop["value"].as_str()?;
            }
        }
        let npc = NPC::new(name, &diag_path, hitbox);
        npcs.push(npc);
    }

    Some(npcs)
}

fn parse_interactable(table: &Value) -> Vec<Interactable> {
    let mut ret_vec = vec![];
    let table = match table.get("objects") {
        Some(table) => table,
        None => {
            error!("parse_interactable: Invalid layer");
            return ret_vec;
        }
    };
    let table = match table.as_array() {
        Some(table) => table,
        None => {
            error!("parse_interactable: Layer can not be represented as array");
            return ret_vec;
        }
    };

    for item in table {
        match item["type"].as_str().unwrap().to_lowercase().as_str() {
            "chest" => ret_vec.push(make_chest(item)),
            "door" => ret_vec.push(make_door(item)),
            x => {
                error!("Unrecognised interactable type {}", x);
                dbg!(item);
            }
        }
    }

    ret_vec
}

fn make_door(table: &Value) -> Interactable {
    let x = get_pos(table, "x", "make_door") * RATIO;
    let y = get_pos(table, "y", "make_door") * RATIO;
    let w = get_pos(table, "width", "make_door") * RATIO;
    let h = get_pos(table, "height", "make_door") * RATIO;
    let hitbox = Rect::new(x, y, w, h);

    let command = get_command(&table["properties"]).unwrap();
    let commands: Vec<&str> = command.split_whitespace().collect();
    let command = commands[0].trim().into();
    let pos_x = commands[1].trim().parse::<f32>().unwrap() * TILE;
    let pos_y = commands[2].trim().parse::<f32>().unwrap() * TILE;
    let location = vec2(pos_x, pos_y);

    let door = Door::new(hitbox, command, location);
    Box::new(door)
}

fn make_chest(table: &Value) -> Interactable {
    let x = get_pos(table, "x", "make_chest") * RATIO + PIXEL;
    let y = get_pos(table, "y", "make_chest") * RATIO + PIXEL;
    let item = match get_item(table) {
        Ok(item) => item,
        Err(ItemErr::ParseErr(name)) => {
            warn!("Invalid Item name {name}, defaulting to slime");
            Item::slime(1)
        }
        Err(ItemErr::NotSameType) => {
            warn!("Key is not of type string, defaulting to slime");
            Item::slime(1)
        }
        Err(ItemErr::NoKey) => {
            warn!("Key 'item' does not exist, defaulting to slime");
            Item::slime(1)
        }
    };
    let chest = Chest::new(vec2(x, y), item);
    Box::new(chest)
}

fn get_pos(table: &Value, value: &str, func: &str) -> f32 {
    let result = match table.get(value) {
        Some(value) => value,
        None => {
            warn!("{func} [WARN] Field {value} does not exist, falling back to 0 as default");
            return 0.;
        }
    };
    let value = match result.as_f64() {
        Some(float) => float as f32,
        None => {
            warn!("{func} [WARN] Field {value} is not of type float, falling back to 0 as default");
            return 0.;
        }
    };
    value
}

enum ItemErr {
    NoKey,
    NotSameType,
    ParseErr(String),
}

fn get_item(table: &Value) -> Result<Item, ItemErr> {
    let key = table.get("properties").ok_or(ItemErr::NoKey)?;
    let key = key.as_array().ok_or(ItemErr::NoKey)?;
    // Since tiled would not allow an array to have 0 item this code should not cause a crash
    let key = key[0]["value"].as_str().ok_or(ItemErr::NotSameType)?;

    let item = match key.to_lowercase().as_str() {
        "rusty sword" | "rusty_sword" => Ok(Item::rusty_sword()),
        "black sword" | "black_sword" => Ok(Item::black_sword()),
        "slime" => Ok(Item::slime(1)),
        "mushroom" => Ok(Item::mushroom(1)),
        x => Err(ItemErr::ParseErr(String::from(x))),
    };
    item
}
