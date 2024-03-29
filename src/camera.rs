use crate::player::{Collidable, PlayerState, PIXEL};
use crate::ui::main_menu::MainMenu;
use crate::{logic::*, map::Area};
use macroquad::prelude::*;
use textwrap::Options;

const CAM_SPEED: f32 = 1. / 8.;

const SHEET_SIZE: u16 = 12;
const BLANK_TILE: u16 = 0;
pub const TERRAIN_TILE_SIZE: f32 = 16.;

impl Game {
    pub fn new_camera_offset(&mut self) {
        let mut cam_box = self.cam_box();

        // Shift the camera box's center to the player's
        let player_pos = self.player.hitbox().center();
        let cam_center = cam_box.center();
        cam_box.x += player_pos.x - cam_center.x;
        cam_box.y += player_pos.y - cam_center.y;

        let bound_box = self.bound_box();

        if bound_box.w < cam_box.w {
            cam_box = cam_box.center_on(vec2(bound_box.center().x, cam_box.center().y))
        } else {
            snap_x(&mut cam_box, bound_box)
        }

        if bound_box.h < cam_box.h {
            cam_box = cam_box.center_on(vec2(cam_box.center().x, bound_box.center().y))
        } else {
            snap_y(&mut cam_box, bound_box)
        }
        // So uhm, the camera will start to follow the player once the player has gone out of bound.
        // Since it would be quite nice to hide some easter eggs with it, this is a feauture now.
        self.set_offset(cam_box.center())
    }

    fn set_offset(&mut self, new_offset: Vec2) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let curent_offset = vec2(
            -self.cam_offset.x * screen_width,
            self.cam_offset.y * screen_height,
        );

