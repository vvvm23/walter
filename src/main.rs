mod ecs;
mod rendering;
mod physics;
mod battle;

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
    let ctx: &mut Context = &mut rendering::init_window(1920.0, 1080.0).unwrap();

    // create a test move:
    let test_move: ecs::Move = ecs::Move::new(
        "Megidolaon".to_string(), "{source} let loose terrifying energy!".to_string(),
        "Extreme Almighty damage to all foes.".to_string(),
        None, Some(50),
        true, Some(120), None,
        None, None,
        true, Some(ecs::AreaTarget::Enemy),
        false, 0.0,
        1.0,
    );

    // Create global audio entity for some music :)
    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::AudioComponent(AudioComponent::new(
            ctx, "/music.flac", true
        )));
    world.build_entity(e);

    // Create a circle and add some velocity
    //let e: PartialEntity = ecs::World::create_entity()
        //.add_component(Component::RenderablePrimitiveComponent(RenderablePrimitiveComponent::new(
            //ecs::Shape::Circle{r: 100.0}, graphics::DrawMode::fill(), graphics::Color{r:1.0, g:0.0, b:0.0, a:1.0},
        //)))
        //.add_component(Component::PositionComponent(PositionComponent::new(
            //1920.0, 0.0,
        //)))
        //.add_component(Component::VelocityComponent(VelocityComponent::new(
            //-2.0, 2.0,
        //)))
        //.add_component(Component::RotationComponent(RotationComponent::new(
            //0.0, 
        //)))
        //.add_component(Component::RotationalVelocityComponent(RotationalVelocityComponent::new(
            //0.1
        //)));
    //world.build_entity(e);

    //// Create 10 cheems and add velocity
    //for i in 1..10 {
        //let e: PartialEntity = ecs::World::create_entity()
            //.add_component(Component::RenderableSpriteComponent(RenderableSpriteComponent::new(
                //ctx, "/cheem.png", 0.5, 0.5,
            //)))
            //.add_component(Component::PositionComponent(PositionComponent::new(
                //960.0, 0.0,
            //)))
            //.add_component(Component::VelocityComponent(VelocityComponent::new(
                //-1.0 + (i as f32)*0.2, 1.0,
            //)))
            //.add_component(Component::RotationComponent(RotationComponent::new(
                //-1.0 + (i as f32)*0.2,
            //)))
            //.add_component(Component::RotationalVelocityComponent(RotationalVelocityComponent::new(
                //0.1
            //)));

        //world.build_entity(e);
    //}

    //// Set volume and play audio
    //world.audio_components.get_mut(&0).unwrap().set_volume(0.0);
    //world.audio_components.get_mut(&0).unwrap().play();

    // tmp game loop
    for i in 1..1000 {
        println!("Iteration {}", i);
        
        let mut eid: u16 = 0;
        if i % 100 == 0  {
            for (id, c) in world.renderable_sprite_components.iter() {
                eid = *id;
                break;
            }
            world.remove_entity(&eid);
        }

        physics::velocity_system(&mut world);
        physics::rot_velocity_system(&mut world);
        rendering::rendering_system(&mut world, ctx);
        println!("");
    }
    Ok(())
}
