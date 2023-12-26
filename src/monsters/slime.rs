use crate::logic::*;
use crate::player::{Collidable, Player, Props};
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

use super::{spawner::MobType, Entity, IsAMonster};

const SLIME_HEALTH: f32 = 50.;
const SLIME_SPEED: f32 = 150.;
const SLIME_MAX_TRACKING: f32 = 500.;

#[derive(Clone)]
pub struct Slime {
    pub props: Props,
    damage: f32,
}
impl IsAMonster for Slime {
    fn tick(&mut self, player: &mut Player, walls: &[Rect]) {
        let player_pos = player.pos();

        self.move_to(player_pos);
        self.damage_player(player);
        self.props.new_pos();
        self.wall_collsion(walls);

        self.change_anim()
    }

    fn tick_anim(&mut self) {
        self.props.animation.update();
    }

    fn max_health(&self) -> f32 {
        SLIME_HEALTH
    }

    fn move_to(&mut self, player_pos: Vec2) {
        // May looks dawnting but it's just the Pythagoras theorem
        let dist = ((self.props.pos.x - player_pos.x).powi(2)
            + (self.props.pos.y - player_pos.y).powi(2))
        .sqrt();

        if dist > SLIME_MAX_TRACKING {
            return;
        }
        self.props.move_to(player_pos, SLIME_SPEED)
    }

    fn damage_player(&self, player: &mut Player) {
        if !player.invul_time.is_done() {
            return;
        }
        let self_hitbox = self.hitbox();
        let player_hitbox = player.hitbox();

        if let Some(_) = self_hitbox.intersect(player_hitbox) {
            player.props.health -= self.damage;
            player.invul_time.repeat()
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
            &texture["slime"],
            self.props.pos.x,
            self.props.pos.y,
            WHITE,
            draw_param,
        );
        self.draw_health_bar(&texture["ui"])
    }

    fn change_anim(&mut self) {
        let speed = self.props.velocity.length();

        if speed < 1. {
            self.props.animation.set_animation(0);
        } else {
            self.props.animation.set_animation(1);
        }
    }

    fn get_props(&self) -> &Props {
        &self.props
    }

    fn get_mut_props(&mut self) -> &mut Props {
        &mut self.props
    }

    fn get_type(&self) -> MobType {
        MobType::Slime
    }
}

impl Slime {
    pub fn from(pos: Vec2) -> Self {
        let animation = slime_animations();
        let props = Props::from(pos, SLIME_HEALTH, animation);
        Slime { props, damage: 10. }
    }
}

impl Collidable for Slime {
    fn mut_pos(&mut self) -> &mut Vec2 {
        &mut self.props.pos
    }

    fn hitbox(&self) -> Rect {
        Rect {
            x: self.props.pos.x,
            y: self.props.pos.y,
            w: STANDARD_SQUARE,
            h: STANDARD_SQUARE,
        }
    }

    fn pos(&self) -> Vec2 {
        self.props.pos
    }
}

impl Entity for Slime {}

fn slime_animations() -> AnimatedSprite {
    AnimatedSprite::new(
        TILE_SIZE as u32,
        TILE_SIZE as u32,
        &[make_anim("idle", 0, 4, 8), make_anim("moving", 1, 6, 8)],
        true,
    )
}
