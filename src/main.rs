mod ecs;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{RenderableComponent, HealthComponent, VelocityComponent, PositionComponent};

fn velocity_system(world: &mut ecs::World) {
    for (id, c) in world.velocity_components.iter() {
        println!("{} {} {}", id, c.dx, c.dy);
        if (world.position_components.contains_key(id)) {
            let pc: &mut ecs::PositionComponent = world.position_components.get_mut(id).unwrap();
            println!("{}, {}", pc.x, pc.y);
            pc.translate_component(c);
            println!("{}, {}", pc.x, pc.y);
        }
    }
}

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
    velocity_system(&mut world);
}
