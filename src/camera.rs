use crate::logic::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

const TARGET_WIDTH: f32 = 1280.;
const TARGET_HEIGHT: f32 = 720.;

const CAM_SPEED: f32 = 1. / 10.;

impl Game {
    pub fn new_camera_offset(&mut self) {
        let cam_box = self.cam_box();
        let player_hitbox = self.player.hitbox();

        let screen_width = screen_width();
        let screen_height = screen_height();

        if cam_box.intersect(player_hitbox) == Some(player_hitbox) {
            return;
        }
        if player_hitbox.x < cam_box.x {
            self.cam_offset_x += CAM_SPEED * (cam_box.x - player_hitbox.x) / screen_width
        }
        if player_hitbox.x + STANDARD_SQUARE > cam_box.x + cam_box.w {
            self.cam_offset_x += CAM_SPEED
                * ((cam_box.x + cam_box.w) - (player_hitbox.x + player_hitbox.w))
                / screen_width
        }
        if player_hitbox.y < cam_box.y {
            self.cam_offset_y -= CAM_SPEED * (cam_box.y - player_hitbox.y) / screen_height
        }
        if player_hitbox.y + STANDARD_SQUARE > cam_box.y + cam_box.h {
            self.cam_offset_y -= CAM_SPEED
                * ((cam_box.y + cam_box.h) - (player_hitbox.y + player_hitbox.h))
                / screen_height
        }
    }

    fn cam_box(&self) -> Rect {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let ratio_x = TARGET_WIDTH / screen_width;
        let ratio_y = TARGET_HEIGHT / screen_height;

        let cam_pos_x = -self.cam_offset_x * screen_width * ratio_x;
        let cam_pos_y = self.cam_offset_y * screen_height * ratio_y;

        let bound_x = screen_width - 2. * STANDARD_SQUARE;
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
        let screen_width = screen_width();
        let screen_height = screen_height();
        let zoom_x = (1. / screen_width) * (screen_width / TARGET_WIDTH);
        let zoom_y = (1. / screen_height) * (screen_height / TARGET_HEIGHT);
        let camera: Camera2D = Camera2D {
            offset: vec2(self.cam_offset_x, self.cam_offset_y),
            target: vec2(0., 0.),
            zoom: vec2(zoom_x, zoom_y),
            ..Default::default()
        };
        set_camera(&camera);

        self.draw_walls();
        self.draw_monsters();
        self.draw_player();
        self.draw_gates();
    }

    fn draw_monsters(&self) {
        for monster in self.maps[&self.current_map].enemies.iter() {
            draw_rectangle(
                monster.pos_x,
                monster.pos_y,
                STANDARD_SQUARE,
                STANDARD_SQUARE,
                RED,
            )
        }
    }

    fn draw_walls(&self) {
        for wall in self.maps[&self.current_map].walls.iter() {
            draw_rectangle(wall.x, wall.y, wall.w, wall.h, BLUE);
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
        if self.player.attack_cooldown > 0 {
            self.draw_weapon();
        }
    }

    fn draw_weapon(&self) {
        let draw_param = DrawTextureParams {
            source: Some(Rect::new(
                TILE_SIZE * 1.,
                TILE_SIZE * 0.,
                TILE_SIZE,
                TILE_SIZE,
            )),
            rotation: -PI * 0.5,
            dest_size: Some(Vec2 {
                x: STANDARD_SQUARE,
                y: STANDARD_SQUARE,
            }),
            ..Default::default()
        };
        draw_texture_ex(
            &self.textures.player,
            &self.player.pos_x - STANDARD_SQUARE,
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
