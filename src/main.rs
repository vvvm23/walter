mod ecs;
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

    // TODO: Initialise world here
    let mut world: ecs::World = ecs::World {};
    println!("{:?}", world);

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
        // Actual game loop
        let d_time = ggez::timer::delta(ctx);
        let d_time: f64 = ggez::timer::duration_to_f64(d_time);

        // Update

        // TODO: Check if any new threads need to be spawned

        // Draw
        ggez::graphics::present(ctx)?;
        ggez::timer::yield_now();
    }

    Ok(())
}

fn main() -> ggez::GameResult {
    let (ctx, events_loop) = &mut init_window(1600.0, 1200.0).build()?;

    game_loop(ctx, events_loop)?;

    Ok(())
}
