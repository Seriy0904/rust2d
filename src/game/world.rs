use std::vec;

use macroquad::{
    color::Color,
    math::Vec2,
    shapes::{draw_line, draw_rectangle},
    text::draw_text,
    window::{screen_height, screen_width},
};

use super::{
    bullet::Bullet, camera::Camera, enemy::Enemy, entities::SpritedEntityData, map::Map,
    mini_map::MiniMap, player::Player, textures::TextureManager,
};

pub const VIEW_LENGTH: f32 = 300.0;

pub struct World {
    player: Player,
    map: Map,
    mini_map: MiniMap,
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
        let texture_manager = TextureManager::new(
            "src\\assets\\rassengan.png",
            "src\\assets\\chel.png",
            // "src\\assets\\hui.png",
        )
        .await;
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
        let mini_map = MiniMap;
        return World {
            player,
            map,
            mini_map,
            camera,
            enemies,
            bullets: bullets,
            texture_manager,
        };
    }
    pub fn update(&mut self) {
        self.player
            .update(&self.map, &mut self.bullets, &self.texture_manager);
        let mut bullet_ind = 0;
        while bullet_ind < self.bullets.len() {
            if self.bullets[bullet_ind].update(&self.map) {
                self.bullets.remove(bullet_ind);
            }
            bullet_ind += 1;
        }
        self.camera.set_angle(self.player.view_angle);
        self.camera.set_pos(self.player.pos);
        self.camera.update();
        self.draw();
    }
    fn draw(&mut self) {
        self.camera.draw_env();
        self.camera.draw_walls(&self.map);
        self.mini_map.draw_map(&self.map);
        self.player.draw();
        let mut all_entites = self.collect_all_entites();
        self.camera.sort_by_camera(&mut all_entites);
        self.camera.draw_all_entites(&all_entites);
        self.mini_map.draw_entities(&all_entites);
        self.draw_hud();
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
        draw_text(
            &format!(
                "SECONDS: {}",
                ((self.player.cooldownleft * 10.0) as u32) as f32 / 10.0
            ),
            350.0,
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
}
