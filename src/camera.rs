use macroquad::prelude::*;
use macroquad::rand::rand;

use std::f32::consts::PI;

use crate::monsters::*;
use crate::player::Collidable;
use crate::{logic::*, map::Area};

const CAM_SPEED: f32 = 1. / 10.;

const SHEET_SIZE: u8 = 12;
pub const TERRAIN_TILE_SIZE: f32 = 16.;

impl Game {
    pub fn new_camera_offset(&mut self) {
        let mut cam_box = self.cam_box();

        // Shift the camera box's center to the player's
        let player_pos = self.player.hitbox().center();
        let cam_center = cam_box.center();
        cam_box.x += player_pos.x -cam_center.x;
        cam_box.y += player_pos.y - cam_center.y;

        let bound_box = self.bound_box();

        // Inverse collision detection lmao
        if let Some(rect) = bound_box.intersect(cam_box) {
            if is_key_pressed(KeyCode::F3) {
                println!("{:#?}", cam_box);
                println!("{:#?}", bound_box);
            }
            if cam_box.top() < bound_box.top() {
                cam_box.y += cam_box.h - rect.h
            }
            if cam_box.bottom() > bound_box.bottom() {
                cam_box.y -= cam_box.h - rect.h
            }
            if cam_box.left() < bound_box.left() {
                cam_box.x +=cam_box.w - rect.w
            }
            if cam_box.right() > bound_box.right() {
                cam_box.x -= cam_box.w - rect.w
            }
            if is_key_pressed(KeyCode::F4) {
                println!("{:#?}", cam_box);
                println!("{:#?}", bound_box);
            }
        }

        self.set_offset(cam_box.center())
    }

    fn set_offset(&mut self, new_offset: Vec2) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let curent_offset = vec2(
            -self.cam_offset_x * screen_width,
            self.cam_offset_y * screen_height,
        );

        self.cam_offset_x += (curent_offset.x - new_offset.x) / screen_width * CAM_SPEED;
        self.cam_offset_y -= (curent_offset.y - new_offset.y) / screen_height * CAM_SPEED;
    }

    fn bound_box(&self) -> Rect {
        let bounds = &self.maps[&self.current_map].bound;
        let bounds = vec2(bounds.x as f32 * STANDARD_SQUARE, bounds.y as f32 * STANDARD_SQUARE);
        Rect::new(0., 0., bounds.x, bounds.y)

    }

    fn cam_box(&self) -> Rect {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let cam_pos_x = -self.cam_offset_x * screen_width;
        let cam_pos_y = self.cam_offset_y * screen_height;

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
            offset: vec2(self.cam_offset_x, self.cam_offset_y),
            target: vec2(0., 0.),
            zoom: vec2(zoom_x, zoom_y),
            ..Default::default()
        };
        set_camera(&camera);

        self.hud();
        self.draw_terrain();
        self.draw_monsters();
        self.draw_player();
        self.draw_gates();
        self.draw_decorations();
        self.cam_box().draw();

        if self.current_state == GameState::GUI {
            self.player.show_inv();
        }
        self.debug_draw();
    }

    fn debug_draw(&self) {
        let bounds = &self.maps[&self.current_map].bound;
        let bounds = vec2(
            bounds.x as f32 * STANDARD_SQUARE,
            bounds.y as f32 * STANDARD_SQUARE,
        );
        let bound_box = Rect::new(0., 0., bounds.x, bounds.y);
        bound_box.draw()
    }

    fn draw_monsters(&self) {
        for monster in self.maps[&self.current_map].enemies.iter() {
            monster.draw(&self.textures)
        }
    }

    fn draw_terrain(&self) {
        let map = &self.maps[&self.current_map];
        let screen_center = self.cam_box().center();
        map.draw_tiles(&self.textures.terrain, screen_center, "terrain");
    }

    fn draw_decorations(&self) {
        let map = &self.maps[&self.current_map];
        let screen_center = self.cam_box().center();
        map.draw_tiles(&self.textures.terrain, screen_center, "decorations");
    }

    fn draw_player(&mut self) {
        // Basicly this makes the player flash after it's hurt
        if self.player.invul_time > 0. {
            if rand() % 3 == 0 {
                return;
            }
        }
        let dest_size = Some(self.player.props.animation.frame().dest_size * SCALE_FACTOR);
        let draw_param = DrawTextureParams {
            source: Some(self.player.props.animation.frame().source_rect),
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(
            &self.textures.player,
            self.player.props.x,
            self.player.props.y,
            WHITE,
            draw_param,
        );
        if self.player.attack_cooldown > 0. {
            self.draw_weapon();
        }
    }

    fn draw_weapon(&self) {
        // Oh my god this is such a spaghetti mess
        let rotation = self.player.get_weapon_angle();
        let draw_pos = self.player.get_draw_pos();
        let slash_pos = self.player.slash_pos();
        let draw_param = DrawTextureParams {
            source: Some(Rect::new(
                TILE_SIZE * 0.,
                TILE_SIZE * 8.,
                TILE_SIZE,
                TILE_SIZE,
            )),
            rotation,
            dest_size: Some(Vec2 {
                x: STANDARD_SQUARE,
                y: STANDARD_SQUARE,
            }),
            ..Default::default()
        };
        draw_texture_ex(
            &self.textures.player,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            draw_param,
        );
        // draw slash sprite

        let dest_size = Some(vec2(STANDARD_SQUARE * 2., STANDARD_SQUARE));
        let draw_param = DrawTextureParams {
            source: Some(Rect::new(
                TILE_SIZE * 7.,
                TILE_SIZE * 0.,
                TILE_SIZE * 2.,
                TILE_SIZE,
            )),
            rotation: self.player.current_angle() + PI / 2.,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(
            &self.textures.player,
            slash_pos.x,
            slash_pos.y,
            WHITE,
            draw_param,
        );
    }

    fn draw_gates(&self) {
        let gates = &self.maps[&self.current_map].gates;

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
}

fn to_index(point: u8) -> (f32, f32) {
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
                // Id of 0 indicate that the tile is blank
                if source_id == 0 {
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

fn gen_draw_params(source_id: u8) -> DrawTextureParams {
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

impl Monster {
    pub fn draw(&self, texture: &Textures) {
        match self {
            Monster::Slime(slime) => slime.draw(&texture.slime),
            Monster::Mushroom(mushroom) => mushroom.draw(&texture.mushroom),
        }
    }
}
