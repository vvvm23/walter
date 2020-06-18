use ggez;

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

fn game_loop(ctx: &mut ggez::Context, e_loop: &mut ggez::event::EventsLoop) -> ggez::GameResult {
    use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent};

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
    }

    // Actual game loop
    // Update

    // Draw

    Ok(())
}

fn main() -> ggez::GameResult {
    let (ctx, events_loop) = &mut init_window(1600.0, 1200.0).build()?;

    game_loop(ctx, events_loop)?;

    Ok(())
}
