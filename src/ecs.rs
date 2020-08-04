/// Walter Engine ECS System 
/// 
/// First we define a Component trait. All components must implement this trait.
/// This lets us store references to components in an Entity together in one HashMap
/// We can also store the actual components in one vector in the main state.
/// This will let us define any number of Components without having to alter
/// the main state.
/// 
/// However, we still wish to have some way to iterate over all components of one 
/// type. We can instead keep a HashMap<ComponentType, Vec<Arc<Component>>> in the
/// main state. We can achieve the same effect as one vector if we find a way to
/// iterate over the enum ComponentType.
///
/// Entity will contain data that applies to all entities (such as position, moving 
/// out of the previous PositionComponent). It will also contain a lookup to 
/// components associated with that Entity. With this, we can lookup the components
/// of an entity from the Entity struct itself or from one of its components.
///
/// New entities must be created through EntityAllocator to ensure consistency 
/// with ids.
///

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait Component {
    fn get_owner(&self) -> Entity;
    fn update(&mut self) {}
}
impl std::fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Fighter,
    Inventory,
    Playable,
}

#[derive(Debug)]
pub struct Entity {
    pub id: u16,
    pub x: f64,
    pub y: f64,
    components: HashMap<ComponentType, Arc<dyn Component>>,
}

impl Entity {
    fn new(id: u16) -> Self {
        Entity {
            id: id,
            x: 0.0,
            y: 0.0,
            components: HashMap::new()
        }
    }
    
    fn get_component(&self, c: ComponentType) -> Option<Arc<dyn Component>> {
        match self.components.get(&c) {
            Some(a) => Some(Arc::clone(a)),
            None => None,
        }
    }
}

const MAX_ENTITIES: u16 = 256;
pub struct EntityAllocator {
    entities: Vec<Entity>,

}

impl EntityAllocator {
    pub fn new() -> Self {
        EntityAllocator {
            entities: Vec::with_capacity(MAX_ENTITIES as usize),
        }
    }
}
