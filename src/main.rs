mod ecs;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{RenderableComponent, HealthComponent};

fn main() {
    println!("Aloha World!");

    let mut world: ecs::World = ecs::World::new();

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderableComponent(RenderableComponent::new(
            "epic".to_string()
        )))
        .add_component(Component::HealthComponent(HealthComponent::new(
            100,
        )));

    world.build_entity(e);
    
    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderableComponent(RenderableComponent::new(
            "not epic".to_string()
        )))
        .add_component(Component::HealthComponent(HealthComponent::new(
            1000,
        )));

    world.build_entity(e);

}