        // Because S M O O T H
        self.cam_offset.x += (curent_offset.x - new_offset.x) / screen_width * CAM_SPEED;
        self.cam_offset.y -= (curent_offset.y - new_offset.y) / screen_height * CAM_SPEED;
    }

    fn bound_box(&self) -> Rect {
        let mesh = &self.maps[&self.current_map].draw_mesh.terrain;
        let bound_x = mesh[0].len();
        let bound_y = mesh.len();
        let bounds = vec2(bound_x as f32 * TILE, bound_y as f32 * TILE);
        Rect::new(0., 0., bounds.x, bounds.y)
    }

    pub fn cam_box(&self) -> Rect {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let cam_pos_x = -self.cam_offset.x * screen_width;
        let cam_pos_y = self.cam_offset.y * screen_height;

        Rect::new(
            cam_pos_x - screen_width,
            cam_pos_y - screen_height,
            screen_width * 2.,
            screen_height * 2.,
        )
    }

    pub fn draw(&mut self) {
        let zoom_x = 1. / screen_width();
        let zoom_y = 1. / screen_height();
        let camera: Camera2D = Camera2D {
            offset: self.cam_offset,
            target: vec2(0., 0.),
            zoom: vec2(zoom_x, zoom_y),
            ..Default::default()
        };
        set_camera(&camera);

        if let GameState::GUI(GUIType::MainMenu(menu)) = &self.state {
            self.draw_main_menu(menu);
            return;
        }

        self.draw_terrain();
        self.draw_monsters();
        self.draw_interactables();
        self.draw_player();
        self.draw_projectiles();
        self.draw_decorations();
        self.draw_npcs();
        self.draw_items();
        self.hud();

        match &self.state {
            GameState::Normal | GameState::Quit => (),
            GameState::GUI(gui) => self.draw_gui(gui),
            GameState::Talking(..) => self.draw_dialog(),
            GameState::Transition(transition) => draw_transition(self.cam_box(), &transition.timer),
        }
    }

    fn draw_gui(&self, gui: &GUIType) {
        match gui {
            GUIType::Inventory => self.show_inv(),
            GUIType::DeathScreen(death_screen) => {
                death_screen.draw_buttons(&self.textures["ui"], &self.font)
            }
            GUIType::MainMenu(_) => return, // This should not be reachable since it's already been covered
        }
    }

    fn draw_main_menu(&self, menu: &MainMenu) {
        menu.draw_background(&self.textures["menu_bg"], self.cam_box());
        menu.draw_buttons(&self.textures["ui"], &self.font);
    }

    fn draw_items(&self) {
        let items = &self.maps[&self.current_map].items;

        for item in items {
            item.draw(&self.textures["ui"])
        }
    }

    fn draw_interactables(&self) {
        let interactables = &self.maps[&self.current_map].interactables;
        let search_box = self.player.search_box();
        for item in interactables {
            item.draw(&self.textures["chest"]);
            if !item.hitbox().overlaps(&search_box) {
                continue;
            }
            item.draw_overlay(&self.textures["ui"])
        }
    }

    fn draw_projectiles(&self) {
        let projectiles = &self.maps[&self.current_map].projectiles;

        for projectile in projectiles {
            projectile.draw(&self.textures["player"])
        }
    }

    fn draw_monsters(&self) {
        let map = &self.maps[&self.current_map];
        for monster in map.enemies.iter() {
            monster.draw(&self.textures);
        }
    }

    fn draw_terrain(&self) {
        let screen = self.cam_box().shift(TILE, TILE);
        let mesh = &self.maps[&self.current_map].draw_mesh.terrain;
        let texture = &self.textures["terrain"];
        draw_tiles(mesh, vec2(0., 0.), texture, Some(screen), TERRAIN_TILE_SIZE);
    }

    fn draw_decorations(&self) {
        let screen = self.cam_box().shift(TILE, TILE);
        let mesh = &self.maps[&self.current_map].draw_mesh.decorations;
        let texture = &self.textures["terrain"];
        draw_tiles(mesh, vec2(0., 0.), texture, Some(screen), TERRAIN_TILE_SIZE);
    }

    fn draw_player(&self) {
        let player = &self.player;
        let mouse_pos = self.get_mouse_pos();
        match self.player.state {
            PlayerState::Attacking(attack) => {
                player.draw_weapon(&self.textures["ui"]);
                player.draw_slash(&self.textures["slash"], attack.mouse_pos, attack.timer);
            }
            PlayerState::Throwing(time) => {
                player.draw_held_proj(&self.textures["player"], mouse_pos);
                player.draw_throw_indicator(mouse_pos, &self.textures["player"], time);
            }
            _ => (),
        }
        player.draw(&self.textures["player"]);
    }

    #[allow(dead_code)]
    // Used for debugging
    fn draw_gates(&self, map: &Area) {
        let gates = &map.gates;

        for gate in gates {
            draw_rectangle(
                gate.hitbox.x,
                gate.hitbox.y,
                gate.hitbox.w,
                gate.hitbox.h,
                GREEN,
            )
        }
    }

    fn draw_npcs(&self) {
        let search_box = self.player.search_box();
        let map = &self.maps[&self.current_map];
        let npcs = &map.npcs;

        for npc in npcs {
            npc.draw(&self.textures[&npc.name]);

            if npc.hitbox.overlaps(&search_box) {
                npc.draw_overlay(&self.textures["ui"]);
            }
        }
    }

    fn draw_dialog(&self) {
        let (line, char) = match self.state {
            GameState::Talking(line, char) => (line, char),
            _ => return,
        };

        let npcs = &self.maps[&self.current_map].npcs;
        let npc = npcs.iter().find(|npc| npc.is_talking).unwrap();
        let text: String = npc.dialogs[line][..char].iter().collect();
        let mut diag_box = self.diag_box();
        self.draw_diag_box(&self.textures["ui"]);
        let params = TextParams {
            font_size: 50,
            font: Some(&self.font),
            color: BLACK,
            ..Default::default()
        };
        diag_box.x += 5. * PIXEL;
        render_text(diag_box, &text, params);
    }

    fn draw_diag_box(&self, texture: &Texture2D) {
        let diag_box = self.diag_box();
        let mesh = diag_mesh();

        draw_tiles(&mesh, diag_box.point(), texture, None, TILE_SIZE);
    }

    fn diag_box(&self) -> Rect {
        let cam_box = self.cam_box();
        let diag_width = 9. * TILE;
        let diag_height = 3. * TILE;

        Rect::new(
            cam_box.center().x - diag_width / 2.,
            cam_box.bottom() - diag_height,
            diag_width,
            diag_height,
        )
    }
}

fn to_index(point: &u16, tile_size: f32) -> (f32, f32) {
    let x;
    let y;
    if point % SHEET_SIZE == 0 {
        x = SHEET_SIZE as f32 - 1.;
        y = (point / SHEET_SIZE) as f32 - 1.;
    } else {
        x = (point % SHEET_SIZE - 1) as f32;
        y = (point / SHEET_SIZE) as f32;
    }

    (x * tile_size, y * tile_size)
}

