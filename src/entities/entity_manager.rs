use crate::entities::entity::Entity;

pub struct EntityManager {
    entities: Vec<Entity>,
    player_idx: usize,
}

// Constructor
impl EntityManager {
    pub fn new(player: Entity) -> Self {
        Self {
            entities: vec![player],
            player_idx: 0,
        }
    }
}

// Queries
impl EntityManager {
    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn get_player(&self) -> &Entity {
        &self.entities[self.player_idx]
    }

    pub fn get_player_mut(&mut self) -> &mut Entity {
        &mut self.entities[self.player_idx]
    }

    pub fn get_entity(&self, id: usize) -> &Entity {
        &self.entities[id]
    }

    pub fn get_entity_mut(&mut self, id: usize) -> &mut Entity {
        &mut self.entities[id]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.iter_mut()
    }
}
