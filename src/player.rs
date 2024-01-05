use crate::logic::*;
use crate::map::Projectile;
use crate::ui::inventory::Inventory;
use crate::weapons::Weapon;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::rand::rand;
use std::f32::consts::PI;

pub const INVUL_TIME: f32 = 1.0;
pub const PIXEL: f32 = (1. / TILE_SIZE) * STANDARD_SQUARE;
pub const PLAYER_HEALTH: f32 = 100.;
const PLAYER_VELOCITY: f32 = 350.;
const FRICTION: f32 = 25.;

// So, this is a hack that I have managed to come up with
#[derive(Clone, PartialEq, Copy)]
pub struct Attack {
    pub timer: Timer,
    pub mouse_pos: Vec2,
    pub attacked: bool,
}

#[derive(Clone, PartialEq)]
pub enum PlayerState {
    Transition,
    Normal,
    Throwing(f32),
    Attacking(Attack),
}
#[derive(Clone)]
pub struct Player {
    pub props: Props,
    pub held_weapon: Weapon,
    pub state: PlayerState,
    pub invul_time: Timer,
    pub facing: Orientation,
    pub inventory: Inventory,
    pub combo: u8,
}

#[derive(Clone)]
pub struct Props {
    pub velocity: Vec2,
    pub health: f32,
    pub animation: AnimatedSprite,
    pub pos: Vec2,
    pub should_despawn: bool,
    pub flip_sprite: bool,
}

#[derive(Clone, Debug)]
pub enum Orientation {
    Left,
    Right,
    Down,
    Up,
}

pub trait Collidable {
    fn hitbox(&self) -> Rect;
    fn mut_pos(&mut self) -> &mut Vec2;
    fn pos(&self) -> Vec2;
    fn wall_collsion(&mut self, walls: &[Rect]) {
        let hitbox = self.hitbox();
        let pos = self.mut_pos();

        for wall in walls {
            if let Some(rect) = wall.intersect(hitbox) {
                if rect.w < rect.h {
                    if hitbox.right() > wall.right() {
                        pos.x += rect.w
                    } else {
                        pos.x -= rect.w
                    }
                } else {
                    if hitbox.bottom() > wall.bottom() {
                        pos.y += rect.h
                    } else {
                        pos.y -= rect.h
                    }
                }
            }
        }
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Rect {
        Rect {
            x: self.props.pos.x + 6. * PIXEL,
            y: self.props.pos.y + 16. * PIXEL,
            w: 11. * PIXEL,
            h: 6. * PIXEL,
        }
    }

    fn mut_pos(&mut self) -> &mut Vec2 {
        &mut self.props.pos
    }

    fn pos(&self) -> Vec2 {
        self.hitbox().center()
    }
}

impl Props {
    pub fn from(pos: Vec2, heath: f32, animation: AnimatedSprite) -> Self {
        Props {
            velocity: vec2(0., 0.),
            health: heath,
            animation,
            pos,
            should_despawn: false,
            flip_sprite: false,
        }
    }

    pub fn new_pos(&mut self) {
        let delta_time = get_frame_time();
        self.pos.x += self.velocity.x * delta_time;
        self.pos.y += self.velocity.y * delta_time;

        self.velocity *= 1. - FRICTION * delta_time;
    }

    pub fn move_to(&mut self, point: Vec2, speed: f32) {
        let vector = vec2(point.x - self.pos.x, point.y - self.pos.y).normalize() * speed;
        self.velocity += vector
    }
    pub fn is_moving(&self) -> bool {
        self.velocity.length() > 10.
    }

