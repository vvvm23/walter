use std::collections::HashMap;

use mint;

use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::audio;
use ggez::audio::SoundSource;

// Audio Component //
// TODO: decide whether to have file switching or use multiple components for each sound. <28-05-20, vvvm23> //
pub struct AudioComponent {
    music_path: String,
    music: audio::Source,
    repeat: bool,
}

impl AudioComponent {
    pub fn new(ctx: &mut Context, path: &str, repeat: bool) -> AudioComponent {
        AudioComponent {
            music_path: path.to_string(),
            music: audio::Source::new(ctx, path).unwrap(),
            repeat: repeat,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.music.set_volume(volume);
    }

    pub fn play(&mut self) {
        if self.repeat {
            self.music.repeat();
        }
        self.music.play_detached();
    }
}

// Enum for available primitive shapes.
pub enum Shape {
    Circle{ r: f32 }, // Radius
    Rectangle{ w: f32, h: f32 }, // width, height
}

// Component to draw simple shapes
pub struct RenderablePrimitiveComponent {
    shape: Shape,
    draw_mode: graphics::DrawMode,
    colour: graphics::Color,
}

impl RenderablePrimitiveComponent {
    pub fn new(shape: Shape, draw_mode: graphics::DrawMode, colour: graphics::Color) -> RenderablePrimitiveComponent {
        RenderablePrimitiveComponent {
            shape: shape,
            draw_mode: draw_mode,
            colour: colour,
        }
    }

    pub fn build_mesh(&self, ctx: &mut Context) -> graphics::Mesh {
        match self.shape {
            Shape::Circle{r} => graphics::Mesh::new_circle(
                ctx,
                self.draw_mode,
                na::Point2::new(0.0, 0.0),
                r,
                1.0,
                self.colour,
            ),
            Shape::Rectangle{w,h} => graphics::Mesh::new_rectangle(
                ctx,
                self.draw_mode,
                graphics::Rect {x: 0.0, y: 0.0, w: w, h: h},
                self.colour
            )
        }.unwrap()
    }
}

// Component to draw sprites //
// TODO: switchable sprite for animation, either in form of another component or in here <28-05-20, vvvm23> //
// TODO: share textures to reduce time fetching textures <28-05-20, vvvm23> //
// TODO: texture atlas? <28-05-20, vvvm23> //
pub struct RenderableSpriteComponent {
    pub texture_path: String,
    pub texture: graphics::Image,
    pub scale: mint::Vector2<f32>,
}

impl RenderableSpriteComponent {
    pub fn new(ctx: &mut Context, path: &str, scale_x: f32, scale_y: f32) -> RenderableSpriteComponent {
        RenderableSpriteComponent {
            texture_path: path.to_string(),
            texture: graphics::Image::new(ctx, path).unwrap(),
            scale: mint::Vector2{x: scale_x, y: scale_y},
        }
    }
}

// Component to define entity health points.
// All entities with HealthComponent can have health and can be killed.
// TODO: death callback <28-05-20, vvvm23> //
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

// Component to define position of entity
// When rendering, use this to define position. Else, default to origin.
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

    // translate by f32 args
    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    // translate by mint::Vector2
    pub fn translate_vector(&mut self, v: mint::Vector2<f32>) {
        self.x += v.x;
        self.y += v.y;
    }

    // translate by a ecs::VelocityComponent
    pub fn translate_component(&mut self, c: &VelocityComponent) {
        self.x += c.dx;
        self.y += c.dy;
    }
}

// Component to define entity speed
// All entities with velocity and position component will move based on velocity
// TODO: rotational velocity <28-05-20, vvvm23> //
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

    // convert to mint::Vector2 
    // TODO: overriding possible in rust? <28-05-20, vvvm23> //
    pub fn to_vector(&self) -> mint::Vector2<f32> {
        mint::Vector2 {x: self.dx, y: self.dy,}
    }
}

// Component Enum to allow for match later
// TODO: rework with traits perhaps <25-05-20, vvvm23> //
pub enum Component {
    HealthComponent(HealthComponent),
    VelocityComponent(VelocityComponent),
    PositionComponent(PositionComponent),
    RenderablePrimitiveComponent(RenderablePrimitiveComponent),
    RenderableSpriteComponent(RenderableSpriteComponent),
    AudioComponent(AudioComponent),
}

// Simple wrapper for entity ID
pub struct Entity {
    id: u16,
}

// Partially constructed entity, containing components added so far.
pub struct PartialEntity {
    components: Vec<Component>,
}

impl PartialEntity {
    pub fn add_component(mut self, component: Component) -> PartialEntity {
        self.components.push(component);
        self
    }
}

// Defines current world state, contains all components currently in world.
pub struct World {
    pub max_id: u16,
    pub entities: Vec<Entity>,

    pub health_components: HashMap<u16, HealthComponent>,
    pub position_components: HashMap<u16, PositionComponent>,
    pub velocity_components: HashMap<u16, VelocityComponent>,
    pub renderable_primitive_components: HashMap<u16, RenderablePrimitiveComponent>,
    pub renderable_sprite_components: HashMap<u16, RenderableSpriteComponent>,
    pub audio_components: HashMap<u16, AudioComponent>,
}

impl World {
    // Generate a new world with empty component tables
    pub fn new() -> World {
        World {
            max_id: 0,
            entities: Vec::new(),

            health_components: HashMap::new(),
            position_components: HashMap::new(),
            velocity_components: HashMap::new(),
            renderable_primitive_components: HashMap::new(),
            renderable_sprite_components: HashMap::new(),
            audio_components: HashMap::new(),
        }
    }

    // Return an entity builder
    pub fn create_entity() -> PartialEntity {
        PartialEntity {
            components: Vec::new(),
        }
    }

    // Given a partial entity, build it into a concrete one.
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
                Component::PositionComponent(pc) => {self.position_components.insert(
                    self.max_id,
                    pc,
                ); ()},
                Component::VelocityComponent(vc) => {self.velocity_components.insert(
                    self.max_id,
                    vc,
                ); ()},
                Component::RenderablePrimitiveComponent(rc) => {self.renderable_primitive_components.insert(
                    self.max_id,
                    rc,
                ); ()},
                Component::RenderableSpriteComponent(rc) => {self.renderable_sprite_components.insert(
                    self.max_id,
                    rc,
                ); ()},
                Component::AudioComponent(ac) => {self.audio_components.insert(
                    self.max_id,
                    ac,
                ); ()},
            }
        }

        self.max_id += 1;
    }

}
