use crate::ecs as ecs;

use ecs::Component;
use ecs::{RenderableSpriteComponent, RenderablePrimitiveComponent};

use ggez::graphics;
use ggez::{Context, GameResult};

use ggez::nalgebra as na;

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

pub fn draw_container(ctx: &mut Context, position: na::Point2<f32>, scale: mint::Vector2<f32>) -> GameResult {
    let border_size: f32 = 3.0;

    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {x: 0.0, y: 0.0, w: scale.x, h: scale.y},
        graphics::WHITE,
    ).unwrap();
    let mut draw_param = graphics::DrawParam::default()
        .dest(position);
    graphics::draw(ctx, &mesh, draw_param)?;

    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {x: border_size, y: border_size, w: scale.x - 2.0*border_size, h: scale.y - 2.0*border_size},
        [0.3, 0.3, 0.3, 1.0].into(),
    ).unwrap();

    let mut draw_param = graphics::DrawParam::default()
        .dest(position);
    graphics::draw(ctx, &mesh, draw_param)?;
    Ok(())
}

// TODO: either hide enemy, or move to other side of screen <04-06-20, vvvm23> //
// TODO: face sprites for fighters. <04-06-20, vvvm23> //
pub fn draw_fighter_stats(world: &mut ecs::World, ctx: &mut Context) -> GameResult {
    let mut i_fighters: u8 = 0;
    let text_pad: f32 = 10.0;
    for (id, c) in world.fighter_components.iter() {
        // TODO: check if it has health <04-06-20, vvvm23> //
        // TODO: or merge health? <04-06-20, vvvm23> //
        let health: &ecs::HealthComponent = world.health_components.get(&id).unwrap();
        
        draw_container(ctx, na::Point2::new(1200.0 - text_pad, 100.0 - text_pad + (i_fighters*200) as f32), mint::Vector2{x:300.0, y:150.0});

        let name_text: graphics::Text = graphics::Text::new(format!("{}", c.name));
        graphics::draw(ctx, &name_text, (na::Point2::new(1200.0, 100.0+(i_fighters*200) as f32), graphics::WHITE));
        
        let health_text: graphics::Text = graphics::Text::new(format!("{0: <5} {1} / {2}", "HP:", health.hp, health.max_hp));
        graphics::draw(ctx, &health_text, (na::Point2::new(1200.0, 100.0+20.0+(i_fighters*200) as f32), graphics::WHITE));

        let sp_text: graphics::Text = graphics::Text::new(format!("{0: <5} {1} / {2}", "SP:", c.sp, c.max_sp));
        graphics::draw(ctx, &sp_text, (na::Point2::new(1200.0, 100.0+40.0+(i_fighters*200) as f32), graphics::WHITE));

        i_fighters += 1;
    }
    Ok(())
}

