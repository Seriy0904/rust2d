use std::sync::Arc;

use macroquad::{
    color::{Color, WHITE},
    math::Vec2,
    texture::Texture2D,
};

pub struct SpritedEntityData {
    pub pos: Vec2,
    pub texture: Arc<Texture2D>,
    pub size: Vec2,
    pub color: Color,
}
impl SpritedEntityData {
    pub fn new(pos: Vec2, size: Vec2, texture: Arc<Texture2D>) -> Self {
        return Self {
            pos,
            size,
            texture,
            color: WHITE,
        };
    }
    pub fn change_color(&mut self, color: Color) {
        self.color = color;
    }
}
