use macroquad::prelude::*;
use serde_json::Value;

use crate::camera::TERRAIN_TILE_SIZE;
use crate::logic::STANDARD_SQUARE;
use crate::monsters::Monster;

const GATE_HITBOX_SCALE: f32 = 0.2;

pub struct Area {
    pub enemies: Vec<Monster>,
    pub walls: Vec<Wall>,
    //pub interactables: Vec<Interactable>,
    pub gates: Vec<Gate>,
    pub bound: Bound,
    pub draw_mesh: Vec<Vec<u8>>,
}

#[derive(Clone)]
pub struct Wall {
    pub hitbox: Rect,
    pub elevation: u8,
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
    fn from(table: &Value) -> Self {
        let x = table["width"].as_u64().unwrap() as u8;
        let y = table["height"].as_u64().unwrap() as u8;
        Bound { x, y }
    }
}

impl Area {
    pub fn from(json_string: &str) -> (String, Self) {
        // TODO parse this shit
        let parsed: Value = serde_json::from_str(json_string).unwrap();

        let bound = Bound::from(&parsed);

        let mut draw_mesh = vec![];
        let mut walls = vec![];

        for layer in parsed["layers"].as_array().unwrap() {
            match layer["name"].as_str().unwrap().to_lowercase().as_str() {
                "terrain" => {
                    draw_mesh = make_render_mesh(&bound, layer).unwrap();
                }
                "walls" => {
                    walls = make_walls(layer).unwrap();
                }
                _ => (),
            }
        }

        (
            "Village".to_string(),
            Area {
                enemies: vec![Monster::slime()],
                bound,
                draw_mesh,
                gates: vec![],
                walls,
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

// I dont really need these to be of type Option but doing so will alow me to use the ? operator,
// which is shorter than just writing out .unwrap()
fn make_render_mesh(bound: &Bound, objects: &Value) -> Option<Vec<Vec<u8>>> {
    let parsed = &objects["data"].as_array()?;
    let temp: Vec<u8> = parsed
        .iter()
        .map(|elem| elem.as_i64().unwrap() as u8)
        .collect();
    let return_vec = temp
        .chunks(bound.x as usize)
        .map(|elem| elem.into())
        .collect();
    Some(return_vec)
}

fn make_walls(objects: &Value) -> Option<Vec<Wall>> {
    let raw_data = objects["objects"].as_array()?;
    let mut walls: Vec<Wall> = vec![];
    for wall in raw_data {
        let x = wall["x"].as_f64()? as f32;
        let y = wall["y"].as_f64()? as f32;
        let w = wall["width"].as_f64()? as f32;
        let h = wall["height"].as_f64()? as f32;
        let elevation = get_elev(wall).unwrap();

        let hitbox = Rect::new(
            x / TERRAIN_TILE_SIZE * STANDARD_SQUARE,
            y / TERRAIN_TILE_SIZE * STANDARD_SQUARE,
            w / TERRAIN_TILE_SIZE * STANDARD_SQUARE,
            h / TERRAIN_TILE_SIZE * STANDARD_SQUARE,
        );

        walls.push(Wall { hitbox, elevation });
    }
    Some(walls)
}

fn get_elev(object: &Value) -> Option<u8> {
    let properties = object["properties"].as_array()?;
    for property in properties {
        if property["name"].as_str()? == "elevation" {
            return Some(property["value"].as_u64()? as u8);
        }
    }
    None
}
