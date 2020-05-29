use crate::ecs as ecs;

use ecs::Component;
use ecs::{RenderableSpriteComponent, RenderablePrimitiveComponent};

use ggez::nalgebra as na;
use ggez::graphics;
use ggez::{Context, GameResult};

// Initialises the window given the width and height
// TODO: pass more window parameters, eg. screen mode <28-05-20, vvvm23> //
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

// System to handle all renderable components. calls other rendering subsystems
pub fn rendering_system(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

    primitive_rendering(world, ctx);
    sprite_rendering(world, ctx);

    graphics::present(ctx)?;
    Ok(())
}

// System to render all primitive components. Simply iterates through these components and draws.
pub fn primitive_rendering(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    for (id, c) in world.renderable_primitive_components.iter() {
        let point: na::Point2<f32>;
        if (!world.position_components.contains_key(id)) {
            point = na::Point2::new(0.0, 0.0);
        } else {
            point = world.position_components.get(id).unwrap().to_point();
        }

        let mesh = c.build_mesh(ctx);
        let mut draw_param = graphics::DrawParam::default()
            .dest(point);

        if world.rotation_components.contains_key(id) {
            let rc: &ecs::RotationComponent = world.rotation_components.get(id).unwrap();
            draw_param = draw_param.rotation(rc.rot);
        }

        graphics::draw(ctx, &mesh, draw_param)?;
    }
    Ok(())
}

// System to render all sprite components. Simply iterates through these components and draws.
// TODO: Sprite batching <28-05-20, vvvm23> //
pub fn sprite_rendering(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    for (id, c) in world.renderable_sprite_components.iter() {
        let point: na::Point2<f32>;
        if (!world.position_components.contains_key(id)) {
            point = na::Point2::new(0.0, 0.0);
        } else {
            point = world.position_components.get(id).unwrap().to_point();
        }

        let mut draw_param = graphics::DrawParam::default()
            .dest(point)
            .scale(c.scale);

        if world.rotation_components.contains_key(id) {
            let rc: &ecs::RotationComponent = world.rotation_components.get(id).unwrap();
            draw_param = draw_param.rotation(rc.rot);
        }
    
        graphics::draw(ctx, &c.texture, draw_param)?;
    }
    Ok(())
}

