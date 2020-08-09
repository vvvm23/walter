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
    //let e1 = state.new_entity();
    //let fc = battle::FighterComponent::new(e1.clone())
        //.set_level(100)
        //.set_max_hp(999).set_max_sp(999)
        //.set_attack(99).set_defence(99)
        //.set_agility(99).set_luck(99)
        //.add_move(
            //&Rc::new(battle::Move::new("Megidolaon", "Colossal damage to all enemies.")
            //.set_hp_cost(0)
            //.set_sp_cost(48)
            //.set_power(100))
        //);

    //e1.add_fighter(&mut state, fc)
        //.add_position(&mut state, 0.0, 0.0)
        //.add_sprite(&mut state, &mut window, "./resources/walter.png")
        //.add_null(&mut state);

    let e2 = state.new_entity()
        .add_position(&mut state, 0.0, 0.0)
        .add_sprite(&mut state, &mut window, "./resources/cheems.png");

    {
        let e2_sprite = state.sprite_components.get_mut(e2).unwrap();
        e2_sprite.set_scale(0.2);
        e2_sprite.scene_add(&mut window);
    }

    let mut cheems = vec![e2];

    let mut walters: Vec<(ecs::Entity, f32, f32)> = vec![];
    for _ in 0..128 {
        let e = state.new_entity()
            .add_position(&mut state, rng.gen::<f32>() * 3.0 - 1.5, rng.gen::<f32>() * 2.0 - 1.0)
            .add_sprite(&mut state, &mut window, "./resources/walter.png")
            .add_null(&mut state);
        let e_sprite = state.sprite_components.get_mut(e).unwrap();
        e_sprite.set_scale(0.2);
        e_sprite.scene_add(&mut window);

        let interval: f32 = rng.gen::<f32>() + 0.1;
        walters.push((e, 0.0, interval));
    }

    let CHEEMS_INTERVAL: f32 = 0.5;
    let MAX_CHEEMS: usize = 100;
    let mut cheems_timer: f32 = 0.0;

    let mut now = SystemTime::now();
    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        let elapsed = match now.elapsed() {
            Ok(elapsed) => elapsed.as_secs_f32(),
            Err(_) => 1000.0 / 60.0
        };
        now = SystemTime::now();
        
        cheems_timer += elapsed;

        if cheems_timer > CHEEMS_INTERVAL {
            for c in cheems.iter() {
                let c_pos = state.position_components.get_mut(*c).unwrap();
                let c_sprite = state.sprite_components.get_mut(*c).unwrap();

                c_pos.x = rng.gen::<f32>() * 2.0 - 1.0;
                c_pos.y = rng.gen::<f32>() * 2.0 - 1.0;
                c_sprite.update_pos([c_pos.x, c_pos.y, 0.0]);
            }
    
            if cheems.len() < MAX_CHEEMS {
                let new_c = state.new_entity()
                    .add_position(&mut state, 0.0, 0.0)
                    .add_sprite(&mut state, &mut window, "./resources/cheems.png");

                let new_c_sprite = state.sprite_components.get_mut(new_c).unwrap();
                new_c_sprite.set_scale(0.2);
                new_c_sprite.scene_add(&mut window);
                cheems.push(new_c);
            }

            cheems_timer -= CHEEMS_INTERVAL;
        }

        for w in walters.iter_mut() {
            w.1 += elapsed;
            if w.1 > w.2 {
                w.1 -= w.2;
                let e_pos = state.position_components.get_mut(w.0).unwrap();
                let e_sprite = state.sprite_components.get_mut(w.0).unwrap();
                e_pos.x = rng.gen::<f32>() * 3.0 - 1.5;
                e_pos.y = rng.gen::<f32>() * 2.0 - 1.0;
                e_sprite.update_pos([e_pos.x, e_pos.y, 0.0]);
            }
        }

        window.render(&camera);
    }

}
