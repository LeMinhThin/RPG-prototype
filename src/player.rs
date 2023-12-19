use crate::logic::*;
use crate::weapons::Weapon;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::rand::rand;
use std::f32::consts::PI;

pub const INVUL_TIME: f32 = 1.0;
pub const PIXEL: f32 = (1. / TILE_SIZE) * STANDARD_SQUARE;
pub const PLAYER_HEALTH: f32 = 100.;
const PLAYER_VELOCITY: f32 = 350.;
const FRICTION: f32 = 1. / 2.;

#[derive(Clone, PartialEq)]
pub enum PlayerState {
    Transition,
    Normal,
    Attacking(Timer),
}
#[derive(Clone)]
pub struct Player {
    pub props: Props,
    pub held_weapon: Weapon,
    pub state: PlayerState,
    pub invul_time: Timer,
    pub facing: Orientation,
}

#[derive(Clone)]
pub struct Props {
    pub movement_vector: Vec2,
    pub health: f32,
    pub animation: AnimatedSprite,
    pub x: f32,
    pub y: f32,
}

pub trait Collidable {
    fn hitbox(&self) -> Rect;
    fn mut_pos(&mut self) -> (&mut f32, &mut f32);
    fn pos(&self) -> Vec2;
    fn wall_collsion(&mut self, walls: &[Rect]) {
        let hitbox = self.hitbox();
        let (pos_x, pos_y) = self.mut_pos();

        for wall in walls {
            let rect: Rect;
            if let Some(x) = wall.intersect(hitbox) {
                rect = x
            } else {
                continue;
            }

            if rect.w < rect.h {
                if hitbox.right() > wall.right() {
                    *pos_x += rect.w
                } else {
                    *pos_x -= rect.w
                }
            } else {
                if hitbox.bottom() > wall.bottom() {
                    *pos_y += rect.h
                } else {
                    *pos_y -= rect.h
                }
            }
        }
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Rect {
        Rect {
            x: self.props.x + 10. * PIXEL,
            y: self.props.y + 16. * PIXEL,
            w: 11. * PIXEL,
            h: 6. * PIXEL,
        }
    }

    fn mut_pos(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.props.x, &mut self.props.y)
    }

    fn pos(&self) -> Vec2 {
        self.hitbox().center()
    }
}

impl Props {
    pub fn from(x: f32, y: f32, heath: f32, animation: AnimatedSprite) -> Self {
        Props {
            movement_vector: vec2(0., 0.),
            health: heath,
            animation,
            x,
            y,
        }
    }

    pub fn get_pos(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    pub fn new_pos(&mut self) {
        let delta_time = get_frame_time();
        self.x += self.movement_vector.x * delta_time;
        self.y += self.movement_vector.y * delta_time;

        self.movement_vector *= FRICTION;
    }

    pub fn move_to(&mut self, point: Vec2, speed: f32) {
        let vector = vec2(point.x - self.x, point.y - self.y).normalize() * speed;
        self.movement_vector += vector
    }
    fn is_moving(&self) -> bool {
        self.movement_vector.length() > 10.
    }
}

impl Player {
    pub fn new() -> Self {
        let animation = player_animations();
        Player {
            state: PlayerState::Normal,
            invul_time: Timer::new(INVUL_TIME),
            props: Props::from(0., 0., PLAYER_HEALTH, animation),
            held_weapon: Weapon::sword(),
            facing: Orientation::Down,
        }
    }

    pub fn weapon_hitbox(&self) -> Rect {
        let weapon_lenght = self.held_weapon.lenght;
        let player_hitbox = self.hitbox();
        let player_center = player_hitbox.center();
        let (w, h) = match self.facing {
            Orientation::Up | Orientation::Down => (2. * weapon_lenght, weapon_lenght),
            Orientation::Left | Orientation::Right => (weapon_lenght, 2. * weapon_lenght),
        };
        #[rustfmt::skip]
        let (x, y) = match self.facing {
            Orientation::Up    => (player_center.x - weapon_lenght, player_hitbox.top() - weapon_lenght),
            Orientation::Left  => (player_hitbox.left() - weapon_lenght, player_center.y - weapon_lenght),
            Orientation::Down  => (player_center.x - weapon_lenght, player_hitbox.bottom()),
            Orientation::Right => (player_hitbox.right(), player_center.y - weapon_lenght),
        };
        Rect::new(x, y, w, h)
    }

    pub fn new_pos(&mut self) {
        if self.state != PlayerState::Normal {
            return;
        }
        let mut movement_vector: Vec2 = vec2(0., 0.);
        if is_key_down(KeyCode::W) {
            movement_vector.y += -1.;
            self.facing = Orientation::Up;
        }
        if is_key_down(KeyCode::S) {
            movement_vector.y += 1.;
            self.facing = Orientation::Down;
        }
        if is_key_down(KeyCode::A) {
            movement_vector.x += -1.;
            self.facing = Orientation::Left;
        }
        if is_key_down(KeyCode::D) {
            movement_vector.x += 1.;
            self.facing = Orientation::Right;
        }
        if movement_vector != Vec2::ZERO {
            movement_vector = movement_vector.normalize() * PLAYER_VELOCITY;
        }
        self.props.movement_vector += movement_vector;

        self.props.new_pos();
    }

    pub fn tick(&mut self) {
        if let PlayerState::Attacking(mut timer) = self.state {
            timer.tick();
            self.state = match timer.is_done() {
                true => PlayerState::Normal,
                false => PlayerState::Attacking(timer),
            };
            self.change_anim(false);
            return;
        }
        if self.state == PlayerState::Transition {
            return;
        }
        self.invul_time.tick();
        self.new_pos();
        self.change_anim(self.props.is_moving());
    }

