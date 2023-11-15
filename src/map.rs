use macroquad::prelude::Rect;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use toml::{map::Map, Table, Value};

use crate::logic::STANDARD_SQUARE;
use crate::monsters::Monster;

#[derive(Clone, Debug)]
pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Rect>,
    pub interactables: Vec<Interactable>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InteractableType {
    Gate,
    //Item,
}

#[derive(Clone, Debug)]
pub struct Interactable {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub command: String,
    pub kind: InteractableType,
}

impl Area {
    pub fn from(map: PathBuf) -> (String, Self) {
        let mut walls: Vec<Rect> = vec![];
        let mut interactables = vec![];
        let enemies = vec![Monster::cube(500., 200.)];

        let content = read_to_string(map).unwrap();
        let parsed = content.parse::<Table>().unwrap();

        if let Some(x) = parsed.get("walls") {
            walls = make_walls(x.as_array().unwrap())
        }

        if let Some(x) = parsed.get("interactables") {
            interactables = make_interactables(x.as_table().unwrap());
        }

        (
            parsed["name"].as_str().unwrap().to_string(),
            Area {
                enemies,
                walls,
                interactables,
            },
        )
    }

    pub fn get_gates(&self) -> Vec<Interactable> {
        self.interactables
            .clone()
            .into_iter()
            .filter(|x| x.kind == InteractableType::Gate)
            .collect()
    }
}

impl Interactable {
    fn gate(x: f32, y: f32, w: f32, h: f32, command: String) -> Self {
        Interactable {
            x: x * STANDARD_SQUARE,
            y: y * STANDARD_SQUARE,
            w: w * STANDARD_SQUARE,
            h: h * STANDARD_SQUARE,
            command,
            kind: InteractableType::Gate,
        }
    }

    pub fn hitbox(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
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

fn make_interactables(content: &Map<String, Value>) -> Vec<Interactable> {
    let mut gates: Vec<Interactable> = vec![];
    for gate in content {
        let positions = gate.1["position"].as_array().unwrap();
        let x = *&positions[0].as_integer().unwrap() as f32;
        let y = *&positions[1].as_integer().unwrap() as f32;
        let w = *&positions[2].as_integer().unwrap() as f32;
        let h = *&positions[3].as_integer().unwrap() as f32;
        let command = gate.1["on_activate"].as_str().unwrap().to_string();
        gates.push(Interactable::gate(x, y, w, h, command))
    }
    gates
}
