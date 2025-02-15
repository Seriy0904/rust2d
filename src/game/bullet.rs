use std::sync::Arc;

use macroquad::{
    math::{vec2, Vec2},
    texture::Texture2D,
    time::get_frame_time,
};

use super::entities::SpritedEntityData;
const BULLET_SPEED: f32 = 10.0;
pub struct Bullet {
    sprited_entity_data: SpritedEntityData,
    dir: Vec2,
}

impl Bullet {
    pub fn new(pos: Vec2, dir: Vec2, texture: Arc<Texture2D>) -> Self {
        return Self {
            sprited_entity_data: SpritedEntityData::new(pos, vec2(30.0, 30.0), texture),
            dir,
        };
    }
    pub fn update(&mut self) {
        let delta = get_frame_time();
        self.sprited_entity_data.pos += delta * self.dir * BULLET_SPEED;
    }
    pub fn draw(&self) -> &SpritedEntityData {
        return &self.sprited_entity_data;
    }
}
