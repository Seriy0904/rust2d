use std::sync::Arc;

use macroquad::{math::Vec2, texture::Texture2D};

pub struct SpritedEntityData {
    pub pos: Vec2,
    pub texture: Arc<Texture2D>,
    pub size: Vec2,
}
impl SpritedEntityData {
    pub fn new(pos: Vec2, size: Vec2, texture: Arc<Texture2D>) -> Self {
        return Self { pos, size, texture };
    }
}
