mod ecs;
mod rendering;
mod physics;
mod battle;

use std::rc::Rc;
use std::{thread, time, path};

use ecs::Component;
use ecs::PartialEntity;

use ecs::{HealthComponent, VelocityComponent, PositionComponent, RenderablePrimitiveComponent, RenderableSpriteComponent, AudioComponent, BackgroundComponent};
use ecs::{RotationComponent, RotationalVelocityComponent};
use ecs::{FighterComponent};
use ecs::{BobComponent};

use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult, timer};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};

use ggez::nalgebra as na;

impl EventHandler for ecs::World {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 60;

        while timer::check_update_time(ctx, TARGET_FPS) {
            let time_elapsed: f32 = 1.0 / (TARGET_FPS as f32);

            // TODO: handle any inputs <07-06-20, vvvm23> //
            // TODO: redirect inputs to correct system <07-06-20, vvv23> //
            // TODO: update current systems <07-06-20, vvvm23> //
            physics::velocity_system(self);
            physics::rot_velocity_system(self);
            physics::bob_system(self, &time_elapsed);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        rendering::rendering_system(self, ctx);
        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        // TODO: key down handler <07-06-20, vvvm23> //
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        // TODO: key up handler <07-06-20, vvvm23> //
    }
}

fn test_entity_create(world: &mut ecs::World, ctx: &mut Context) {
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
            3000,
        )))
        .add_component(Component::BobComponent(BobComponent::new(
            15.0, 1.0,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            "Cheems".to_string(), ecs::Faction::Ally, ecs::AI::Random, Some(9999), vec![test_move.clone(), test_move2.clone()], 100, 80, 50, 70, 80, 0.0, 40, 10, Some("/cheem_profile.png".to_string()), ctx,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            400.0, 800.0,
        )))
        .add_component(Component::RenderableSpriteComponent(RenderableSpriteComponent::new(
            ctx, "/cheems_sit.png", -1.0, 1.0,
        )));
    world.build_entity(e_source);

    let e_target: PartialEntity = ecs::World::create_entity()
        .add_component(Component::HealthComponent(HealthComponent::new(
            8000,
        )))
        .add_component(Component::FighterComponent(FighterComponent::new(
            "Walter".to_string(), ecs::Faction::Enemy, ecs::AI::Random, Some(9999), vec![test_move.clone(), test_move2.clone()], 90, 50, 100, 50, 80, 0.0, 40, 0, Some("/walter_profile.png".to_string()), ctx,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            900.0, 600.0,
        )))
        .add_component(Component::RenderableSpriteComponent(RenderableSpriteComponent::new(
            ctx, "/walter_buff.png", 1.8, 1.8,
        )));
    world.build_entity(e_target);

    let e_background: PartialEntity = ecs::World::create_entity()
        .add_component(Component::BackgroundComponent(BackgroundComponent::new(
            ctx, "/forest.png", 1.0, 1.0,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            0.0, 0.0,
        )));
    world.build_entity(e_background);

}

fn main() -> GameResult {
    // create empty world
    let mut world: ecs::World = ecs::World::new();
    world.current_mode = ecs::GameMode::Battle;

    // initialise window
    let (ctx, events_loop) = &mut rendering::init_window(1600.0, 1200.0).build()?;

    // populate world with entities
    test_entity_create(&mut world, ctx);

    // start the game loop
    event::run(ctx, events_loop, &mut world)

}
