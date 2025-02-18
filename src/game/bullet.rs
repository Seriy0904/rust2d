use macroquad::{
    math::{vec2, Vec2},
    time::get_frame_time,
};

use super::{entities::SpritedEntityData, map::Map, textures::TextureManager};
const BULLET_SPEED: f32 = 40.0;
pub struct Bullet {
    pub sprited_entity_data: SpritedEntityData,
    pub dir: Vec2,
    pub ricocher_count: usize,
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
            ricocher_count: 0,
        };
    }
    pub fn update(&mut self, map: &Map) -> bool {
        let delta = get_frame_time();
        let map_pos =
            (self.sprited_entity_data.pos) / vec2(map.map_item_width, map.map_item_height);
        let pot_map_pos = (self.sprited_entity_data.pos + delta * self.dir * BULLET_SPEED)
            / vec2(map.map_item_width, map.map_item_height);
        if pot_map_pos.x >= 0.0
            && pot_map_pos.x < map.map_size as f32
            && pot_map_pos.y >= 0.0
            && pot_map_pos.y < map.map_size as f32
        {
            if map.map[pot_map_pos.y as usize][map_pos.x as usize] == 1 {
                self.dir.y *= -1.0;
                self.ricocher_count += 1;
            }
            if map.map[map_pos.y as usize][pot_map_pos.x as usize] == 1 {
                self.dir.x *= -1.0;
                self.ricocher_count += 1;
            }
        }
        if self.ricocher_count >= 4 {
            return true;
        }
        self.sprited_entity_data.pos =
            self.sprited_entity_data.pos + delta * self.dir * BULLET_SPEED;
        return false;
    }
    pub fn draw(&self) -> &SpritedEntityData {
        return &self.sprited_entity_data;
    }
}
