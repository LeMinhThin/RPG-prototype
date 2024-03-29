use crate::ui::items::Item;
use crate::{logic::*, player::*};
use macroquad::prelude::*;
use spawner::MobType;

pub mod mushroom;
pub mod slime;
pub mod spawner;

pub trait IsAMonster {
    fn loot(&self) -> Option<Item>;
    fn tick(&mut self, player: &mut Player, walls: &[Rect]);
    fn tick_anim(&mut self);
    fn damage_player(&self, player: &mut Player);
    fn move_to(&mut self, player_pos: Vec2);
    fn draw(&self, texture: &Textures);
    fn get_props(&self) -> &Props;
    fn get_mut_props(&mut self) -> &mut Props;
    fn get_type(&self) -> MobType;
    fn max_health(&self) -> f32;
    fn draw_health_bar(&self, texture: &Texture2D) {
        let props = self.get_props();
        if props.health == self.max_health() {
            return;
        }
        if props.health <= 0. {
            return;
        }
        let source = Some(Rect::new(
            2. * TILE_SIZE,
            0. * TILE_SIZE,
            TILE_SIZE,
            TILE_SIZE,
        ));
        let dest_size = Some(vec2(TILE, TILE));
        let draw_param = DrawTextureParams {
            source,
            dest_size,
            ..Default::default()
        };
        draw_texture_ex(
            texture,
            props.pos.x,
            props.pos.y - TILE / 2.,
            WHITE,
            draw_param,
        );
        let health_percentage = props.health / self.max_health();
        let heath_bar = Rect::new(
            props.pos.x + PIXEL,
            props.pos.y - PIXEL,
            22. * health_percentage * PIXEL,
            2. * PIXEL,
        );

        draw_rectangle(heath_bar.x, heath_bar.y, heath_bar.w, heath_bar.h, RED)
    }
}

pub trait Entity: IsAMonster + Collidable {}
