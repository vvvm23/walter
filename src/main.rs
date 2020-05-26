mod ecs;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{RenderableComponent, HealthComponent, VelocityComponent, PositionComponent};

fn main() {
    let mut world: ecs::World = ecs::World::new();

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderableComponent(RenderableComponent::new(
            "epic".to_string()
        )))
        .add_component(Component::HealthComponent(HealthComponent::new(
            100,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            50.0, 50.0,
        )))
        .add_component(Component::VelocityComponent(VelocityComponent::new(
            1.0, 1.0,
        )));
    world.build_entity(e);
    
}
