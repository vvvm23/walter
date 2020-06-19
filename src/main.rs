mod ecs;
mod component;
mod system;

use std::sync::{Arc, RwLock};
use std::thread;
use ggez;

/// Initialises window with specified width and height
fn init_window(width: f32, height: f32) -> ggez::ContextBuilder {
    let wm: ggez::conf::WindowMode = ggez::conf::WindowMode {
        width: width,
        height: height,
        ..Default::default()
    };

    let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23")
        .add_resource_path(std::path::PathBuf::from("./resources"))
        .window_mode(wm);
    cb
}

/// Contains main game loop
fn game_loop(ctx: &mut ggez::Context, e_loop: &mut ggez::event::EventsLoop) -> ggez::GameResult {
    use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent};

    let world = Arc::new(RwLock::new(ecs::World::new()));

    let mut atlas = system::rendering::TextureAtlas::new();
    atlas.load(ctx, "/cheems.png");
    atlas.load(ctx, "/night_desert.png");
    let atlas = Arc::new(atlas);

    let entity_back = ecs::PartialEntity::new()
        .add_component(component::rendering::BackgroundComponent::new(atlas.get("/night_desert.png")));
    world.write().unwrap().build_entity(entity_back);

    let mut make_child: bool = true;

    while ctx.continuing {
        ctx.timer_context.tick(); // Tell internal timer a frame has happened

        // Handle input events
        e_loop.poll_events(|e| {
            ctx.process_event(&e);
            match e {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ggez::event::quit(ctx),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
                        ggez::event::KeyCode::Escape => ggez::event::quit(ctx),
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });
        println!("{}", make_child);
        if make_child {
            make_child = false;
            let world_child = Arc::clone(&world);
            let atlas_child = Arc::clone(&atlas);
            thread::spawn(move || {
                println!("Spawning Child thread");
                for _ in 1..10000000 {
                    println!("Child Thread creates new entity");
                    let entity_child = ecs::PartialEntity::new()
                        .add_component(component::physics::PositionComponent::new(-200.0, -200.0))
                        .add_component(component::physics::VelocityComponent::new(1.0, 1.0))
                        //.add_component(component::rendering::PrimitiveRenderableComponent::new(component::rendering::Shape::Circle{r:10.0}, ggez::graphics::DrawMode::fill(), ggez::graphics::WHITE));
                        .add_component(component::rendering::SpriteRenderableComponent::new(atlas_child.get("/cheems.png"), 0.5, 0.5));
                    world_child.write().unwrap().build_entity(entity_child);
                    std::thread::sleep_ms(1500);
                }
            });
        }

        // Actual game loop
        let d_time = ggez::timer::delta(ctx);
        let d_time: f64 = ggez::timer::duration_to_f64(d_time);

        // Update
        // TODO: Check if any new threads need to be spawned
        println!("Parent thread");
        system::physics::velocity_system(Arc::clone(&world));
        
        // Draw
        ggez::graphics::present(ctx)?;
        ggez::graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        system::rendering::background_rendering_system(Arc::clone(&world), ctx)?;
        system::rendering::primitive_rendering_system(Arc::clone(&world), ctx)?;
        system::rendering::sprite_rendering_system(Arc::clone(&world), ctx)?;
        ggez::timer::yield_now();
        println!("{}", ggez::timer::fps(ctx));
    }

    Ok(())
}

fn main() -> ggez::GameResult {
    let (ctx, events_loop) = &mut init_window(1600.0, 1200.0).build()?;

    game_loop(ctx, events_loop)?;

    Ok(())
}