    pub fn knockback(&mut self, knockback: Vec2) {
        self.velocity += knockback
    }
}

impl Player {
    pub fn new() -> Self {
        let animation = player_animations();
        Player {
            state: PlayerState::Normal,
            invul_time: Timer::new(INVUL_TIME),
            props: Props::from(vec2(0., 0.), PLAYER_HEALTH, animation),
            held_weapon: Weapon::sword(),
            facing: Orientation::Down,
            inventory: Inventory::empty(),
            combo: 0,
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
        self.props.velocity += movement_vector;

        self.props.new_pos();
    }

    pub fn tick(&mut self, mouse_pos: Vec2) {
        self.invul_time.tick();
        self.state_management();

        if self.state == PlayerState::Normal {
            self.new_pos();
            self.change_anim(self.props.is_moving());
            return;
        }
        if let PlayerState::Throwing(time) = self.state {
            let angle = angle_between(self.pos(), mouse_pos);
            self.facing = should_face(angle);
            self.change_anim(false);
            self.state = PlayerState::Throwing(time + get_frame_time());
            return;
        }
        if let PlayerState::Attacking(attack) = self.state {
            let prog = attack.progress();
            if prog > 0.5 {
                self.props.new_pos();
            }
        }
    }

    pub fn current_projectile(&self, mouse_pos: Vec2) -> Projectile {
        let angle = angle_between(self.pos(), mouse_pos);
        let vec = Vec2::from_angle(angle);
        Projectile::new(self.projectile_pos(mouse_pos), vec)
    }

    fn state_management(&mut self) {
        if let PlayerState::Attacking(mut attack) = self.state {
            attack.timer.tick();
            self.state = match attack.timer.is_done() {
                true => PlayerState::Normal,
                false => PlayerState::Attacking(attack),
            };
            self.change_anim(false);
            return;
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            self.state = PlayerState::Throwing(0.)
        }
        if is_mouse_button_released(MouseButton::Right) {
            self.state = PlayerState::Normal
        }
    }

    pub fn face(&mut self, mouse_pos: Vec2) {
        let angle = angle_between(self.pos(), mouse_pos);
        self.facing = should_face(angle);
    }

    pub fn attack(&mut self, mouse_pos: Vec2) {
        let timer = Timer::new(self.held_weapon.cooldown);
        let attack = Attack {
            timer,
            mouse_pos,
            attacked: false,
        };
        self.state = PlayerState::Attacking(attack);
        self.combo = (self.combo + 1) % 2
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

    fn weapon_angle(&self) -> f32 {
        let (timer, mouse_pos) = match self.state {
            PlayerState::Attacking(attack) => (attack.timer, attack.mouse_pos),
            _ => return 0.,
        };
        // I know it doesn't make sense but it works
        let arc_lenght = 7. * PI / 12.;
        let mut timer_progress = timer.time / timer.duration;
        if timer_progress < 0.5 {
            timer_progress = 0.
        } else {
            timer_progress = 1.
        }

        -angle_between(mouse_pos, self.pos()) + (arc_lenght / 2. - timer_progress * arc_lenght)
    }

    pub fn draw_weapon(&self, texture: &Texture2D) {
        let source = self.weapon_texture();
        if source.is_none() {
            return;
        }
        let rotation = self.weapon_angle();
        let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
        let params = DrawTextureParams {
            dest_size,
            source,
            rotation: rotation + 1. * PI / 4.,
            ..Default::default()
        };

        let pos = self.pos();
        let center = vec2(
            pos.x + (20. * PIXEL * rotation.cos()),
            pos.y + (20. * PIXEL * rotation.sin()),
        ) - STANDARD_SQUARE / 2.;

        draw_texture_ex(texture, center.x, center.y, WHITE, params);
    }

    pub fn draw_slash(&self, texture: &Texture2D, mouse_pos: Vec2, timer: Timer) {
        let progress = 1. - timer.time / timer.duration;
        if progress < 0.5 {
            return;
        }
        let rotation = -angle_between(self.pos(), mouse_pos) + 1. * PI / 6.;
        let source = Some(source_rect(progress));
        let dest_size = Some(vec2(2. * STANDARD_SQUARE, 2. * STANDARD_SQUARE));
        let params = DrawTextureParams {
            dest_size,
            source,
            rotation,
            ..Default::default()
        };

        let player_pos = self.pos();

        draw_texture_ex(
            texture,
            player_pos.x - STANDARD_SQUARE,
            player_pos.y - STANDARD_SQUARE,
            WHITE,
            params,
        )
    }

    pub fn draw(&self, texture: &Texture2D) {
        // Basicly this makes the player flash after it's been hurt
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
        draw_texture_ex(
            texture,
            self.props.pos.x,
            self.props.pos.y,
            WHITE,
            draw_param,
        );
    }

    fn projectile_pos(&self, mouse_pos: Vec2) -> Vec2 {
        let pos = self.pos();
        let angle = angle_between(pos, mouse_pos);
        let front = vec2(pos.x - 6. * PIXEL, pos.y - 14. * PIXEL);
        let back = vec2(pos.x - 28. * PIXEL, pos.y - 14. * PIXEL);
        if (angle < PI / 2. && angle > 0.) || (angle > -PI / 2. && angle < 0.) {
            return front;
        } else {
            return back;
        }
    }

    pub fn draw_held_proj(&self, texture: &Texture2D, mouse_pos: Vec2) {
        let pos = self.projectile_pos(mouse_pos);
        let source = Some(Rect::new(TILE_SIZE * 6., TILE_SIZE, TILE_SIZE, TILE_SIZE));
        let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
        let params = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(texture, pos.x, pos.y, WHITE, params)
    }

    pub fn draw_throw_indicator(&self, mouse_pos: Vec2, texture: &Texture2D, time: f32) {
        let pos = self.pos();
        let vec = (mouse_pos - pos).normalize() * 500.;
        let pos = vec2(
            pos.x + vec.x - STANDARD_SQUARE / 2.,
            pos.y + vec.y - STANDARD_SQUARE / 2.,
        );

        let source = Some(Rect::new(TILE_SIZE * 7., TILE_SIZE, TILE_SIZE, TILE_SIZE));
        let dest_size = Some(vec2(STANDARD_SQUARE, STANDARD_SQUARE));
        let rotation = time * 2. * PI;
        let params = DrawTextureParams {
            source,
            dest_size,
            rotation,
            ..Default::default()
        };
        draw_texture_ex(texture, pos.x, pos.y, WHITE, params)
    }

    fn weapon_texture(&self) -> Option<Rect> {
        let inv = &self.inventory.content[12];
        if let None = inv {
            return None;
        }
        return match inv.as_ref().unwrap().name() {
            "Rusty sword" => Some(Rect::new(0., 3. * TILE_SIZE, TILE_SIZE, TILE_SIZE)),
            _ => None,
        };
    }
}

impl Attack {
    pub fn progress(&self) -> f32 {
        1. - self.timer.time / self.timer.duration
    }
}

#[rustfmt::skip]
fn player_animations() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[
            make_anim("idle_down",  0, 6, 12),
            make_anim("idle_left",  1, 6, 12),
            make_anim("idle_up",    2, 6, 12),
            make_anim("idle_right", 3, 6, 12),
            make_anim("walk_down",  4, 6, 12),
            make_anim("walk_left",  5, 6, 12),
            make_anim("walk_up",    6, 6, 12),
            make_anim("walk_right", 7, 6, 12),
        ],
        true,
    )
}

pub fn angle_between(start_point: Vec2, end_point: Vec2) -> f32 {
    let vector = (end_point - start_point).normalize();
    vector.angle_between(vec2(1., 0.))
}

fn should_face(angle: f32) -> Orientation {
    // My god that's a mouthful
    if (angle > -PI / 4. && angle < 0.) || (angle < PI / 4. && angle > 0.) {
        Orientation::Right
    } else if angle > PI / 4. && angle < 3. * PI / 4. {
        Orientation::Up
    } else if (angle > 3. * PI / 4. && angle < PI) || (angle > -PI && angle < -3. * PI / 4.) {
        Orientation::Left
    } else {
        Orientation::Down
    }
}

fn source_rect(progress: f32) -> Rect {
    let mut ret_rect = Rect::new(0., 0., TILE_SIZE * 2., TILE_SIZE * 2.);
    if progress > 0.875 {
        ret_rect.x = 6. * TILE_SIZE;
        return ret_rect;
    }
    if progress > 0.75 {
        ret_rect.x = 4. * TILE_SIZE;
        return ret_rect;
    }
    if progress > 0.625 {
        ret_rect.x = 2. * TILE_SIZE;
    }
    ret_rect
}
