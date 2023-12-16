use std::collections::HashMap;

use macroquad::prelude::*;

use crate::monsters::*;
use crate::npc::NPC;
use crate::player::Collidable;
use crate::{logic::*, map::Area};
use spawner::SpawnerType;

const CAM_SPEED: f32 = 1. / 10.;

const SHEET_SIZE: u16 = 12;
const BLANK_TILE: u16 = 0;
pub const TERRAIN_TILE_SIZE: f32 = 16.;

impl Game {
    pub fn new_camera_offset(&mut self) {
        let mut cam_box = self.cam_box();

        // Shift the camera box's center to the player's
        let player_pos = self.player.hitbox().center();
        let cam_center = cam_box.center();
        cam_box.x += player_pos.x - cam_center.x;
        cam_box.y += player_pos.y - cam_center.y;

        let bound_box = self.bound_box();

        // Inverse collision detection lmao
        if let Some(rect) = bound_box.intersect(cam_box) {
            if cam_box.top() < bound_box.top() {
                cam_box.y += cam_box.h - rect.h
            }
            if cam_box.bottom() > bound_box.bottom() {
                cam_box.y -= cam_box.h - rect.h
            }
            if cam_box.left() < bound_box.left() {
                cam_box.x += cam_box.w - rect.w
            }
            if cam_box.right() > bound_box.right() {
                cam_box.x -= cam_box.w - rect.w
            }
        }
        // So uhm the camera will start to follow the player once the player has gone out of bound.
        // Since it would be quite nice to hide some easter eggs with it, this is a feauture now.
        self.set_offset(cam_box.center())
    }

    fn set_offset(&mut self, new_offset: Vec2) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let curent_offset = vec2(
            -self.cam_offset.x * screen_width,
            self.cam_offset.y * screen_height,
        );

        // Because S M O O T H
        self.cam_offset.x += (curent_offset.x - new_offset.x) / screen_width * CAM_SPEED;
        self.cam_offset.y -= (curent_offset.y - new_offset.y) / screen_height * CAM_SPEED;
    }

    fn bound_box(&self) -> Rect {
        let bounds = &self.maps[&self.current_map].bound;
        let bounds = vec2(
            bounds.x as f32 * STANDARD_SQUARE,
            bounds.y as f32 * STANDARD_SQUARE,
        );
        Rect::new(0., 0., bounds.x, bounds.y)
    }

    pub fn cam_box(&self) -> Rect {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let cam_pos_x = -self.cam_offset.x * screen_width;
        let cam_pos_y = self.cam_offset.y * screen_height;

        Rect::new(
            cam_pos_x - screen_width,
            cam_pos_y - screen_height,
            screen_width * 2.,
            screen_height * 2.,
        )
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);
        let zoom_x = 1. / screen_width();
        let zoom_y = 1. / screen_height();
        let camera: Camera2D = Camera2D {
            offset: self.cam_offset,
            target: vec2(0., 0.),
            zoom: vec2(zoom_x, zoom_y),
            ..Default::default()
        };
        set_camera(&camera);

        let current_map = &self.maps[&self.current_map];
        let player_pos = self.player.pos();

        self.hud();
        self.draw_terrain(current_map);
        self.draw_monsters(current_map);
        self.draw_player();
        //self.draw_gates(current_map);
        self.draw_decorations(current_map);
        self.draw_npcs(current_map, player_pos);

        match &self.current_state {
            GameState::Normal => (),
            GameState::Talking(_) => self.draw_dialog(&current_map.npcs),
            GameState::GUI => self.show_inv(),
            GameState::Transition(timer) => draw_transition(self.cam_box(), timer),
        }
    }

    fn draw_monsters(&self, map: &Area) {
        for monster in map.enemies.iter() {
            monster.draw(&self.textures)
        }
    }

    fn draw_terrain(&self, map: &Area) {
        let draw_type = "terrain";
        let screen_center = self.cam_box().center();
        map.draw_tiles(&self.textures[draw_type], screen_center, draw_type);
    }

    fn draw_decorations(&self, map: &Area) {
        let draw_type = "decorations";
        let screen_center = self.cam_box().center();
        map.draw_tiles(&self.textures["terrain"], screen_center, draw_type);
    }

    fn draw_player(&self) {
        self.player.draw(&self.textures["player"])
    }

    #[allow(dead_code)]
    // Used for debugging
    fn draw_gates(&self, map: &Area) {
        let gates = &map.gates;

        for gate in gates {
            draw_rectangle(
                gate.location.x,
                gate.location.y,
                gate.location.w,
                gate.location.h,
                GREEN,
            )
        }
    }

    fn draw_npcs(&self, map: &Area, player_pos: Vec2) {
        let npcs = &map.npcs;

        for npc in npcs {
            npc.draw(&self.textures[&npc.name]);

            if npc.hitbox.center().distance(player_pos) < STANDARD_SQUARE {
                npc.draw_overlay(&self.textures["ui"]);
            }
        }
    }

    fn draw_dialog(&self, npc: &[NPC]) {
        let index = match self.current_state {
            GameState::Talking(x) => x,
            _ => return,
        };

        let npc = npc.iter().find(|npc| npc.is_talking).unwrap();
        let text = &npc.dialogs[index];
        let diag_box = self.diag_box();
        self.draw_diag_box();
        render_text(diag_box, text, &self.font);
    }

    fn draw_diag_box(&self) {
        let diag_box = self.diag_box();

        draw_rectangle(diag_box.x, diag_box.y, diag_box.w, diag_box.h, GRAY);
    }

    fn diag_box(&self) -> Rect {
        let cam_box = self.cam_box();
        let diag_width = 1200.;
        let diag_height = 400.;

        Rect::new(
            cam_box.center().x - diag_width / 2.,
            cam_box.bottom() - diag_height,
            diag_width,
            diag_height,
        )
    }
}

