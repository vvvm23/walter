mod ecs;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{RenderableComponent, HealthComponent, VelocityComponent, PositionComponent};


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

fn rendering_system(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

    for (id, c) in world.renderable_components.iter() {
        let point: na::Point2<f32>;
        if (!world.position_components.contains_key(id)) {
            point = na::Point2::new(0.0, 0.0);
        } else {
            point = world.position_components.get(id).unwrap().to_point();
        }

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            100.0,
            1.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (point,))?;

    }
    graphics::present(ctx)?;
    Ok(())
}

fn main() -> GameResult {
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

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderableComponent(RenderableComponent::new(
            "not epic".to_string()
        )))
        .add_component(Component::HealthComponent(HealthComponent::new(
            100,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            10.0, 10.0,
        )))
        .add_component(Component::VelocityComponent(VelocityComponent::new(
            2.0, 2.0,
        )));
    world.build_entity(e);

    let wm: ggez::conf::WindowMode = ggez::conf::WindowMode {
        width: 1920.0,
        height: 1080.0,
        ..Default::default()
    };

    let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23").window_mode(wm);
    let (ctx, event_loop) = &mut cb.build()?;

    // tmp main loop
    for i in 1..1000 {
        println!("Iteration {}", i);
        velocity_system(&mut world);
        rendering_system(&mut world, ctx);
        println!("");
    }
    Ok(())
}
