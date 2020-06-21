use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashSet, HashMap};

use crate::component as component;
use crate::system as system;

#[derive(PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u16,
}

pub struct PartialEntity {
    components: Vec<component::Component>,
}

impl PartialEntity {
    pub fn new() -> PartialEntity {
        PartialEntity {
            components: Vec::new(),
        }
    }

    pub fn add_component(mut self, c: component::Component) -> PartialEntity {
        self.components.push(c);
        self
    }
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
    pub battle_instance: Option<Arc<RwLock<system::battle::BattleInstance>>>,

    // TODO: Compress all components into one point
    pub position_components: HashMap<Arc<Entity>, Arc<RwLock<component::physics::PositionComponent>>>,
    pub velocity_components: HashMap<Arc<Entity>, Arc<RwLock<component::physics::VelocityComponent>>>,
    pub primitive_components: HashMap<Arc<Entity>, Arc<RwLock<component::rendering::PrimitiveRenderableComponent>>>,
    pub sprite_components: HashMap<Arc<Entity>, Arc<RwLock<component::rendering::SpriteRenderableComponent>>>,
    pub background_components: HashMap<Arc<Entity>, Arc<RwLock<component::rendering::BackgroundComponent>>>,
    pub fighter_components: HashMap<Arc<Entity>, Arc<RwLock<component::battle::FighterComponent>>>,
    pub text_box_components: HashMap<Arc<Entity>, Arc<RwLock<component::rendering::TextBoxComponent>>>,
}

impl World {
    pub fn new() -> World {
        World {
            entity_set: EntitySet::new(),
            battle_instance: None,

            position_components:        HashMap::new(),
            velocity_components:        HashMap::new(),
            primitive_components:       HashMap::new(),
            sprite_components:          HashMap::new(),
            background_components:      HashMap::new(),
            fighter_components:         HashMap::new(),
            text_box_components:        HashMap::new(),
        }
    }

    pub fn build_entity(&mut self, pe: PartialEntity) -> Arc<Entity> {
        use component::Component as Component;

        let e = self.entity_set.new_entity();
        for c in pe.components {
            match c {
                Component::PositionComponent(r) => {
                    self.position_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                },
                Component::VelocityComponent(r) => {
                    self.velocity_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                },
                Component::PrimitiveRenderableComponent(r) => {
                    self.primitive_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                }
                Component::SpriteRenderableComponent(r) => {
                    self.sprite_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                },
                Component::BackgroundComponent(r) => {
                    self.background_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                },
                Component::FighterComponent(r) => {
                    self.fighter_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                },
                Component::TextBoxComponent(r) => {
                    self.text_box_components.insert(Arc::clone(&e), Arc::new(RwLock::new(r))); ()
                },
            }
        }
        Arc::clone(&e)
    }
}
