use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashSet, HashMap};

use crate::component as component;

#[derive(PartialEq, Eq, Hash)]
pub struct Entity {
    id: u16,
}

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

pub struct World {
    pub entity_set: EntitySet,

    // TODO: Compress all components into one point
    pub position_components: HashMap<Arc<Entity>, Arc<RwLock<component::physics::PositionComponent>>>,
    pub velocity_components: HashMap<Arc<Entity>, Arc<RwLock<component::physics::VelocityComponent>>>,
}

impl World {
    pub fn new() -> World {
        World {
            entity_set: EntitySet::new(),

            position_components: HashMap::new(),
            velocity_components: HashMap::new(),
        }
    }
}
