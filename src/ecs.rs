use std::collections::HashMap;

use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;

pub struct RenderableComponent {
    path: String,
}

impl RenderableComponent {
    pub fn new(sprite_path: String) -> RenderableComponent {
        let c = RenderableComponent {
            path: sprite_path,
        };
        c
    }
}

pub struct HealthComponent {
    hp: u16,
    alive: bool,
}

impl HealthComponent {
    pub fn new(max_hp: u16) -> HealthComponent {
        let c = HealthComponent {
            hp: max_hp,
            alive: true,
        };
        c
    }
}

// TODO: rework with traits perhaps <25-05-20, vvvm23> //
pub enum Component {
    RenderableComponent(RenderableComponent),
    HealthComponent(HealthComponent),
}

struct Entity {
    id: u16,
}

pub struct PartialEntity {
    components: Vec<Component>,
}

impl PartialEntity {
    pub fn add_component(mut self, component: Component) -> PartialEntity {
        self.components.push(component);
        self
    }
}

pub struct World {
    pub max_id: u16,
    entities: Vec<Entity>,

    renderable_components: HashMap<u16, RenderableComponent>,
    health_components: HashMap<u16, HealthComponent>,
}

impl World {
    pub fn new() -> World {
        World {
            max_id: 0,
            entities: Vec::new(),

            renderable_components: HashMap::new(),
            health_components: HashMap::new(),

        }
    }

    pub fn create_entity() -> PartialEntity {
        PartialEntity {
            components: Vec::new(),
        }
    }

    pub fn build_entity(&mut self, partial: PartialEntity) {
        let e: Entity = Entity {
            id: self.max_id,
        };
        self.entities.push(e);

        for c in partial.components {
            // TODO: A bit hacky here... <25-05-20, vvvm23> //
            match c {
                Component::HealthComponent(hc) => {self.health_components.insert(
                    self.max_id,
                    hc,
                ); ()},
                Component::RenderableComponent(rc) => {self.renderable_components.insert(
                    self.max_id,
                    rc,
                ); ()},
            }
        }

        self.max_id += 1;
    }
}
