mod ecs;
mod component;
mod system;

use std::sync::{Arc, RwLock};
use std::thread;
use ggez;
use ggez::audio::SoundSource;

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
    use component::battle::{Faction, MoveTarget, AOETarget, SingleTarget};

    let world = Arc::new(RwLock::new(ecs::World::new()));

    let mut texture_atlas = system::rendering::TextureAtlas::new();
    texture_atlas.load(ctx, "/cheems_profile.png");
    texture_atlas.load(ctx, "/cheems.png");
    texture_atlas.load(ctx, "/night_desert.png");
    let texture_atlas = Arc::new(texture_atlas);

    let mut sound_atlas = system::audio::AudioAtlas::new();
    sound_atlas.load(ctx, "/music.flac");
    let sound_atlas = Arc::new(sound_atlas);
    //let bgm = sound_atlas.get("/music.flac");
    //bgm.write().unwrap().play()?;

    // TODO: Load audio in similar audio atlas

    let move_1 = component::battle::Move::new(
        "Cross Slash", "Slash the target twice", "$source slashed at $target!",
        0, 10, 80, 1.0, MoveTarget::Single(SingleTarget::Enemy),
    );
    let move_2 = component::battle::Move::new(
        "God's Hand", "Colossal Physical Damage", "$source crushed the $target!",
        0, 30, 120, 0.9, MoveTarget::Single(SingleTarget::Enemy),
    );
    let move_3 = component::battle::Move::new(
        "Flying Press", "Channel the reckless spirit of Hawk.", "$source slams down hard from a great height!", 50, 0, 120, 0.95, MoveTarget::Single(SingleTarget::Enemy),
    );

    let entity_back = ecs::PartialEntity::new()
        .add_component(component::rendering::BackgroundComponent::new(texture_atlas.get("/night_desert.png")));
    world.write().unwrap().build_entity(entity_back);
    let bi = system::battle::BattleInstance::new("cheems");
    world.write().unwrap().battle_instance = Some(Arc::new(RwLock::new(
        bi
    )));

    let entity_log = ecs::PartialEntity::new()
        .add_component(component::physics::PositionComponent::new(10.0, 990.0))
        .add_component(component::rendering::TextBoxComponent::new(
            10, 1580.0, 200.0,
            [0.2, 0.2, 0.2, 1.0].into(), ggez::graphics::WHITE,
        ));
    let entity_log = world.write().unwrap().build_entity(entity_log);
    let logger = Arc::clone(world.read().unwrap().text_box_components.get(&entity_log).unwrap());
    logger.write().unwrap().add_line("Foobar");
    logger.write().unwrap().add_line("Doobar");

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
        if make_child {
            make_child = false;
            let world_child = Arc::clone(&world);
            let atlas_child = Arc::clone(&texture_atlas);
            let move_1_child = Arc::clone(&move_1);
            let move_2_child = Arc::clone(&move_2);
            let move_3_child = Arc::clone(&move_3);
            let logger_child = Arc::clone(&logger);
            thread::spawn(move || {
                println!("Spawning Child thread");
                for _ in 1..10000 {
                    let entity_child = ecs::PartialEntity::new()
                        .add_component(component::battle::FighterComponent::new("Cheems", 100, Faction::Ally, component::battle::AI::Random, 500, 200, vec![Arc::clone(&move_1_child), Arc::clone(&move_2_child), Arc::clone(&move_3_child)], 100, 100, 100, 100, Some(atlas_child.get("/cheems_profile.png"))))
                        .add_component(component::physics::PositionComponent::new(-200.0, -200.0))
                        .add_component(component::physics::VelocityComponent::new(100.0, 100.0))
                        .add_component(component::rendering::SpriteRenderableComponent::new(atlas_child.get("/cheems.png"), 0.5, 0.5));

                    let new_cheem = world_child.write().unwrap().build_entity(entity_child);
                    world_child.write().unwrap().battle_instance.as_ref().unwrap().write().unwrap().add_entities(&mut vec![Arc::clone(&new_cheem)]);

                    //println!("created new cheems");
                    //println!("cheems {} details:", new_cheem.id);
                    //println!("{:?}", world_child.read().unwrap().fighter_components.get(&new_cheem).unwrap());
                    //println!("cheems");
                    logger_child.write().unwrap().add_line(&format!("New cheems spawned with id {}", new_cheem.id));

                    std::thread::sleep_ms(100);
                }
            });
        }

        // Actual game loop
        let d_time = ggez::timer::delta(ctx);
        let d_time: f64 = ggez::timer::duration_to_f64(d_time);

        // Update
        system::physics::velocity_system(Arc::clone(&world), &d_time);
        
        // Draw
        ggez::graphics::present(ctx)?;
        ggez::graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        system::rendering::background_rendering_system(Arc::clone(&world), ctx)?;
        system::rendering::primitive_rendering_system(Arc::clone(&world), ctx)?;
        system::rendering::sprite_rendering_system(Arc::clone(&world), ctx)?;
        system::rendering::ally_stats_rendering_system(Arc::clone(&world), ctx)?;
        //system::rendering::draw_container(10.0, 990.0, 1580.0, 200.0, [0.2, 0.2, 0.2, 1.0].into(), ctx)?;
        system::rendering::textbox_rendering_system(Arc::clone(&world), ctx)?;
        system::rendering::draw_fps(ctx)?;
        ggez::timer::yield_now();
    }

    Ok(())
}

fn main() -> ggez::GameResult {
    let (ctx, events_loop) = &mut init_window(1600.0, 1200.0).build()?;

    game_loop(ctx, events_loop)?;

    Ok(())
}
