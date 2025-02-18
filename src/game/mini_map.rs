use macroquad::{
    color::{Color, GREEN},
    shapes::{draw_circle, draw_rectangle},
};

use super::{entities::SpritedEntityData, map::Map};

pub struct MiniMap;
impl MiniMap {
    pub fn draw_map(&self, map: &Map) {
        for y in 0..map.map_height {
            for x in 0..map.map_width {
                if map.map[y][x] == 1 {
                    draw_rectangle(
                        (x as f32) * map.map_item_width,
                        (y as f32) * map.map_item_height,
                        map.map_item_width,
                        map.map_item_height,
                        Color {
                            r: 1.0,
                            g: 1.0,
                            b: 0.0,
                            a: 1.0,
                        },
                    );
                }
            }
        }
    }
    pub fn draw_entities(&self, entities: &Vec<&SpritedEntityData>) {
        for entity in entities {
            draw_circle(entity.pos.x, entity.pos.y, 2.0, GREEN);
        }
    }
}
