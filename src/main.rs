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

    let ws: ggez::conf::WindowSetup = ggez::conf::WindowSetup {
        title: "walter".to_string(),
        ..Default::default()
    };

    let cb = ggez::ContextBuilder::new("walter 0.0", "vvvm23")
        .add_resource_path(std::path::PathBuf::from("./resources"))
        .window_mode(wm)
        .window_setup(ws);
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
    texture_atlas.load(ctx, "/walter.png");
    texture_atlas.load(ctx, "/walter_profile.png");
    let texture_atlas = Arc::new(texture_atlas);

    let mut sound_atlas = system::audio::AudioAtlas::new();
    sound_atlas.load(ctx, "/music.flac");
    //sound_atlas.load(ctx, "/music.wav");
    let sound_atlas = Arc::new(sound_atlas);
    //let bgm = sound_atlas.get("/music.wav");
    //bgm.write().unwrap().play()?;

    // TODO: Load audio in similar audio atlas

    let move_1 = component::battle::Move::new(
        "Cross Slash", "Slash the target twice", "$source slashed at $target!",
        0, 10, 80, true, 1.0, MoveTarget::Single(SingleTarget::Enemy),
    );
    let move_2 = component::battle::Move::new(
        "God's Hand", "Colossal Physical Damage", "$source crushed the $target!",
        0, 30, 120, true, 0.9, MoveTarget::Single(SingleTarget::Enemy),
    );
    let move_3 = component::battle::Move::new(
        "Flying Press", "Channel the reckless spirit of Hawk.", "$source slams down hard from a great height!", 50, 0, 120, true, 0.95, MoveTarget::Single(SingleTarget::Enemy),
    );

    let entity_back = ecs::PartialEntity::new()
        .add_component(component::rendering::BackgroundComponent::new(texture_atlas.get("/night_desert.png")));
    world.write().unwrap().build_entity(entity_back);
    let bi = system::battle::BattleInstance::new("cheems");
    world.write().unwrap().battle_instance = Some(Arc::new(RwLock::new(
        bi
    )));

    let entity_log = ecs::PartialEntity::new()
        .add_component(component::physics::PositionComponent::new(10.0, 740.0))
        .add_component(component::rendering::TextBoxComponent::new(
            8, 1180.0, 150.0,
            [0.2, 0.2, 0.2, 1.0].into(), ggez::graphics::WHITE,
        ));
    let entity_log = world.write().unwrap().build_entity(entity_log);
    let logger = Arc::clone(world.read().unwrap().text_box_components.get(&entity_log).unwrap());
    logger.write().unwrap().add_line("Foobar");
    logger.write().unwrap().add_line("Doobar");

    let mut cheems_collection: Vec<Arc<ecs::Entity>> = Vec::new();

    let cheems = ecs::PartialEntity::new()
    .add_component(component::battle::FighterComponent::new("Cheems #1", 100, Faction::Ally, component::battle::AI::Random, 500, 200, vec![Arc::clone(&move_1), Arc::clone(&move_2), Arc::clone(&move_3)], 100, 100, 100, 100, Some(texture_atlas.get("/cheems_profile.png"))))
    .add_component(component::physics::PositionComponent::new(150.0, 400.0))
    .add_component(component::rendering::SpriteRenderableComponent::new(texture_atlas.get("/cheems.png"), -0.3, 0.3))
    .add_component(component::physics::IdleBobComponent::new(15.0, 1.5));

    let new_cheems = world.write().unwrap().build_entity(cheems);
    cheems_collection.push(Arc::clone(&new_cheems));

    let cheems = ecs::PartialEntity::new()
    .add_component(component::battle::FighterComponent::new("Cheems #2", 100, Faction::Ally, component::battle::AI::Random, 500, 200, vec![Arc::clone(&move_1), Arc::clone(&move_2), Arc::clone(&move_3)], 100, 100, 100, 100, Some(texture_atlas.get("/cheems_profile.png"))))
    .add_component(component::physics::PositionComponent::new(300.0, 500.0))
    .add_component(component::rendering::SpriteRenderableComponent::new(texture_atlas.get("/cheems.png"), -0.3, 0.3))
    .add_component(component::physics::IdleBobComponent::new(10.0, 0.7));

    let new_cheems = world.write().unwrap().build_entity(cheems);
    cheems_collection.push(Arc::clone(&new_cheems));

    let cheems = ecs::PartialEntity::new()
    .add_component(component::battle::FighterComponent::new("Walter #1", 100, Faction::Enemy, component::battle::AI::Random, 500, 200, vec![Arc::clone(&move_1), Arc::clone(&move_2), Arc::clone(&move_3)], 100, 100, 100, 100, Some(texture_atlas.get("/walter_profile.png"))))
    .add_component(component::physics::PositionComponent::new(600.0, 500.0))
    .add_component(component::rendering::SpriteRenderableComponent::new(texture_atlas.get("/walter.png"), 0.3, 0.3))
    .add_component(component::physics::IdleBobComponent::new(5.0, 2.0));

    let new_cheems = world.write().unwrap().build_entity(cheems);
    cheems_collection.push(Arc::clone(&new_cheems));


    let cheems = ecs::PartialEntity::new()
    .add_component(component::battle::FighterComponent::new("Walter #2", 100, Faction::Enemy, component::battle::AI::Random, 500, 200, vec![Arc::clone(&move_1), Arc::clone(&move_2), Arc::clone(&move_3)], 100, 100, 100, 100, Some(texture_atlas.get("/walter_profile.png")))) 
    .add_component(component::physics::PositionComponent::new(750.0, 400.0))
    .add_component(component::rendering::SpriteRenderableComponent::new(texture_atlas.get("/walter.png"), 0.3, 0.3))
    .add_component(component::physics::IdleBobComponent::new(10.0, 1.0));

    let new_cheems = world.write().unwrap().build_entity(cheems);
    cheems_collection.push(Arc::clone(&new_cheems));

    let cheems_1 = Arc::clone(&cheems_collection[0]);
    let walter_1 = Arc::clone(&cheems_collection[2]);

    world.read().unwrap().battle_instance.as_ref().unwrap().write().unwrap().add_entities(&mut cheems_collection);

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
                    WindowEvent::MouseInput { state, .. } => {
                        if let ggez::event::winit_event::ElementState::Pressed = state {
                            println!("{:?}", ggez::input::mouse::position(&ctx))
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        if make_child {
            make_child = false;
            let world = Arc::clone(&world);
            let cheems_1 = Arc::clone(&cheems_1);
            let walter_1 = Arc::clone(&walter_1);

            thread::spawn(move || {
                for _ in 1..100 {
                    system::battle::execute_effect(Arc::clone(&world), Arc::clone(&walter_1), Arc::clone(&cheems_1), system::battle::MoveResult {
                        hit: true, hp_cost: 0, sp_cost: 0, hp: 50, damaging: true,
                    });
                    std::thread::sleep_ms(2000);
                }
            });
        }

        //if make_child {
            //make_child = false;
            //let world_child = Arc::clone(&world);
            //let atlas_child = Arc::clone(&texture_atlas);
            //let move_1_child = Arc::clone(&move_1);
            //let move_2_child = Arc::clone(&move_2);
            //let move_3_child = Arc::clone(&move_3);
            //let logger_child = Arc::clone(&logger);
            //thread::spawn(move || {
                //println!("Spawning Child thread");
                //for _ in 1..10000 {
                    //let entity_child = ecs::PartialEntity::new()
                        //.add_component(component::battle::FighterComponent::new("Cheems", 100, Faction::Ally, component::battle::AI::Random, 500, 200, vec![Arc::clone(&move_1_child), Arc::clone(&move_2_child), Arc::clone(&move_3_child)], 100, 100, 100, 100, Some(atlas_child.get("/cheems_profile.png"))))
                        //.add_component(component::physics::PositionComponent::new(-200.0, -200.0))
                        //.add_component(component::physics::VelocityComponent::new(100.0, 100.0))
                        //.add_component(component::rendering::SpriteRenderableComponent::new(atlas_child.get("/cheems.png"), 0.3, 0.3));

                    //let new_cheem = world_child.write().unwrap().build_entity(entity_child);
                    //world_child.read().unwrap().battle_instance.as_ref().unwrap().write().unwrap().add_entities(&mut vec![Arc::clone(&new_cheem)]);

                    ////let (random_move, random_target) = system::battle::ai_handover(Arc::clone(&new_cheem), Arc::clone(&world_child));
                    ////let random_target = match random_target {
                        ////system::battle::AOEOrSingle::Single(e) => format!("{}", e.id),
                        ////system::battle::AOEOrSingle::AOE(s) => "many cheems".to_string(),
                    ////};
                    ////logger_child.write().unwrap().add_line(&random_move.use_message
                                                           ////.replace("$source", &format!("Cheems {}", new_cheem.id))
                                                           ////.replace("$target", &format!("Cheems {}", random_target)));
                    //world_child.read().unwrap().battle_instance.as_ref().unwrap().write().unwrap().state = system::battle::BattleState::Available;
                    //std::thread::sleep_ms(1000);
                //}
            //});
        //}

        // Actual game loop
        let d_time = ggez::timer::delta(ctx);
        let d_time: f64 = ggez::timer::duration_to_f64(d_time);

        // Update
        system::physics::velocity_system(Arc::clone(&world), &d_time);
        system::physics::idle_bob_system(Arc::clone(&world), &d_time);

        // In reality, this guard will be much more sophisticated
        //if world.read().unwrap().battle_instance.as_ref().unwrap().read().unwrap().entities.len() > 0 {
            //system::battle::battle_loop(Arc::clone(&world));
        //}

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
    let (ctx, events_loop) = &mut init_window(1200.0, 900.0).build()?;

    game_loop(ctx, events_loop)?;

    Ok(())
}
