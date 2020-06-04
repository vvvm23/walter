mod ecs;
mod rendering;
mod physics;
mod battle;

use std::rc::Rc;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{HealthComponent, VelocityComponent, PositionComponent, RenderablePrimitiveComponent, RenderableSpriteComponent, AudioComponent};
use ecs::{RotationComponent, RotationalVelocityComponent};
use ecs::{FighterComponent};

use ggez::graphics;
use ggez::{Context, GameResult};

use ggez::nalgebra as na;

fn main() -> GameResult {
    // create empty world
    let mut world: ecs::World = ecs::World::new();

    // initialise window
    let ctx: &mut Context = &mut rendering::init_window(1600.0, 1200.0).unwrap();

    // create a test move:
    let test_move: Rc<ecs::Move> = Rc::new(ecs::Move::new(
        "Megidolaon".to_string(), "$source let loose terrifying energy!".to_string(),
        "Extreme Almighty damage to all foes.".to_string(),
        None, Some(50),
        true, Some(120), None,
        None, None,
        true, Some(ecs::AreaTarget::Enemy),
        true, 0.2,
        1.0,
    ));

    let test_move2: Rc<ecs::Move> = Rc::new(ecs::Move::new(
        "Psycho Force".to_string(), "$source assaulted $target's mind!".to_string(),
        "Colossal Psychokinesis Damage to a single foe".to_string(),
        None, Some(30),
        true, Some(200), Some(50),
        None, None,
        false, None,
        false, 0.0,
        1.0,
    ));

    let e_source: PartialEntity = ecs::World::create_entity()
        .add_component(Component::HealthComponent(HealthComponent::new(
            1000,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            "Cheem".to_string(), ecs::Faction::Ally, ecs::AI::Random, Some(500), vec![test_move.clone(), test_move2.clone()], 100, 80, 50, 70, 80, 0.0, 40, 10,
        )));
    world.build_entity(e_source);

    let e_target: PartialEntity = ecs::World::create_entity()
        .add_component(Component::HealthComponent(HealthComponent::new(
            500,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            "Shadow Cheem".to_string(), ecs::Faction::Enemy, ecs::AI::Random, Some(50), vec![test_move.clone(), test_move2.clone()], 90, 50, 100, 50, 80, 0.0, 40, 0,
        )));
    world.build_entity(e_target);

    //let result = battle::battle_loop(&mut world, vec![0], vec![1]);
    //match result {
        //battle::BattleResult::Win => println!("You win!"),
        //_ => println!("You lose!"),
    //};

    for i in 1..1000 {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        rendering::draw_container(ctx, na::Point2::new(100.0, 100.0), mint::Vector2{x:300.0, y:150.0});
        rendering::draw_fighter_stats(&mut world, ctx);

        graphics::present(ctx);
        ggez::timer::yield_now();
    }

    Ok(())
}