fn to_index(point: u16) -> (f32, f32) {
    let x;
    let y;
    if point % SHEET_SIZE == 0 {
        x = SHEET_SIZE as f32 - 1.;
        y = (point / SHEET_SIZE) as f32 - 1.;
    } else {
        x = (point % SHEET_SIZE - 1) as f32;
        y = (point / SHEET_SIZE) as f32;
    }

    (x * TERRAIN_TILE_SIZE, y * TERRAIN_TILE_SIZE)
}

fn to_coord(x: usize, y: usize) -> (f32, f32) {
    let x = x as f32 * STANDARD_SQUARE;
    let y = y as f32 * STANDARD_SQUARE;
    (x, y)
}

fn screen_box(screen_center: Vec2) -> Rect {
    let center = screen_center;

    let screen_width = screen_width();
    let screen_height = screen_height();

    // I don't know why you need to multiply everything by 2 for it to work,
    // It just works ok, don't ask
    Rect {
        x: center.x - screen_width * 1.2,
        y: center.y - screen_height * 1.2,
        w: screen_width * 2.4,
        h: screen_height * 2.4,
    }
}

// For debugging and prototyping purposes

#[allow(dead_code)]
pub trait Draw {
    fn draw(&self);
}

impl Draw for Rect {
    fn draw(&self) {
        draw_line(self.left(), self.top(), self.left(), self.bottom(), 3., RED);
        draw_line(self.left(), self.top(), self.right(), self.top(), 3., RED);
        draw_line(
            self.left(),
            self.bottom(),
            self.right(),
            self.bottom(),
            3.,
            RED,
        );
        draw_line(
            self.right(),
            self.top(),
            self.right(),
            self.bottom(),
            3.,
            RED,
        )
    }
}

impl Area {
    fn draw_tiles(&self, texture: &Texture2D, screen_center: Vec2, to_draw: &str) {
        let cam_box = screen_box(screen_center);

        let mesh = match to_draw {
            "terrain" => &self.draw_mesh.terrain,
            "decorations" => &self.draw_mesh.decorations,
            x => panic!("you forgot to account for {x}"),
        };

        for y_coord in 0..self.bound.y {
            for x_coord in 0..self.bound.x {
                let source_id = mesh[y_coord][x_coord];

                if source_id == BLANK_TILE {
                    continue;
                }
                let params = gen_draw_params(source_id);
                let (x, y) = to_coord(x_coord, y_coord);

                if cam_box.contains(vec2(x, y)) {
                    draw_texture_ex(texture, x, y, WHITE, params)
                }
            }
        }
    }
}

fn gen_draw_params(source_id: u16) -> DrawTextureParams {
    let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
    let (x_index, y_index) = to_index(source_id);

    let source = Some(Rect::new(
        x_index,
        y_index,
        TERRAIN_TILE_SIZE,
        TERRAIN_TILE_SIZE,
    ));
    DrawTextureParams {
        dest_size,
        source,
        ..Default::default()
    }
}

fn render_text(diag_box: Rect, content: &str, font: &Font) {
    let lines: Vec<&str> = content.split("\n").collect();
    let font_size = 48;

    let params = TextParams {
        font_size,
        font: Some(&font),
        ..Default::default()
    };

    let mut offset = 0.;
    for line in lines {
        offset += font_size as f32 + 30.;
        draw_text_ex(line, diag_box.x, diag_box.y + offset, params.clone())
    }
}

fn draw_transition(screen: Rect, timer: &Timer) {
    let timer_progress: f32 = (timer.time / timer.duration) * 2. - 1.;
    draw_rectangle(
        screen.left() - screen.w * timer_progress,
        screen.top(),
        screen.w,
        screen.h,
        BLACK,
    )
}

impl Monster {
    pub fn draw(&self, texture: &HashMap<String, Texture2D>) {
        let monster = self.get();
        match monster.get_type() {
            SpawnerType::Slime => monster.draw(&texture["slime"]),
            SpawnerType::Mushroom => monster.draw(&texture["mushroom"]),
        }
    }
}
