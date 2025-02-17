use macroquad::{
    math::{vec2, Vec2},
    time::get_frame_time,
};

use super::{entities::SpritedEntityData, map::Map, textures::TextureManager};
const BULLET_SPEED: f32 = 10.0;
pub struct Bullet {
    sprited_entity_data: SpritedEntityData,
    dir: Vec2,
}

impl Bullet {
    pub fn new(pos: Vec2, dir: Vec2, texture_manager: &TextureManager) -> Self {
        return Self {
            sprited_entity_data: SpritedEntityData::new(
                pos,
                vec2(20.0, 20.0),
                texture_manager.bullet_texture.clone(),
            ),
            dir,
        };
    }
    pub fn update(&mut self, map: &Map) -> bool {
        let delta = get_frame_time();
        self.sprited_entity_data.pos += delta * self.dir * BULLET_SPEED;
        let map_pos = self.sprited_entity_data.pos / vec2(map.map_item_width, map.map_item_height);
        if map_pos.x >= 0.0
            && map_pos.x < map.map_size as f32
            && map_pos.y >= 0.0
            && map_pos.y < map.map_size as f32
        {
            if map.map[map_pos.y as usize][map_pos.x as usize] == 1 {
                return true;
            }
        }
        return false;
    }
    pub fn draw(&self) -> &SpritedEntityData {
        return &self.sprited_entity_data;
    }
}
