mod ecs;
mod rendering;

use std;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{HealthComponent, VelocityComponent, PositionComponent, RenderablePrimitiveComponent, RenderableSpriteComponent, AudioComponent};

use ggez::nalgebra as na;
use ggez::graphics;
use ggez::{Context, GameResult};

fn velocity_system(world: &mut ecs::World) {
    for (id, c) in world.velocity_components.iter() {
        if (world.position_components.contains_key(id)) {
            let pc: &mut ecs::PositionComponent = world.position_components.get_mut(id).unwrap();
            pc.translate_component(c);
            println!("{}: {}, {}", id, pc.x, pc.y);
        }
    }
}

fn main() -> GameResult {
    let mut world: ecs::World = ecs::World::new();

    let wm: ggez::conf::WindowMode = ggez::conf::WindowMode {
        width: 1920.0,
        height: 800.0,
        ..Default::default()
    };

    let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23").add_resource_path(std::path::PathBuf::from("")).window_mode(wm);
    //let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23").window_mode(wm);
    
    let (ctx, event_loop) = &mut cb.build()?;

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::AudioComponent(AudioComponent::new(
            ctx, "/music.flac", true
        )));
    world.build_entity(e);

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderablePrimitiveComponent(RenderablePrimitiveComponent::new(
            ecs::Shape::Circle{r: 100.0}, graphics::DrawMode::fill(), graphics::Color{r:1.0, g:0.0, b:0.0, a:1.0},
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            1920.0, 0.0,
        )))
        .add_component(Component::VelocityComponent(VelocityComponent::new(
            -2.0, 2.0,
        )));
    world.build_entity(e);

    for i in 1..10 {
        let e: PartialEntity = ecs::World::create_entity()
            .add_component(Component::RenderableSpriteComponent(RenderableSpriteComponent::new(
                ctx, "/cheem.png", 0.5, 0.5,
            )))
            .add_component(Component::PositionComponent(PositionComponent::new(
                960.0, 0.0,
            )))
            .add_component(Component::VelocityComponent(VelocityComponent::new(
                -1.0 + (i as f32)*0.2, 1.0,
            )));
        world.build_entity(e);
    }

    world.audio_components.get_mut(&0).unwrap().set_volume(0.0);
    world.audio_components.get_mut(&0).unwrap().play();

    // tmp main loop
    for i in 1..1000 {
        println!("Iteration {}", i);
        velocity_system(&mut world);
        rendering::rendering_system(&mut world, ctx);
        println!("");
    }
    Ok(())
}
