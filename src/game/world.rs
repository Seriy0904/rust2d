use macroquad::{
    color::Color,
    math::Vec2,
    shapes::{draw_line, draw_rectangle},
    text::draw_text,
    texture::load_texture,
    window::{screen_height, screen_width},
};

use super::{camera::Camera, entities::SpritedEntity, map::Map, player::Player};

pub const VIEW_LENGTH: f32 = 300.0;

pub struct World {
    player: Player,
    map: Map,
    camera: Camera,
    enemies: Vec<SpritedEntity>,
}
impl World {
    pub async fn new() -> World {
        let player = Player::new(20.0, 20.0);
        let map: Map = Map::new();
        let camera = Camera::new(VIEW_LENGTH);
        let texture = load_texture("src\\assets\\chel.png").await.unwrap();

        let enemies = vec![
            SpritedEntity::new(
                Vec2::new(90.0, 90.0),
                Vec2::new(texture.width() / 3.0, texture.height() / 3.0),
                texture.clone(),
            ),
            SpritedEntity::new(
                Vec2::new(120.0, 100.0),
                Vec2::new(texture.width() / 3.0, texture.height() / 3.0),
                texture,
            ),
        ];
        return World {
            player,
            map,
            camera,
            enemies,
        };
    }
    pub fn update(&mut self) {
        self.player.update(&self.map);
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
        self.camera.draw_enemies(&mut self.enemies);
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
