use crate::logic::*;
use macroquad::prelude::*;

const TARGET_WIDTH: f32 = 1600.;
const TARGET_HEIGHT: f32 = 900.;

const CAM_SPEED: f32 = 1. / 10.;

const SHEET_SIZE: u8 = 12;
const TERRAIN_TILE_SIZE: f32 = 16.;

impl Game {
    pub fn new_camera_offset(&mut self) {
        let cam_box = self.cam_box();
        let player_hitbox = self.player.hitbox();

        let screen_width = screen_width();
        let screen_height = screen_height();

        if cam_box.intersect(player_hitbox) == Some(player_hitbox) {
            return;
        }
        if player_hitbox.left() < cam_box.left() {
            self.cam_offset_x += CAM_SPEED * (cam_box.left() - player_hitbox.left()) / screen_width
        }
        if player_hitbox.right() > cam_box.right() {
            self.cam_offset_x +=
                CAM_SPEED * (cam_box.right() - player_hitbox.right()) / screen_width
        }
        if player_hitbox.top() < cam_box.top() {
            self.cam_offset_y -= CAM_SPEED * (cam_box.top() - player_hitbox.top()) / screen_height
        }
        if player_hitbox.bottom() > cam_box.bottom() {
            self.cam_offset_y -=
                CAM_SPEED * (cam_box.bottom() - player_hitbox.bottom()) / screen_height
        }
    }

    fn cam_box(&self) -> Rect {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let ratio_x = TARGET_WIDTH / screen_width;
        let ratio_y = TARGET_HEIGHT / screen_height;

        let cam_pos_x = -self.cam_offset_x * screen_width * ratio_x;
        let cam_pos_y = self.cam_offset_y * screen_height * ratio_y;

        let bound_x = screen_width - 1. * STANDARD_SQUARE;
        let bound_y = screen_height - 1. * STANDARD_SQUARE;

        Rect::new(
            cam_pos_x - bound_x / 2.,
            cam_pos_y - bound_y / 2.,
            bound_x,
            bound_y,
        )
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);
        let zoom_x = 1. / TARGET_WIDTH;
        let zoom_y = 1. / TARGET_HEIGHT;
        let camera: Camera2D = Camera2D {
            offset: vec2(self.cam_offset_x, self.cam_offset_y),
            target: vec2(0., 0.),
            zoom: vec2(zoom_x, zoom_y),
            ..Default::default()
        };
        set_camera(&camera);

        self.draw_terrain();
        self.draw_monsters();
        self.draw_player();
        self.draw_gates();
    }

    fn draw_monsters(&self) {
        for monster in self.maps[&self.current_map].enemies.iter() {
            let dest_size = Some(monster.animation.frame().dest_size * SCALE_FACTOR);
            let draw_param = DrawTextureParams {
                source: Some(monster.animation.frame().source_rect),
                dest_size,
                ..Default::default()
            };
            draw_texture_ex(
                &self.textures.slime,
                monster.pos_x,
                monster.pos_y,
                WHITE,
                draw_param,
            );
        }
    }

    fn draw_terrain(&self) {
        let map = &self.maps[&self.current_map];
        let mesh = &map.render_mesh;
        let screen_box = screen_box(self.cam_box());

        let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
        let mut index_y = 0;

        for y_coord in 0..map.bound.y {
            let mut index_x = 0;
            for x_coord in 0..map.bound.x {
                let (render_pos_x, render_pos_y) = (
                    x_coord as f32 * STANDARD_SQUARE,
                    y_coord as f32 * STANDARD_SQUARE,
                );

                if !screen_box.contains(vec2(render_pos_x, render_pos_y)) {
                    index_x += 1;
                    continue;
                }

                let (x, y) = to_index(mesh[index_y][index_x]);

                let source = Some(Rect {
                    x,
                    y,
                    w: TERRAIN_TILE_SIZE,
                    h: TERRAIN_TILE_SIZE,
                });

                let params = DrawTextureParams {
                    source,
                    dest_size,
                    ..Default::default()
                };

                draw_texture_ex(
                    &self.textures.terrain,
                    render_pos_x,
                    render_pos_y,
                    WHITE,
                    params,
                );
                index_x += 1;
            }
            index_y += 1;
        }
    }

    fn draw_player(&mut self) {
        let dest_size = Some(self.player.animation.frame().dest_size * SCALE_FACTOR);
        let draw_param = DrawTextureParams {
            source: Some(self.player.animation.frame().source_rect),
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(
            &self.textures.player,
            self.player.pos_x,
            self.player.pos_y,
            WHITE,
            draw_param,
        );
        if self.player.attack_cooldown > 0. {
            self.draw_weapon();
        }
    }

    fn draw_weapon(&self) {
        let rotation = self.player.get_weapon_angle();
        let draw_param = DrawTextureParams {
            source: Some(Rect::new(
                TILE_SIZE * 0.,
                TILE_SIZE * 8.,
                TILE_SIZE,
                TILE_SIZE,
            )),
            rotation,
            // This breaks the game for some reason
            //pivot: Some(draw_pos),
            dest_size: Some(Vec2 {
                x: STANDARD_SQUARE,
                y: STANDARD_SQUARE,
            }),
            ..Default::default()
        };
        draw_texture_ex(
            &self.textures.player,
            self.player.pos_x,
            self.player.pos_y,
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
    let x = (point % SHEET_SIZE - 1) as f32;
    let y = (point / SHEET_SIZE) as f32;

    (x * TERRAIN_TILE_SIZE, y * TERRAIN_TILE_SIZE)
}

fn screen_box(cam_box: Rect) -> Rect {
    let center = cam_box.center();

    let screen_width = screen_width();
    let screen_height = screen_height();

    // I don't know why you need to multiply everything by 2 for it to work,
    // It just works ok, don't ask
    Rect {
        x: center.x - screen_width * 1.3,
        y: center.y - screen_height * 1.3,
        w: screen_width * 2.6,
        h: screen_height * 2.6,
    }
}

// For debugging and prototyping purposes

#[allow(dead_code)]
pub trait Draw {
    fn draw(&self);
}

impl Draw for Rect {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, RED)
    }
}
