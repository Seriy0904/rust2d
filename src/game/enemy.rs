use super::entities::SpritedEntityData;

pub struct Enemy {
    pub sprited_entity_data: SpritedEntityData,
}
impl Enemy {
    pub fn new(sprited_entity_data: SpritedEntityData) -> Self {
        return Self {
            sprited_entity_data,
        };
    }
}
