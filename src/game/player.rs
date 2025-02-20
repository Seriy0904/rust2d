use std::f32::consts::PI;

use macroquad::{miniquad::window::quit, prelude::*};

use super::{bullet::Bullet, map::Map, textures::TextureManager};

const PLAYER_RADIUS: f32 = 4.0;
const PLAYER_DEFAULT_SPEED: f32 = 20.0;

pub const ROTATE_SPEED_RADIANS: f32 = PI * 50.0;

const PLAYER_SHOOT_COOLDWON: f32 = 0.01;

pub struct Player {
    pub pos: Vec2,
    pub view_angle: f32,
    radius: f32,
    // glance_len: f32,
    dir: Vec2,
    speed: f32,
    cooldown: f32,
    pub cooldownleft: f32,
}
impl Player {
    pub fn new(x_pos: f32, y_pos: f32) -> Player {
        return Player {
            pos: Vec2::new(x_pos, y_pos),
            view_angle: 0.0,
            radius: PLAYER_RADIUS,
            dir: Vec2::ZERO,
            speed: PLAYER_DEFAULT_SPEED,
            cooldown: PLAYER_SHOOT_COOLDWON,
            cooldownleft: 0.0,
        };
    }
    pub fn update(
        &mut self,
        map: &Map,
        bullets: &mut Vec<Bullet>,
        texture_manager: &TextureManager,
    ) {
        self.shooting(texture_manager, bullets);
        self.movement(map);
        // self.check_hitting(bullets);
    }
    pub fn draw(&self) {
        self.draw_char();
    }
    fn draw_char(&self) {
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.radius,
            Color {
                r: 255.0,
                g: 255.0,
                b: 255.0,
                a: 255.0,
            },
        );
    }
    fn check_hitting(&self, bullets: &mut Vec<Bullet>) {
        for bullet in bullets {
            if (bullet.sprited_entity_data.pos - self.pos).length() < self.radius - 1.0 {
                quit();
            }
        }
    }
    fn shooting(&mut self, texture_manager: &TextureManager, bullets: &mut Vec<Bullet>) {
        if is_key_down(KeyCode::Space) {
            if self.cooldownleft == 0.0 {
                let dir = vec2(self.view_angle.cos(), self.view_angle.sin());
                let bullet = Bullet::new(self.pos + dir * self.radius, dir, &texture_manager);
                bullets.push(bullet);
                self.cooldownleft = self.cooldown;
            }
        }
        self.cooldownleft -= get_frame_time();
        if self.cooldownleft < 0.0 {
            self.cooldownleft = 0.0;
        }
    }

    fn check_for_collision(&mut self, map: &Map) {
        let pot_player_pos = self.pos + self.radius * self.dir.signum() + self.dir;
        let pot_map_pos =
            (pot_player_pos / Vec2::new(map.map_item_width, map.map_item_height)).floor();
        let map_pos = (self.pos / Vec2::new(map.map_item_width, map.map_item_height)).floor();
        if pot_map_pos.y < 0.0
            || pot_map_pos.y >= map.map_height as f32
            || pot_map_pos.x < 0.0
            || pot_map_pos.x >= map.map_width as f32
        {
            self.dir.x = 0.0;
            self.dir.y = 0.0;
            return;
        }
        if map.map[pot_map_pos.y as usize][map_pos.x as usize] == 1 {
            self.dir.y = 0.0;
        }
        if map.map[map_pos.y as usize][pot_map_pos.x as usize] == 1 {
            self.dir.x = 0.0;
        }
        if self.dir.y != 0.0
            && self.dir.x != 0.0
            && map.map[pot_map_pos.y as usize][pot_map_pos.x as usize] == 1
        {
            self.dir.x = 0.0;
            self.dir.y = 0.0;
        }
    }
    fn movement(&mut self, map: &Map) {
        let mut offset = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            offset.y += 1.0;
        }
        if is_key_down(KeyCode::S) {
            offset.y -= 1.0;
        }
        if is_key_down(KeyCode::A) {
            offset.x += 1.0;
        }
        if is_key_down(KeyCode::D) {
            offset.x -= 1.0;
        }
        let delta = get_frame_time();
        self.view_angle += -mouse_delta_position().x * ROTATE_SPEED_RADIANS * delta;
        if self.view_angle < 0.0 {
            self.view_angle = PI * 2.0;
        } else if self.view_angle > PI * 2.0 {
            self.view_angle = 0.00001;
        }
        self.dir = (offset.y * Vec2::new(self.view_angle.cos(), self.view_angle.sin())
            + offset.x * Vec2::new(self.view_angle.sin(), -self.view_angle.cos()))
            * self.speed
            * delta;
        self.check_for_collision(map);
        self.pos += self.dir;
        self.dir = Vec2::ZERO;
    }
}
