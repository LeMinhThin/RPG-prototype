use macroquad::prelude::*;
use serde_json::Value;
use std::rc::Rc;

use crate::camera::TERRAIN_TILE_SIZE;
use crate::logic::{Timer, KNOCKBACK, STANDARD_SQUARE, TILE_SIZE};
use crate::monsters::*;
use crate::npc::NPC;
use crate::player::PIXEL;
use crate::ui::items::*;
use spawner::*;

pub const RATIO: f32 = STANDARD_SQUARE / TERRAIN_TILE_SIZE;
const PROJ_SPEED: f32 = 2000.;

pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Rect>,
    pub gates: Vec<Gate>,
    pub spawners: Vec<Spawner>,
    pub npcs: Vec<NPC>,
    pub projectiles: Vec<Projectile>,
    pub draw_mesh: Meshes,
    pub items: Vec<ItemEntity>,
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
    pub command: Rc<str>,
    pub location: Rect,
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
        let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
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
            center.x - STANDARD_SQUARE / 2.,
            center.y - STANDARD_SQUARE / 2.,
            WHITE,
            params,
        )
    }

    pub fn tick(&mut self, monsters: &mut Vec<Monster>) {
        self.new_pos();
        self.life_time.tick();
        let hitbox = self.hitbox();

        for monster in monsters.iter_mut() {
            let monster = monster.get_mut();
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
    pub fn from(json_string: &str) -> (String, Self) {
        let parsed: Value = serde_json::from_str(json_string).unwrap();
        let name = parsed["class"].as_str().unwrap();

        let mut draw_mesh = Meshes::new();
        let mut walls = vec![];
        let mut spawners = vec![];
        let mut gates = vec![];
        let mut npcs = vec![];

        for layer in parsed["layers"].as_array().unwrap() {
            match layer["name"].as_str().unwrap().to_lowercase().as_str() {
                "terrain" => {
                    draw_mesh.terrain = make_render_mesh(layer).unwrap();
                }
                "walls" => {
                    walls = make_walls(layer).unwrap();
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
                _ => (),
            }
        }

        (
            name.to_string(),
            Area {
                enemies: vec![],
                projectiles: vec![],
                items: vec![],
                spawners,
                draw_mesh,
                gates,
                walls,
                npcs,
            },
        )
    }

    pub fn clean_up(&mut self) {
        let projectiles = &mut self.projectiles;
        let mobs = &mut self.enemies;
        for mob in mobs.iter() {
            let mob = mob.get();
            if mob.get_props().health <= 0. {
                self.items.push(ItemEntity::new(Item::slime(1), mob.pos()))
            }
        }
        let items = &mut self.items;
        mobs.retain(|mob| mob.get().get_props().health > 0.);
        projectiles.retain(|proj| !proj.should_despawn && !proj.life_time.is_done());
        items.retain(|item| !item.should_delete);
    }
}
impl Gate {
    fn new(x: f32, y: f32, w: f32, h: f32, command: &str) -> Self {
        let location = Rect::new(x, y, w, h);
        Gate {
            location,
            command: command.into(),
        }
    }

    pub fn hitbox(&self) -> Rect {
        self.location
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
    let mut spawn_radius = 3. * STANDARD_SQUARE;
    let mut kind = MobType::Slime;
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

fn what_kind(name: &str) -> MobType {
    match name {
        "slime" => MobType::Slime,
        "mushroom" => MobType::Mushroom,
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

fn get_command(objects: &Value) -> Option<&str> {
    let commands = objects.as_array().unwrap();

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
