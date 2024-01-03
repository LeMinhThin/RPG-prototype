use crate::logic::*;
use crate::player::*;
use crate::ui::items::Item;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use macroquad::rand::rand;

use super::{spawner::MobType, Entity, IsAMonster};

const MUSHROOM_HEALTH: f32 = 20.;
const MUSHROOM_TRACKING_RANGE: f32 = 500.;
const MUSHROOM_SPEED: f32 = 100.;

#[derive(Clone)]
pub struct Mushroom {
    props: Props,
    damage: f32,
}

impl IsAMonster for Mushroom {
    fn tick(&mut self, player: &mut Player, walls: &[Rect]) {
        let player_pos = player.pos();
        self.move_to(player_pos);
        self.damage_player(player);
        self.props.new_pos();
        self.wall_collsion(walls);

        if self.props.health <= 0. {
            self.props.should_despawn = true
        }
    }

    fn tick_anim(&mut self) {
        self.props.animation.update();

        if self.props.is_moving() {
            self.props.animation.set_animation(0)
        } else {
            self.props.animation.set_animation(1)
        }
        if self.props.velocity.x > 0. {
            self.props.flip_sprite = false;
        }
        if self.props.velocity.x < 0. {
            self.props.flip_sprite = true
        }
    }

    fn max_health(&self) -> f32 {
        MUSHROOM_HEALTH
    }

    fn move_to(&mut self, player_pos: Vec2) {
        let dist = self.pos().distance(player_pos);
        if dist > MUSHROOM_TRACKING_RANGE {
            return;
        }

        self.props.move_to(player_pos, MUSHROOM_SPEED);
    }

    fn damage_player(&self, player: &mut Player) {
        if !player.invul_time.is_done() {
            return;
        }

        if let Some(_) = self.hitbox().intersect(player.hitbox()) {
            player.props.health -= self.damage;
            player.invul_time.repeat()
        }
    }

    fn draw(&self, texture: &Textures) {
        let dest_size = Some(self.props.animation.frame().dest_size * SCALE_FACTOR);
        let source = Some(self.props.animation.frame().source_rect);
        let draw_param = DrawTextureParams {
            source,
            dest_size,
            flip_x: self.props.flip_sprite,
            ..Default::default()
        };
        draw_texture_ex(
            &texture["mushroom"],
            self.props.pos.x,
            self.props.pos.y,
            WHITE,
            draw_param,
        );
        self.draw_health_bar(&texture["ui"]);
    }

    fn get_props(&self) -> &Props {
        &self.props
    }

    fn get_mut_props(&mut self) -> &mut Props {
        &mut self.props
    }

    fn get_type(&self) -> MobType {
        MobType::Mushroom
    }

    fn loot(&self) -> Option<Item> {
        let count = (rand() % 2) as u8;
        if count == 0 {
            return None;
        }
        Some(Item::mushroom(count))
    }
}

impl Mushroom {
    pub fn from(pos: Vec2) -> Self {
        let animation = mushroom_animation();
        let props = Props::from(pos, MUSHROOM_HEALTH, animation);

        Mushroom { props, damage: 5. }
    }
}

impl Collidable for Mushroom {
    fn hitbox(&self) -> Rect {
        Rect {
            x: self.props.pos.x,
            y: self.props.pos.y,
            w: STANDARD_SQUARE,
            h: STANDARD_SQUARE,
        }
    }

    fn mut_pos(&mut self) -> &mut Vec2 {
        &mut self.props.pos
    }
    fn pos(&self) -> Vec2 {
        self.props.pos
    }
}

impl Entity for Mushroom {}

fn mushroom_animation() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[
            make_anim("walk right", 0, 2, 4),
            make_anim("idle", 1, 2, 4),
            make_anim("walk left", 2, 2, 4),
        ],
        true,
    )
}
