use std::{sync::Arc, vec};

use macroquad::{
    camera,
    color::Color,
    math::{vec2, Vec2},
    shapes::{draw_line, draw_rectangle},
    text::draw_text,
    texture::load_texture,
    window::{screen_height, screen_width},
};

use super::{
    bullet::{self, Bullet},
    camera::Camera,
    enemy::Enemy,
    entities::SpritedEntityData,
    map::Map,
    player::Player,
    textures::TextureManager,
};

pub const VIEW_LENGTH: f32 = 300.0;

pub struct World {
    player: Player,
    map: Map,
    camera: Camera,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    texture_manager: TextureManager,
}
impl World {
    pub async fn new() -> World {
        let player = Player::new(20.0, 20.0);
        let map: Map = Map::new();
        let camera = Camera::new(VIEW_LENGTH);
        let texture_manager =
            TextureManager::new("src\\assets\\bullet.png", "src\\assets\\chel.png").await;
        let enemies = vec![
            Enemy::new(SpritedEntityData::new(
                Vec2::new(90.0, 90.0),
                Vec2::new(
                    texture_manager.enemy_texture.width() / 3.0,
                    texture_manager.enemy_texture.height() / 3.0,
                ),
                texture_manager.enemy_texture.clone(),
            )),
            Enemy::new(SpritedEntityData::new(
                Vec2::new(120.0, 100.0),
                Vec2::new(
                    texture_manager.enemy_texture.width() / 3.0,
                    texture_manager.enemy_texture.height() / 3.0,
                ),
                texture_manager.enemy_texture.clone(),
            )),
        ];
        let bullets = vec![];
        return World {
            player,
            map,
            camera,
            enemies,
            bullets: bullets,
            texture_manager,
        };
    }
    pub fn update(&mut self) {
        self.player
            .update(&self.map, &mut self.bullets, &self.texture_manager);
        for bullet in &mut self.bullets {
            bullet.update();
        }
        self.camera.set_angle(self.player.view_angle);
        self.camera.set_pos(self.player.pos);
        self.draw();
    }
    fn draw(&mut self) {
        self.draw_env();
        self.camera.draw_map(&self.map);
        self.draw_hud();
        self.map.draw();
        self.player.draw();
        let all_entites = self.collect_all_entites();
        self.camera.draw_all_entites(all_entites);
    }
    fn collect_all_entites(&self) -> Vec<&SpritedEntityData> {
        let mut entities = vec![];
        for enemy in &self.enemies {
            entities.push(enemy.draw());
        }
        for bullet in &self.bullets {
            entities.push(bullet.draw());
        }
        return entities;
    }
    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }
    fn draw_hud(&self) {
        draw_rectangle(
            0.0,
            screen_height() - 100.0,
            screen_width(),
            100.0,
            Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
        );
        draw_line(
            100.0,
            screen_height() - 100.0,
            100.0,
            screen_height(),
            5.0,
            Color {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 1.0,
            },
        );
        draw_text(
            "100 HP",
            120.0,
            screen_height() - 30.0,
            70.0,
            Color {
                r: 0.8,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        );
    }
    fn draw_env(&self) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height() / 2.0,
            Color {
                r: 0.27,
                g: 0.52,
                b: 0.73,
                a: 1.0,
            },
        );
        let y_offset = screen_height() / (2.0 * 100.0);
        let mut y_start = screen_height() / 2.0;
        while y_start < screen_height() {
            draw_rectangle(
                0.0,
                y_start,
                screen_width(),
                y_start,
                Color {
                    r: ((y_start + 30.0 - (screen_height() / 2.0)) / (screen_height() / 2.0)) * 1.0
                        + 0.02,
                    g: ((y_start + 30.0 - (screen_height() / 2.0)) / (screen_height() / 2.0)) * 1.0
                        + 0.02,
                    b: ((y_start + 30.0 - (screen_height() / 2.0)) / (screen_height() / 2.0)) * 1.0
                        + 0.02,
                    a: 1.0,
                },
            );
            y_start += y_offset;
        }
    }
}
