use std::collections::HashMap;

use mint;

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

pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

impl PositionComponent {
    pub fn new(_x: f32, _y: f32) -> PositionComponent {
        PositionComponent {
            x: _x,
            y: _y,
        }
    }

    pub fn to_point(&self) -> na::Point2<f32> {
        na::Point2::new(self.x, self.y)
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn translate_vector(&mut self, v: mint::Vector2<f32>) {
        self.x += v.x;
        self.y += v.y;
    }

    pub fn translate_component(&mut self, c: &VelocityComponent) {
        self.x += c.dx;
        self.y += c.dy;
    }
}

pub struct VelocityComponent {
    pub dx: f32,
    pub dy: f32,
}

impl VelocityComponent {
    pub fn new(x: f32, y: f32) -> VelocityComponent {
        VelocityComponent {
            dx: x,
            dy: y,
        }
    }

    pub fn to_vector(&self) -> mint::Vector2<f32> {
        mint::Vector2 {x: self.dx, y: self.dy,}
    }
}

// TODO: rework with traits perhaps <25-05-20, vvvm23> //
pub enum Component {
    RenderableComponent(RenderableComponent),
    HealthComponent(HealthComponent),
    VelocityComponent(VelocityComponent),
    PositionComponent(PositionComponent),
}

pub struct Entity {
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
    pub entities: Vec<Entity>,

    pub renderable_components: HashMap<u16, RenderableComponent>,
    pub health_components: HashMap<u16, HealthComponent>,
    pub position_components: HashMap<u16, PositionComponent>,
    pub velocity_components: HashMap<u16, VelocityComponent>,
}

impl World {
    pub fn new() -> World {
        World {
            max_id: 0,
            entities: Vec::new(),

            renderable_components: HashMap::new(),
            health_components: HashMap::new(),
            position_components: HashMap::new(),
            velocity_components: HashMap::new(),
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
            // TODO: Is it though? maybe return a failed state enum instead! <26-05-20, vvvm23> //
            match c {
                Component::HealthComponent(hc) => {self.health_components.insert(
                    self.max_id,
                    hc,
                ); ()},
                Component::RenderableComponent(rc) => {self.renderable_components.insert(
                    self.max_id,
                    rc,
                ); ()},
                Component::PositionComponent(pc) => {self.position_components.insert(
                    self.max_id,
                    pc,
                ); ()},
                Component::VelocityComponent(vc) => {self.velocity_components.insert(
                    self.max_id,
                    vc,
                ); ()},
            }
        }

        self.max_id += 1;
    }

}
