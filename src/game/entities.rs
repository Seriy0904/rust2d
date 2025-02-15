use macroquad::{math::Vec2, texture::Texture2D};

pub struct SpritedEntity {
    pub pos: Vec2,
    pub texture: Texture2D,
    pub size: Vec2,
}
impl SpritedEntity {
    pub fn new(pos: Vec2, size: Vec2, texture: Texture2D) -> Self {
        return Self { pos, texture, size };
    }
}
