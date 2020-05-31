mod ecs;
mod rendering;
mod physics;
mod battle;

use std::rc::Rc;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{HealthComponent, VelocityComponent, PositionComponent, RenderablePrimitiveComponent, RenderableSpriteComponent, AudioComponent};
use ecs::{RotationComponent, RotationalVelocityComponent};
use ecs::{FighterComponent, StatsComponent};

use ggez::graphics;
use ggez::{Context, GameResult};

fn main() -> GameResult {
    // create empty world
    let mut world: ecs::World = ecs::World::new();

    // initialise window
    let ctx: &mut Context = &mut rendering::init_window(1600.0, 1200.0).unwrap();

    // create a test move:
    let test_move: Rc<ecs::Move> = Rc::new(ecs::Move::new(
        "Megidolaon".to_string(), "{source} let loose terrifying energy!".to_string(),
        "Extreme Almighty damage to all foes.".to_string(),
        None, Some(50),
        true, Some(120), None,
        None, None,
        true, Some(ecs::AreaTarget::Enemy),
        false, 0.0,
        1.0,
    ));

    let e_source: PartialEntity = ecs::World::create_entity()
        .add_component(Component::HealthComponent(HealthComponent::new(
            1000,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            Some(500), vec![test_move.clone()]
        )));
    world.build_entity(e_source);

    let e_target: PartialEntity = ecs::World::create_entity()
        .add_component(Component::HealthComponent(HealthComponent::new(
            500,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            Some(50), vec![test_move.clone()]
        )));
    world.build_entity(e_target);

    println!("{}", world.health_components.get(&1).unwrap().hp);
    world.fighter_components.get_mut(&0).unwrap().current_move = Some(world.fighter_components.get(&0).unwrap().moves[0].clone());
    battle::execute_move(&mut world, 0, 1);
    println!("{}", world.health_components.get(&1).unwrap().hp);

    Ok(())
}
