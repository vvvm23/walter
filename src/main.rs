mod ecs;
mod battle;
mod rendering;
mod audio;

use log;
use log::{info, error, debug};
use env_logger;

use std::rc::Rc;
use std::time::SystemTime;

use rand;
use rand::prelude::*;

fn main() {
    env_logger::init();
    let mut rng = rand::thread_rng();

    info!("This is an informative message.");
    
    let mut audio_system = audio::AudioSystem::new();
    audio_system.play_music("resources/music.flac", 0.5);

    let mut window = three::window::Window::builder("walter 0.0")
        .dimensions(1200.0, 900.0)
        .vsync(true)
        .build();

    //window.scene.background = three::Background::Color(0x630012);
    window.scene.background = three::Background::Texture(window.factory.load_texture("./resources/night_desert.png"));

    let cam_centre = [0.0, 0.0];
    let cam_yex = 1.0;
    let cam_zrange = -5.0 .. 5.0;
    let camera = window.factory.orthographic_camera(cam_centre, cam_yex, cam_zrange);

    let mut state = ecs::State::new();
    let e1 = state.new_entity();
    let fc = battle::FighterComponent::new(e1.clone())
        .set_level(100)
        .set_max_hp(999).set_max_sp(999)
        .set_attack(99).set_defence(99)
        .set_agility(99).set_luck(99)
        .add_move(
            &Rc::new(battle::Move::new("Megidolaon", "Colossal damage to all enemies.")
            .set_hp_cost(0)
            .set_sp_cost(48)
            .set_power(100))
        );

    e1.add_fighter(&mut state, fc)
        .add_position(&mut state, 0.0, 0.0)
        .add_sprite(&mut state, &mut window, "./resources/walter.png")
        .add_null(&mut state);

    {
        let mut e1_sprite = state.sprite_components.get_mut(e1).unwrap();
        e1_sprite.set_scale(0.2);
        e1_sprite.scene_add(&mut window);
    }

    let e2 = state.new_entity()
        .add_position(&mut state, -0.5, -0.1)
        .add_sprite(&mut state, &mut window, "resources/cheems.png")
        .add_null(&mut state);

    {
        let mut e2_sprite = state.sprite_components.get_mut(e2).unwrap();
        e2_sprite.set_scale(0.2);
        e2_sprite.scene_add(&mut window);
        e2_sprite.update_pos([0.5, -0.1, 0.2]);
    }
    let mut timer = 0.0;
    let mut audio_int: f32 = rng.gen::<f32>() * 1.0 + 0.1;

    let mut now = SystemTime::now();
    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        let elapsed = match now.elapsed() {
            Ok(elapsed) => elapsed.as_secs_f32(),
            Err(_) => 1000.0 / 60.0
        };
        now = SystemTime::now();

        timer += elapsed;
        if timer > audio_int {
            if rand::random() {
                audio_system.play_sound("resources/Gun1.ogg", 0.5);
            } else {
                audio_system.play_sound("resources/Gun2.ogg", 0.5);
            }

            timer = 0.0;
            audio_int = rng.gen::<f32>() * 1.0 + 0.1;
        }

        window.render(&camera);
        audio_system.update();
    }

}
