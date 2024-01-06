use crate::logic::*;
use crate::player::{Collidable, Player, Props, PIXEL};
use crate::ui::items::Item;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::rand::rand;

use super::{spawner::MobType, Entity, IsAMonster};

const SLIME_HEALTH: f32 = 50.;
const SLIME_SPEED: f32 = 150.;
const SLIME_MAX_TRACKING: f32 = 500.;

#[derive(Clone)]
pub struct Slime {
    props: Props,
    damage: f32,
}
impl IsAMonster for Slime {
    fn tick(&mut self, player: &mut Player, walls: &[Rect]) {
        if self.props.health <= 0. {
            // Lmao wonky math
            if self.props.animation.frame().source_rect.x / TILE_SIZE > 5.5 {
                self.props.should_despawn = true
            }
            return;
        }
        let player_pos = player.props.pos;

        self.move_to(player_pos);
        self.damage_player(player);
        self.props.new_pos();
        self.wall_collsion(walls);
    }

    fn tick_anim(&mut self) {
        self.props.animation.update();
        if self.props.health <= 0. {
            if self.props.animation.current_animation() != 2 {
                self.props.animation.set_animation(2);
                self.props.animation.set_frame(0);
            }
            return;
        }
        if !self.props.is_moving() {
            self.props.animation.set_animation(0);
        } else {
            self.props.animation.set_animation(1);
        }
        if self.props.velocity.x > 0. {
            self.props.flip_sprite = false
        }
        if self.props.velocity.x < 0. {
            self.props.flip_sprite = true
        }
    }

    fn max_health(&self) -> f32 {
        SLIME_HEALTH
    }

    fn move_to(&mut self, player_pos: Vec2) {
        let dist = self.pos().distance(player_pos);

        if dist > SLIME_MAX_TRACKING {
            return;
        }
        self.props.move_to(player_pos, SLIME_SPEED)
    }

    fn damage_player(&self, player: &mut Player) {
        if !player.invul_time.is_done() {
            return;
        }
        let self_hitbox = self.damage_box();
        let player_hitbox = player.hitbox();

        if let Some(_) = self_hitbox.intersect(player_hitbox) {
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
            &texture["slime"],
            self.props.pos.x,
            self.props.pos.y,
            WHITE,
            draw_param,
        );
        self.draw_health_bar(&texture["ui"])
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

    fn loot(&self) -> Option<Item> {
        let count = (rand() % 2) as u8;
        if count == 0 {
            return None;
        }
        Some(Item::slime(count))
    }
}

impl Slime {
    pub fn from(pos: Vec2) -> Self {
        let animation = slime_animations();
        let props = Props::from(pos, SLIME_HEALTH, animation);
        Slime { props, damage: 10. }
    }

    fn damage_box(&self) -> Rect {
        let pos = self.props.pos;
        Rect::new(
            pos.x + 4. * PIXEL,
            pos.y + 12. * PIXEL,
            16. * PIXEL,
            12. * PIXEL,
        )
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
            w: TILE,
            h: TILE,
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
        &[
            make_anim("idle", 0, 4, 8),
            make_anim("moving", 1, 6, 8),
            make_anim("dying", 2, 7, 8),
        ],
        true,
    )
}
