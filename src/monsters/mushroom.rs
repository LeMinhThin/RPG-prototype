use crate::logic::*;
use crate::player::*;
use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;

use super::{spawner::MobType, Entity, IsAMonster};

const MUSHROOM_HEALTH: f32 = 20.;
const MUSHROOM_TRACKING_RANGE: f32 = 500.;
const MUSHROOM_SPEED: f32 = 100.;

#[derive(Clone)]
pub struct Mushroom {
    pub props: Props,
    damage: f32,
}

impl IsAMonster for Mushroom {
    fn tick(&mut self, player: &mut Player, walls: &[Rect]) {
        let player_pos = player.props.get_pos();
        self.move_to(player_pos);
        self.damage_player(player);
        self.props.new_pos();
        self.wall_collsion(walls);

        self.change_anim();
    }

    fn tick_anim(&mut self) {
        self.props.animation.update()
    }

    fn max_health(&self) -> f32 {
        MUSHROOM_HEALTH
    }

    fn move_to(&mut self, player_pos: Vec2) {
        let dist = self.props.get_pos().distance(player_pos);
        if dist > MUSHROOM_TRACKING_RANGE {
            return;
        }

        self.props.move_to(player_pos, MUSHROOM_SPEED);
    }

    fn damage_player(&self, player: &mut Player) {
        if player.invul_time > 0. {
            return;
        }

        if let Some(_) = self.hitbox().intersect(player.hitbox()) {
            player.props.health -= self.damage;
            player.invul_time = INVUL_TIME;
        }
    }

    fn draw(&self, texture: &Textures) {
        let dest_size = Some(self.props.animation.frame().dest_size * SCALE_FACTOR);
        let draw_param = DrawTextureParams {
            source: Some(self.props.animation.frame().source_rect),
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(
            &texture["mushroom"],
            self.props.x,
            self.props.y,
            WHITE,
            draw_param,
        );
        self.draw_health_bar(&texture["ui"]);
    }

    fn change_anim(&mut self) {
        let movement_vector = self.props.movement_vector;

        if movement_vector.length() < 1. {
            self.props.animation.set_animation(1)
        } else if movement_vector.x >= 0. {
            self.props.animation.set_animation(0)
        } else {
            self.props.animation.set_animation(2)
        }
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
}

impl Mushroom {
    pub fn from(x: f32, y: f32) -> Self {
        let animation = mushroom_animation();
        let props = Props::from(x, y, MUSHROOM_HEALTH, animation);

        Mushroom { props, damage: 5. }
    }
}

impl Collidable for Mushroom {
    fn hitbox(&self) -> Rect {
        Rect {
            x: self.props.x,
            y: self.props.y,
            w: STANDARD_SQUARE,
            h: STANDARD_SQUARE,
        }
    }

    fn mut_pos(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.props.x, &mut self.props.y)
    }
    fn pos(&self) -> Vec2 {
        vec2(self.props.x, self.props.y)
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
