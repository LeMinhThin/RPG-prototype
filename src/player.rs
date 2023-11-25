use crate::logic::{STANDARD_SQUARE, TILE_SIZE};
use crate::weapons::Weapon;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

const PLAYER_VELOCITY: f32 = 300.;
const FOUR_PIXELS: f32 = (4. / TILE_SIZE) * STANDARD_SQUARE;

#[derive(Clone)]
pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub animation: AnimatedSprite,
    pub held_weapon: Weapon,
    pub heath: f32,
    pub attack_cooldown: f32,
    pub facing: Orientation,
}

impl Player {
    pub fn new() -> Self {
        let animation = animations();
        Player {
            pos_x: 0.,
            pos_y: 0.,
            held_weapon: Weapon::sword(),
            heath: 100.,
            attack_cooldown: 0.,
            animation,
            facing: Orientation::Down,
        }
    }

    pub fn hitbox(&self) -> Rect {
        Rect {
            x: self.pos_x + (9. / TILE_SIZE) * STANDARD_SQUARE,
            y: self.pos_y + (3. / TILE_SIZE) * STANDARD_SQUARE,
            w: STANDARD_SQUARE * (13. / TILE_SIZE),
            h: STANDARD_SQUARE * (21. / TILE_SIZE),
        }
    }

    pub fn weapon_hitbox(&self) -> Rect {
        let weapon_lenght = self.held_weapon.lenght;
        let player_hitbox = self.hitbox();
        let player_center = player_hitbox.center();
        let (w, h) = match self.facing {
            Orientation::Up | Orientation::Down => (2.* weapon_lenght, weapon_lenght),
            Orientation::Left | Orientation::Right => (weapon_lenght, 2. * weapon_lenght)
        };
        let (x, y) = match self.facing {
            Orientation::Up => (player_center.x - weapon_lenght, player_hitbox.top() - weapon_lenght),
            Orientation::Left => (player_hitbox.left() - weapon_lenght, player_center.y - weapon_lenght),
            Orientation::Down => (player_center.x - weapon_lenght, player_hitbox.bottom()),
            Orientation::Right => (player_hitbox.right(), player_center.y - weapon_lenght)
        };
        Rect::new(
            x,y,w,h
        )
    }

    pub fn new_pos(&mut self, delta_time: &f32) {
        if self.attack_cooldown > 0. {
            return;
        }
        let mut movement_vector: Vec2 = vec2(0., 0.);
        if is_key_down(KeyCode::W) {
            movement_vector.y += -1.
        }
        if is_key_down(KeyCode::A) {
            movement_vector.x += -1.
        }
        if is_key_down(KeyCode::S) {
            movement_vector.y += 1.
        }
        if is_key_down(KeyCode::D) {
            movement_vector.x += 1.
        }
        self.new_orientation(&movement_vector);
        if movement_vector == Vec2::ZERO {
            return;
        }
        movement_vector = movement_vector.normalize();
        self.pos_x += PLAYER_VELOCITY * movement_vector.x * delta_time;
        self.pos_y += PLAYER_VELOCITY * movement_vector.y * delta_time;
    }

    pub fn tick(&mut self) {
        let delta_time = get_frame_time();
        self.new_pos(&delta_time);
        if self.attack_cooldown > 0. {
            self.attack_cooldown -= delta_time;
        }
        if self.attack_cooldown < 0. {
            self.attack_cooldown = 0.
        }
        self.animation.update();
    }

    fn new_orientation(&mut self, movement_vector: &Vec2) {
        let dy = movement_vector.y as i8;
        let dx = movement_vector.x as i8;
        let row;
        if dy == 0 && dx == 0 {
            row = match self.facing {
                Orientation::Down => 0,
                Orientation::Left => 1,
                Orientation::Up => 2,
                Orientation::Right => 3,
            };
        } else {
            let facing = facing(&dy, &dx);
            self.facing = facing;
            row = match self.facing {
                Orientation::Down => 4,
                Orientation::Left => 5,
                Orientation::Up => 6,
                Orientation::Right => 7,
            };
        }
        self.animation.set_animation(row)
    }

    pub fn get_weapon_angle(&self) -> f32 {
        let mut angle = self.current_angle() + PI;

        let elapsed_time = self.held_weapon.cooldown - self.attack_cooldown;
        if elapsed_time < 3. * get_frame_time() {
            angle += PI
        }

        angle
    }

    // This is so unbelievably messy that I don't even want to begin to explain
    pub fn get_draw_pos(&self) -> Vec2 {
        let elapsed_time = self.held_weapon.cooldown - self.attack_cooldown;
        let player_hitbox = self.hitbox();

        let up = vec2(
            player_hitbox.left() - FOUR_PIXELS,
            player_hitbox.top() - STANDARD_SQUARE,
        );
        let right = vec2(player_hitbox.right(), player_hitbox.top());
        let left = vec2(player_hitbox.left() - STANDARD_SQUARE, player_hitbox.top());
        let down = vec2(player_hitbox.left() - FOUR_PIXELS, player_hitbox.bottom());

        if elapsed_time < 3. * get_frame_time() {
            return match self.facing {
                Orientation::Up => left,
                Orientation::Left => down,
                Orientation::Down => right,
                Orientation::Right => up,
            };
        }
        match self.facing {
            Orientation::Up => right,
            Orientation::Left => up,
            Orientation::Down => left,
            Orientation::Right => down,
        }
    }

    pub fn current_angle(&self) -> f32 {
        match self.facing {
            Orientation::Up => -PI / 2.,
            Orientation::Left => PI,
            Orientation::Down => PI / 2.,
            Orientation::Right => 0.,
        }
    }

    // When I die, delete all of these so people wouldn't know I was the author
    pub fn slash_pos(&self) -> Vec2 {
        let player_hitbox = self.hitbox();
        let player_center = player_hitbox.center();

        match self.facing {
            Orientation::Up => vec2(
                player_center.x - STANDARD_SQUARE,
                player_hitbox.top() - STANDARD_SQUARE + FOUR_PIXELS * 1.5,
            ),
            Orientation::Left => vec2(
                player_hitbox.left() - STANDARD_SQUARE - player_hitbox.w,
                player_center.y - STANDARD_SQUARE / 2.,
            ),
            Orientation::Down => vec2(
                player_center.x - STANDARD_SQUARE,
                player_hitbox.bottom() - FOUR_PIXELS,
            ),
            Orientation::Right => vec2(
                player_hitbox.right() - player_hitbox.w,
                player_center.y - STANDARD_SQUARE / 2.,
            ),
        }
    }
}

#[derive(Clone)]
pub enum Orientation {
    Left,
    Right,
    Down,
    Up,
}

fn facing(dy: &i8, dx: &i8) -> Orientation {
    if *dx != 0 {
        match dx {
            1 => Orientation::Right,
            -1 => Orientation::Left,
            _ => panic!("fuck"),
        }
    } else {
        match dy {
            -1 => Orientation::Up,
            1 => Orientation::Down,
            _ => panic!("fuck"),
        }
    }
}

fn animations() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[
            make_anim("idle_down", 0),
            make_anim("idle_left", 1),
            make_anim("idle_up", 2),
            make_anim("idle_right", 3),
            make_anim("walk_down", 4),
            make_anim("walk_left", 5),
            make_anim("walk_up", 6),
            make_anim("walk_right", 7),
        ],
        true,
    )
}
fn make_anim(name: &str, row: u32) -> Animation {
    Animation {
        name: name.to_string(),
        row,
        frames: 6,
        fps: 12,
    }
}
