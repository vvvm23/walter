use crate::ecs as ecs;

use ecs::Component;
use ecs::{RenderableSpriteComponent, RenderablePrimitiveComponent};

use ggez::nalgebra as na;
use ggez::graphics;
use ggez::{Context, GameResult};

pub fn init_window(width: f32, height: f32) -> GameResult<Context> {
    let wm: ggez::conf::WindowMode = ggez::conf::WindowMode {
        width: width,
        height: height,
        ..Default::default()
    };

    let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23").add_resource_path(std::path::PathBuf::from("")).window_mode(wm);
    
    let (ctx, event_loop) = cb.build()?;
    Ok(ctx)
}

pub fn rendering_system(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

    primitive_rendering(world, ctx);
    sprite_rendering(world, ctx);

    graphics::present(ctx)?;
    Ok(())
}

pub fn primitive_rendering(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
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

pub fn sprite_rendering(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
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

