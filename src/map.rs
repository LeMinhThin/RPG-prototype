use macroquad::prelude::Rect;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use toml::{map::Map, Table, Value};

use crate::logic::STANDARD_SQUARE;
use crate::monsters::Monster;

const GATE_HITBOX_SCALE: f32 = 0.2;

pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Rect>,
    //pub interactables: Vec<Interactable>,
    pub gates: Vec<Gate>,
}

#[derive(Clone, Debug)]
pub struct Interactable {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub command: String,
}

impl Area {
    pub fn from(map: PathBuf) -> (String, Self) {
        let mut walls: Vec<Rect> = vec![];
        let mut gates = vec![];
        let enemies = vec![Monster::cube(500., 200.)];

        let content = read_to_string(map).unwrap();
        let parsed = content.parse::<Table>().unwrap();

        if let Some(x) = parsed.get("walls") {
            walls = make_walls(x.as_array().unwrap())
        }

        if let Some(x) = parsed.get("interactables") {
            gates = make_interactables(x.as_table().unwrap());
        }

        (
            parsed["name"].as_str().unwrap().to_string(),
            Area {
                enemies,
                walls,
                gates,
            },
        )
    }
}

impl Interactable {
    /*
    pub fn hitbox(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
    */
}

pub fn get_map_list() -> Vec<PathBuf> {
    let map_path = PathBuf::from("./assets/maps/");
    let maps = read_dir(map_path).unwrap();

    let mut return_vec: Vec<PathBuf> = vec![];
    for map in maps {
        let to_add = map.unwrap().path();
        if !to_add.to_str().unwrap().contains(".toml") {
            continue;
        }
        return_vec.push(to_add)
    }
    return_vec
}

fn make_walls(content: &Vec<Value>) -> Vec<Rect> {
    let mut walls = vec![];
    for i in content {
        if !i.is_array() {
            continue;
        }
        walls.push(Rect::new(
            i[0].as_integer().unwrap() as f32 * STANDARD_SQUARE,
            i[1].as_integer().unwrap() as f32 * STANDARD_SQUARE,
            i[2].as_integer().unwrap() as f32 * STANDARD_SQUARE,
            i[3].as_integer().unwrap() as f32 * STANDARD_SQUARE,
        ))
    }
    walls
}

fn make_interactables(content: &Map<String, Value>) -> Vec<Gate> {
    let mut gates: Vec<Gate> = vec![];
    for gate in content {
        let positions = gate.1["position"].as_array().unwrap();
        let x = *&positions[0].as_integer().unwrap() as f32;
        let y = *&positions[1].as_integer().unwrap() as f32;
        let w = *&positions[2].as_integer().unwrap() as f32;
        let h = *&positions[3].as_integer().unwrap() as f32;
        let command = gate.1["on_activate"].as_str().unwrap().to_string();
        gates.push(Gate::new(x, y, w, h, command))
    }
    gates
}

#[derive(Clone, Debug)]
pub struct Gate {
    pub command: String,
    pub location: Rect,
}

impl Gate {
    fn new(x: f32, y: f32, w: f32, h: f32, command: String) -> Self {
        let location = Rect::new(
            x * STANDARD_SQUARE,
            y * STANDARD_SQUARE,
            w * STANDARD_SQUARE,
            h * STANDARD_SQUARE,
        );
        Gate { location, command }
    }

    pub fn hitbox(&self) -> Rect {
        let x = self.location.x + self.location.w * GATE_HITBOX_SCALE;
        let y = self.location.y + self.location.h * GATE_HITBOX_SCALE;
        let w = self.location.w * (1. - GATE_HITBOX_SCALE * 2.);
        let h = self.location.h * (1. - GATE_HITBOX_SCALE * 2.);
        Rect { x, y, w, h }
    }
}
