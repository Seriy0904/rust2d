use std::sync::Arc;

use macroquad::texture::{load_texture, Texture2D};

pub struct TextureManager {
    pub bullet_texture: Arc<Texture2D>,
    pub enemy_texture: Arc<Texture2D>,
}
impl TextureManager {
    pub async fn new(bullet_texture: &str, enemy_texture: &str) -> Self {
        let bullet_texture = Arc::new(load_texture(bullet_texture).await.unwrap());
        let enemy_texture = Arc::new(load_texture(enemy_texture).await.unwrap());
        Self {
            bullet_texture,
            enemy_texture,
        }
    }
}
