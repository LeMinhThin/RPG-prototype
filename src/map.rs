use macroquad::prelude::*;
use serde_json::Value;

use crate::camera::TERRAIN_TILE_SIZE;
use crate::logic::STANDARD_SQUARE;
use crate::monsters::*;
use crate::npc::NPC;
use spawner::*;

pub const RATIO: f32 = STANDARD_SQUARE / TERRAIN_TILE_SIZE;

pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Rect>,
    pub gates: Vec<Gate>,
    pub spawners: Vec<Spawner>,
    pub npcs: Vec<NPC>,
    pub bound: Bound,
    pub draw_mesh: Meshes,
}

#[derive(Clone, Debug)]
pub struct Gate {
    pub command: String,
    pub location: Rect,
}

#[derive(Clone, Debug)]
pub struct Bound {
    pub x: usize,
    pub y: usize,
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

impl Bound {
    fn from(table: &Value) -> Self {
        let x = table["width"].as_u64().unwrap() as usize;
        let y = table["height"].as_u64().unwrap() as usize;
        Bound { x, y }
    }
}

impl Area {
    pub fn from(json_string: &str) -> (String, Self) {
        let parsed: Value = serde_json::from_str(json_string).unwrap();
        let name = parsed["class"].as_str().unwrap();

        let bound = Bound::from(&parsed);

        let mut draw_mesh = Meshes::new();
        let mut walls = vec![];
        let mut spawners = vec![];
        let mut gates = vec![];
        let mut npcs = vec![];

        for layer in parsed["layers"].as_array().unwrap() {
            match layer["name"].as_str().unwrap().to_lowercase().as_str() {
                "terrain" => {
                    draw_mesh.terrain = make_render_mesh(&bound, layer).unwrap();
                }
                "walls" => {
                    walls = make_walls(layer).unwrap();
                }
                "decorations" => {
                    draw_mesh.decorations = make_render_mesh(&bound, layer).unwrap();
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
                _ => (),
            }
        }

        (
            name.to_string(),
            Area {
                enemies: vec![],
                bound,
                spawners,
                draw_mesh,
                gates,
                walls,
                npcs,
            },
        )
    }

    pub fn clean_up(&mut self) {
        let mut index = 0;
        while index < self.enemies.len() {
            if self.enemies[index].get().get_props().heath <= 0. {
                self.enemies.remove(index);
            }
            index += 1;
        }
    }
}
impl Gate {
    fn new(x: f32, y: f32, w: f32, h: f32, command: String) -> Self {
        let location = Rect::new(x, y, w, h);
        Gate { location, command }
    }

    /*
    pub fn hitbox(&self) -> Rect {
        let x = self.location.x + self.location.w * GATE_HITBOX_SCALE;
        let y = self.location.y + self.location.h * GATE_HITBOX_SCALE;
        let w = self.location.w * (1. - GATE_HITBOX_SCALE * 2.);
        let h = self.location.h * (1. - GATE_HITBOX_SCALE * 2.);
        Rect { x, y, w, h }
    }
    */
    pub fn hitbox(&self) -> Rect {
        self.location
    }
}

// I dont really need these to be of type Option but doing so will alow me to use the ? operator,
// which is shorter than just writing out .unwrap()
fn make_render_mesh(bound: &Bound, objects: &Value) -> Option<Vec<Vec<u16>>> {
    // Ah yes, functional programming
    let parsed = &objects["data"].as_array()?;

    let temp: Vec<u16> = parsed
        .iter()
        .map(|elem| elem.as_i64().unwrap() as u16)
        .collect();

    let return_vec = temp
        .chunks(bound.x as usize)
        .map(|elem| elem.into())
        .collect();

    Some(return_vec)
}

fn make_walls(objects: &Value) -> Option<Vec<Rect>> {
    let raw_data = objects["objects"].as_array()?;
    let mut walls: Vec<Rect> = vec![];
    for wall in raw_data {
        let x = wall["x"].as_f64()? as f32;
        let y = wall["y"].as_f64()? as f32;
        let w = wall["width"].as_f64()? as f32;
        let h = wall["height"].as_f64()? as f32;

        let wall = Rect::new(x * RATIO, y * RATIO, w * RATIO, h * RATIO);

        walls.push(wall);
    }
    Some(walls)
}

fn make_spawners(objects: &Value) -> Option<Vec<Spawner>> {
    let mut spawners: Vec<Spawner> = vec![];
    let data = objects["objects"].as_array()?;
    for spawner in data {
        let x = spawner["x"].as_f64()? as f32;
        let y = spawner["y"].as_f64()? as f32;
        let (cooldown, spawn_radius, kind, max_mob) = get_props(spawner)?;

        let spawner = Spawner::new(kind, spawn_radius, max_mob, cooldown, x * RATIO, y * RATIO);

        spawners.push(spawner)
    }

    Some(spawners)
}

// f32 cooldown
// f32 spawn_radius
// String kind
// int max_mob

fn get_props(objects: &Value) -> Option<(f32, f32, SpawnerType, u32)> {
    let props = objects["properties"].as_array()?;
    // Default values
    let mut cooldown = 30.;
    let mut spawn_radius = 3. * STANDARD_SQUARE;
    let mut kind = SpawnerType::Slime;
    let mut max_mob = 3;

    for prop in props {
        match prop["name"].as_str()? {
            "cooldown" => cooldown = prop["value"].as_f64()? as f32,
            "kind" => kind = what_kind(prop["value"].as_str()?),
            "max_mob" => max_mob = prop["value"].as_f64()? as u32,
            "spawn_radius" => {
                spawn_radius = prop["value"].as_f64()? as f32 * STANDARD_SQUARE;
            }
            x => panic!("you forgot to account for {x}"),
        }
    }

    Some((cooldown, spawn_radius, kind, max_mob))
}

fn what_kind(name: &str) -> SpawnerType {
    match name {
        "slime" => SpawnerType::Slime,
        "mushroom" => SpawnerType::Mushroom,
        x => panic!("you forgot to account for {x}"),
    }
}

fn make_gates(objects: &Value) -> Option<Vec<Gate>> {
    let mut gates: Vec<Gate> = vec![];

    let props = objects["objects"].as_array()?;
    for gate in props {
        let x = gate["x"].as_f64()? as f32 * RATIO;
        let y = gate["y"].as_f64()? as f32 * RATIO;
        let w = gate["width"].as_f64()? as f32 * RATIO;
        let h = gate["height"].as_f64()? as f32 * RATIO;

        let command = get_command(&gate["properties"]).unwrap();

        gates.push(Gate::new(x, y, w, h, command))
    }

    Some(gates)
}

fn get_command(objects: &Value) -> Option<String> {
    let commands = objects.as_array().unwrap();

    let mut string = String::new();
    for i in commands {
        if i["name"].as_str()? == "to" {
            string = i["value"].as_str()?.to_string();
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

        let x = item["x"].as_f64()? as f32;
        let y = item["y"].as_f64()? as f32;
        let w = item["width"].as_f64()? as f32;
        let h = item["height"].as_f64()? as f32;
        let hitbox = Rect::new(x * RATIO, y * RATIO, w * RATIO, h * RATIO);

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
