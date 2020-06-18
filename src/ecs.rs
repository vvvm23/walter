use std::sync::{Arc, Mutex};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u16,
}

#[derive(Debug)]
pub struct EntitySet {
    next_id: u16,
    entities: HashSet<Arc<Entity>>,
}

impl EntitySet {
    pub fn new() -> EntitySet {
        EntitySet {
            next_id: 0,
            entities: HashSet::new(),
        }
    }

    pub fn new_entity(&mut self) -> Arc<Entity> {
        assert!(self.next_id + 1 > self.next_id, "Maximum entity count reached!"); // TODO: Does this check for overflow?
        let entity: Entity = Entity { id: self.next_id };
        let entity: Arc<Entity> = Arc::new(entity);
        self.entities.insert(Arc::clone(&entity));
        self.next_id += 1;
        entity
    }
}

#[derive(Debug)]
pub struct World {

}

impl World {

}
