mod ecs;

use std;

use ecs::Component;
use ecs::PartialEntity;

use ecs::{HealthComponent, VelocityComponent, PositionComponent, RenderablePrimitiveComponent, RenderableSpriteComponent};

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
    //graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

    primitive_rendering(world, ctx);
    sprite_rendering(world, ctx);

    graphics::present(ctx)?;
    Ok(())
}

fn primitive_rendering(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    for (id, c) in world.renderable_primitive_components.iter() {
        let point: na::Point2<f32>;
        if (!world.position_components.contains_key(id)) {
            point = na::Point2::new(0.0, 0.0);
        } else {
            point = world.position_components.get(id).unwrap().to_point();
        }

        let mesh = c.build_mesh(ctx);
        graphics::draw(ctx, &mesh, (point,))?;
    }
    Ok(())
}

fn sprite_rendering(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    for (id, c) in world.renderable_sprite_components.iter() {
        let point: na::Point2<f32>;
        if (!world.position_components.contains_key(id)) {
            point = na::Point2::new(0.0, 0.0);
        } else {
            point = world.position_components.get(id).unwrap().to_point();
        }
        graphics::draw(ctx, &c.texture, graphics::DrawParam::default()
            .dest(point)
            .scale(c.scale)
            )?;
    }
    Ok(())
}

fn main() -> GameResult {
    let mut world: ecs::World = ecs::World::new();

    let wm: ggez::conf::WindowMode = ggez::conf::WindowMode {
        width: 1920.0,
        height: 1080.0,
        ..Default::default()
    };

    let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23").add_resource_path(std::path::PathBuf::from("")).window_mode(wm);
    //let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23").window_mode(wm);
    
    let (ctx, event_loop) = &mut cb.build()?;

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderablePrimitiveComponent(RenderablePrimitiveComponent::new(
            ecs::Shape::Circle{r: 100.0}, graphics::DrawMode::fill(), graphics::WHITE,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            1920.0, 0.0,
        )))
        .add_component(Component::VelocityComponent(VelocityComponent::new(
            -2.0, 2.0,
        )));
    world.build_entity(e);

    let e: PartialEntity = ecs::World::create_entity()
        .add_component(Component::RenderableSpriteComponent(RenderableSpriteComponent::new(
            ctx, "/cheem.png", 0.5, 0.5,
        )))
        .add_component(Component::PositionComponent(PositionComponent::new(
            0.0, 0.0,
        )))
        .add_component(Component::VelocityComponent(VelocityComponent::new(
            2.0, 2.0,
        )));
    world.build_entity(e);

    // tmp main loop
    for i in 1..1000 {
        println!("Iteration {}", i);
        velocity_system(&mut world);
        rendering_system(&mut world, ctx);
        println!("");
    }
    Ok(())
}
