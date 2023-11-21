use crate::logic::{STANDARD_SQUARE, TILE_SIZE};
use crate::weapons::Weapon;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

const PLAYER_VELOCITY: f32 = 300.;

#[derive(Clone)]
pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub animation: AnimatedSprite,
    pub held_weapon: Weapon,
    pub heath: f32,
    pub attack_cooldown: u8,
    facing: Orientation,
}

impl Player {
    pub fn new() -> Self {
        let animation = animations();
        Player {
            pos_x: 0.,
            pos_y: 0.,
            held_weapon: Weapon::sword(),
            heath: 100.,
            attack_cooldown: 0,
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

    pub fn weapon_hitbox(&mut self) -> Rect {
        Rect::new(
            self.pos_x - STANDARD_SQUARE,
            self.pos_y - STANDARD_SQUARE,
            self.held_weapon.lenght,
            self.held_weapon.lenght * 2.,
        )
    }

    pub fn new_pos(&mut self, delta_time: &f32) {
        if self.attack_cooldown > 0 {
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
        if self.attack_cooldown > 0 {
            self.attack_cooldown -= 1;
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
