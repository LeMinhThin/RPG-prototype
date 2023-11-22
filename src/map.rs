use macroquad::prelude::*;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use toml::{map::Map, Table, Value};

use crate::logic::STANDARD_SQUARE;
use crate::monsters::Monster;

const GATE_HITBOX_SCALE: f32 = 0.2;

pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Wall>,
    //pub interactables: Vec<Interactable>,
    pub gates: Vec<Gate>,
    pub bound: Bound,
    pub render_mesh: Vec<Vec<u8>>,
}

#[derive(Clone)]
pub struct Wall {
    pub hitbox: Rect,
    elevation: u8,
}

#[derive(Clone, Debug)]
pub struct Gate {
    pub command: String,
    pub location: Rect,
}

#[derive(Clone, Debug)]
pub struct Bound {
    pub x: u8,
    pub y: u8,
}

impl Bound {
    fn new(x: u8, y: u8) -> Self {
        Bound { x, y }
    }
}

impl Area {
    pub fn from(map: PathBuf) -> (String, Self) {
        let mut walls: Vec<Wall> = vec![];
        let mut gates = vec![];
        let enemies = vec![Monster::slime(500., 200.)];

        let content = read_to_string(map).unwrap();
        let parsed = content.parse::<Table>().unwrap();

        if let Some(x) = parsed.get("walls") {
            walls = make_walls(x.as_array().unwrap())
        }

        if let Some(x) = parsed.get("interactables") {
            gates = make_gates(x.as_table().unwrap());
        }

        let bound = make_bound(&parsed["bound"]);

        let render_mesh = calc_render_mesh(&walls, &bound);

        (
            parsed["name"].as_str().unwrap().to_string(),
            Area {
                enemies,
                walls,
                gates,
                bound,
                render_mesh,
            },
        )
    }
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

fn make_walls(content: &Vec<Value>) -> Vec<Wall> {
    let mut walls = vec![];
    for i in content {
        if !i.is_array() {
            continue;
        }
        let hitbox = Rect::new(
            i[0].as_integer().unwrap() as f32 * STANDARD_SQUARE,
            i[1].as_integer().unwrap() as f32 * STANDARD_SQUARE,
            i[2].as_integer().unwrap() as f32 * STANDARD_SQUARE,
            i[3].as_integer().unwrap() as f32 * STANDARD_SQUARE,
        );
        let elevation = i[4].as_integer().unwrap() as u8;
        walls.push(Wall { hitbox, elevation });
    }
    walls
}

fn make_gates(content: &Map<String, Value>) -> Vec<Gate> {
    let mut gates: Vec<Gate> = vec![];
    for gate in content {
        let positions = gate.1["position"].as_array().unwrap();
        let x = *&positions[0].as_integer().unwrap() as f32;
        let y = *&positions[1].as_integer().unwrap() as f32;
        let w = *&positions[2].as_integer().unwrap() as f32;
        let h = *&positions[3].as_integer().unwrap() as f32;
        let command = gate.1["move_player"].as_str().unwrap().to_string();
        gates.push(Gate::new(x, y, w, h, command))
    }
    gates
}

fn make_bound(bounds: &Value) -> Bound {
    let bound = bounds.as_array().unwrap();

    Bound::new(convert(&bound[0]), convert(&bound[1]))
}

fn convert(value: &Value) -> u8 {
    value.as_integer().unwrap() as u8
}

fn lookup_sprite(input: [bool; 4]) -> u8 {
    // 0 = north_neighbor
    // 1 = south_neighbor
    // 2 = east_neighbor
    // 3 = west_neighbor
    match input {
        [false, true, true, false] => 55,
        [false, true, true, true] => 56,
        [false, true, false, true] => 57,
        [true, true, true, false] => 67,
        [true, true, false, true] => 69,
        [true, false, true, false] => 79,
        [true, false, true, true] => 80,
        [true, false, false, true] => 81,
        [true, true, true, true] => 1,
        _ => panic!("should not happen!"),
    }
}

fn calc_render_mesh(walls: &Vec<Wall>, bounds: &Bound) -> Vec<Vec<u8>> {
    let (width, height) = (bounds.x, bounds.y);
    let mut height_map = make_mesh(width as usize, height as usize);
    for wall in walls {
        let scaled = pos_in_tile(wall);
        modify_mesh(&mut height_map, scaled)
    }

    make_render_mesh(height_map)
}

fn make_mesh(width: usize, height: usize) -> Vec<Vec<u8>> {
    // Ah yes, "Functional Programming"
    let slice: Vec<u8> = (0..=width).map(|_| 0).collect();
    (0..=height).map(|_| slice.clone()).collect()
}

fn pos_in_tile(wall: &Wall) -> Wall {
    let hitbox = wall.hitbox;
    Wall {
        hitbox: Rect {
            x: hitbox.x / STANDARD_SQUARE,
            y: hitbox.y / STANDARD_SQUARE,
            w: hitbox.w / STANDARD_SQUARE,
            h: hitbox.h / STANDARD_SQUARE,
        },
        elevation: wall.elevation,
    }
}

fn modify_mesh(mesh: &mut Vec<Vec<u8>>, wall: Wall) {
    let left = wall.hitbox.x as usize;
    let top = wall.hitbox.y as usize;
    let bottom = wall.hitbox.bottom() as usize;
    let right = wall.hitbox.right() as usize;

    for y_coord in top..bottom {
        for x_coord in left..right {
            mesh[y_coord][x_coord] = wall.elevation
        }
    }
}

fn make_render_mesh(height_map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let lenght_y = height_map.len() - 1;
    let lenght_x = height_map[0].len() - 1;

    let mut render_mesh = make_mesh(lenght_x, lenght_y);

    for y in 0..=lenght_y {
        for x in 0..=lenght_x {
            let current_cell = height_map[y][x];

            // 0 = north_neighbor
            // 1 = south_neighbor
            // 2 = east_neighbor
            // 3 = west_neighbor
            let index_y = y as i16;
            let index_x = x as i16;
            let neighbors: [Option<&u8>; 4] = [
                get_cell(&height_map, index_y - 1, index_x), // north
                get_cell(&height_map, index_y + 1, index_x), // south
                get_cell(&height_map, index_y, index_x + 1), // east
                get_cell(&height_map, index_y, index_x - 1), // west
            ];

            let lookup_array = make_lookup_array(neighbors, &current_cell);
            let cell_sprite = lookup_sprite(lookup_array);

            render_mesh[y][x] = cell_sprite;
        }
    }
    render_mesh
}

fn get_cell(map: &Vec<Vec<u8>>, y: i16, x: i16) -> Option<&u8> {
    if y < 0 {
        return None;
    }
    if x < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;
    map.get(y)?.get(x)
}

fn make_lookup_array(neighbors: [Option<&u8>; 4], point: &u8) -> [bool; 4] {
    // 0 = north_neighbor
    // 1 = south_neighbor
    // 2 = east_neighbor
    // 3 = west_neighbor

    // true = has neighbor

    let mut array: [bool; 4] = [true; 4];

    let mut index = 0;
    while index < array.len() {
        if neighbors[index] == None {
            index += 1;
            continue;
        }
        if neighbors[index].unwrap() < point {
            array[index] = false;
        }
        index += 1;
    }

    array
}
