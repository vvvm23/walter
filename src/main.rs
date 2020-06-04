mod ecs;
mod rendering;
mod physics;
mod battle;

use std::rc::Rc;
use std::{thread, time};

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
            "Cheems".to_string(), ecs::Faction::Ally, ecs::AI::Random, Some(500), vec![test_move.clone(), test_move2.clone()], 100, 80, 50, 70, 80, 0.0, 40, 10, Some("/cheem_profile.png".to_string()), ctx,
        )));
    world.build_entity(e_source);

    let e_target: PartialEntity = ecs::World::create_entity()
        .add_component(Component::HealthComponent(HealthComponent::new(
            2000,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            "Walter".to_string(), ecs::Faction::Enemy, ecs::AI::Random, Some(9999), vec![test_move.clone(), test_move2.clone()], 90, 50, 100, 50, 80, 0.0, 40, 0, Some("/walter_profile.png".to_string()), ctx,
        )));
    world.build_entity(e_target);

    let result = battle::battle_loop(&mut world, ctx, vec![0], vec![1]);
    thread::sleep(time::Duration::from_millis(1000));
    match result {
        battle::BattleResult::Win => println!("You win!"),
        _ => println!("You lose!"),
    };

    //let background: graphics::Image = graphics::Image::new(ctx, "/forest.png").unwrap();
    //let mut draw_param = graphics::DrawParam::default();
    //for i in 1..1000 {
        //println!("{}", i);
        //graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        //// TODO: way to change background and store it <04-06-20, vvvm23> //
        //graphics::draw(ctx, &background, draw_param);

        //rendering::draw_friendly_stats(&mut world, ctx, &vec![0,1]);

        //graphics::present(ctx);
    //}

    Ok(())
}