    pub fn change_anim(&mut self, is_moving: bool) {
        let extra_row = match is_moving {
            true => 4,
            false => 0,
        };

        let row = match self.facing {
            Orientation::Down => 0,
            Orientation::Left => 1,
            Orientation::Up => 2,
            Orientation::Right => 3,
        };

        self.props.animation.set_animation(row + extra_row)
    }

    fn get_attack_cooldown(&self) -> f32 {
        match self.state {
            PlayerState::Attacking(timer) => timer.time,
            _ => panic!("called get_attack_cooldown while not in attacking state"),
        }
    }

    fn get_weapon_angle(&self) -> f32 {
        let cooldown = self.get_attack_cooldown();
        let mut angle = self.current_angle() + PI;

        let elapsed_time = self.held_weapon.cooldown - cooldown;
        if elapsed_time < 3. * get_frame_time() {
            angle += PI
        }

        angle
    }

    // This is so unbelievably messy that I don't even want to begin to explain
    fn get_draw_pos(&self) -> Vec2 {
        let elapsed_time = self.held_weapon.cooldown - self.get_attack_cooldown();
        let player_hitbox = self.abs_hitbox();

        #[rustfmt::skip]
        let up    = vec2(player_hitbox.left() - 4. * PIXEL, player_hitbox.top() - STANDARD_SQUARE);
        let right = vec2(player_hitbox.right(), player_hitbox.top());
        let left = vec2(player_hitbox.left() - STANDARD_SQUARE, player_hitbox.top());
        let down = vec2(player_hitbox.left() - 4. * PIXEL, player_hitbox.bottom());

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

    fn current_angle(&self) -> f32 {
        match self.facing {
            Orientation::Up => -PI / 2.,
            Orientation::Left => PI,
            Orientation::Down => PI / 2.,
            Orientation::Right => 0.,
        }
    }

    // When I die, delete all of these so people wouldn't know I was the author
    fn slash_pos(&self) -> Vec2 {
        let player_hitbox = self.abs_hitbox();
        let player_center = player_hitbox.center();

        match self.facing {
            Orientation::Up => vec2(
                player_center.x - STANDARD_SQUARE,
                player_hitbox.top() - STANDARD_SQUARE + 6. * PIXEL,
            ),
            Orientation::Left => vec2(
                player_hitbox.left() - STANDARD_SQUARE - player_hitbox.w,
                player_center.y - STANDARD_SQUARE / 2.,
            ),
            Orientation::Down => vec2(
                player_center.x - STANDARD_SQUARE,
                player_hitbox.bottom() - 4. * PIXEL,
            ),
            Orientation::Right => vec2(
                player_hitbox.right() - player_hitbox.w,
                player_center.y - STANDARD_SQUARE / 2.,
            ),
        }
    }

    // This method only exist so that I won't have to rewrite the already confusing enough
    // attacking code
    fn abs_hitbox(&self) -> Rect {
        let player_pos = &self.props;
        Rect {
            x: player_pos.x + 9. * PIXEL,
            y: player_pos.y + 3. * PIXEL,
            w: 13. * PIXEL,
            h: 19. * PIXEL,
        }
    }

    pub fn should_attack(&mut self) -> bool {
        if let PlayerState::Attacking(_) = self.state {
            return false;
        }
        if is_key_pressed(KeyCode::Space) {
            self.state = PlayerState::Attacking(Timer::new(self.held_weapon.cooldown));
            return true;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            self.state = PlayerState::Attacking(Timer::new(self.held_weapon.cooldown));
            return true;
        }
        false
    }

    pub fn draw(&self, texture: &Texture2D) {
        // Basicly this makes the player flash after it's hurt
        if !self.invul_time.is_done() {
            if rand() % 3 == 0 {
                return;
            }
        }
        let dest_size = Some(self.props.animation.frame().dest_size * SCALE_FACTOR);
        let draw_param = DrawTextureParams {
            source: Some(self.props.animation.frame().source_rect),
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(texture, self.props.x, self.props.y, WHITE, draw_param);

        if let PlayerState::Attacking(_) = self.state {
            self.draw_weapon(texture)
        }
    }

    fn draw_weapon(&self, texture: &Texture2D) {
        // Oh my god this is such a spaghetti mess
        let rotation = self.get_weapon_angle();
        let draw_pos = self.get_draw_pos();
        let slash_pos = self.slash_pos();
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
        draw_texture_ex(&texture, draw_pos.x, draw_pos.y, WHITE, draw_param);
        // draw slash sprite

        let dest_size = Some(vec2(STANDARD_SQUARE * 2., STANDARD_SQUARE));
        let draw_param = DrawTextureParams {
            source: Some(Rect::new(
                TILE_SIZE * 7.,
                TILE_SIZE * 0.,
                TILE_SIZE * 2.,
                TILE_SIZE,
            )),
            rotation: self.current_angle() + PI / 2.,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(&texture, slash_pos.x, slash_pos.y, WHITE, draw_param);
    }
}

#[derive(Clone)]
pub enum Orientation {
    Left,
    Right,
    Down,
    Up,
}

fn player_animations() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[
            make_anim("idle_down", 0, 6, 12),
            make_anim("idle_left", 1, 6, 12),
            make_anim("idle_up", 2, 6, 12),
            make_anim("idle_right", 3, 6, 12),
            make_anim("walk_down", 4, 6, 12),
            make_anim("walk_left", 5, 6, 12),
            make_anim("walk_up", 6, 6, 12),
            make_anim("walk_right", 7, 6, 12),
        ],
        true,
    )
}
