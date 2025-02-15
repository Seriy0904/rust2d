use super::entities::SpritedEntityData;

pub struct Enemy {
    sprited_entity_data: SpritedEntityData,
}
impl Enemy {
    pub fn new(sprited_entity_data: SpritedEntityData) -> Self {
        return Self {
            sprited_entity_data,
        };
    }
    pub fn draw(&self) -> &SpritedEntityData {
        return &self.sprited_entity_data;
    }
}