// For debugging and prototyping purposes

#[allow(dead_code)]
pub trait Utils {
    fn draw(&self);
    fn shift(&self, x: f32, y: f32) -> Self;
    fn center_on(self, pos: Vec2) -> Self;
}

impl Utils for Rect {
    fn draw(&self) {
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 10., RED)
    }

    fn shift(&self, x: f32, y: f32) -> Self {
        Self {
            x: self.x - x,
            y: self.y - y,
            w: self.w + x,
            h: self.h + x,
        }
    }

    fn center_on(self, pos: Vec2) -> Self {
        let x = pos.x - self.w / 2.;
        let y = pos.y - self.h / 2.;
        Rect::new(x, y, self.w, self.h)
    }
}

fn gen_draw_params(source_id: &u16, tile_size: f32) -> DrawTextureParams {
    // Scale it by a really small factor
    let dest_size = Some(vec2(TILE, TILE) * 1.01f32);
    let (x_index, y_index) = to_index(source_id, tile_size);

    let source = Some(Rect::new(x_index, y_index, tile_size, tile_size));
    DrawTextureParams {
        dest_size,
        source,
        ..Default::default()
    }
}

pub fn render_text(diag_box: Rect, content: &str, params: TextParams) {
    let width = (diag_box.w / params.font_size as f32) * 1.5;
    let max_width = Options::new(width as usize);
    let lines = textwrap::wrap(content, max_width);
    let mut offset = PIXEL * 5.;
    for line in lines {
        offset += params.font_size as f32 * 1.5;
        draw_text_ex(&line, diag_box.x, diag_box.y + offset, params.clone())
    }
}

fn draw_transition(screen: Rect, timer: &Timer) {
    let timer_progress: f32 = (timer.time / timer.duration) * 2. - 1.;
    let new_width = screen.w * 2.0;
    draw_rectangle(
        screen.left() - new_width * timer_progress,
        screen.top(),
        new_width,
        screen.h,
        BLACK,
    )
}

pub fn draw_tiles(
    mesh: &Vec<Vec<u16>>,
    origin: Vec2,
    texture: &Texture2D,
    screen: Option<Rect>,
    tile_size: f32,
) {
    let mut row_num = 0.;
    let mut col_num = 0.;
    for slice in mesh {
        for cell in slice {
            let draw_pos = vec2(col_num * TILE, row_num * TILE);
            if should_skip(draw_pos, screen, cell) {
                col_num += 1.;
                continue;
            }
            let params = gen_draw_params(cell, tile_size);

            draw_texture_ex(
                &texture,
                origin.x + col_num * TILE,
                origin.y + row_num * TILE,
                WHITE,
                params,
            );
            col_num += 1.
        }
        col_num = 0.;
        row_num += 1.;
    }
}

fn should_skip(point: Vec2, screen: Option<Rect>, cell: &u16) -> bool {
    if let Some(rect) = screen {
        if cell == &BLANK_TILE {
            return true;
        }
        if !rect.contains(point) {
            return true;
        }
        return false;
    }
    return false;
}

fn diag_mesh() -> Vec<Vec<u16>> {
    vec![
        vec![7, 8, 8, 8, 8, 8, 8, 8, 9],
        vec![19, 20, 20, 20, 20, 20, 20, 20, 21],
        vec![31, 32, 32, 32, 32, 32, 32, 32, 33],
    ]
}
fn snap_y(cam_box: &mut Rect, bound_box: Rect) {
    if let Some(rect) = bound_box.intersect(*cam_box) {
        if cam_box.top() < bound_box.top() {
            cam_box.y += cam_box.h - rect.h
        }
        if cam_box.bottom() > bound_box.bottom() {
            cam_box.y -= cam_box.h - rect.h
        }
    }
}

fn snap_x(cam_box: &mut Rect, bound_box: Rect) {
    if let Some(rect) = bound_box.intersect(*cam_box) {
        if cam_box.left() < bound_box.left() {
            cam_box.x += cam_box.w - rect.w
        }
        if cam_box.right() > bound_box.right() {
            cam_box.x -= cam_box.w - rect.w
        }
    }
}
