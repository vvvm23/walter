mod ecs;
mod battle;
mod rendering;

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

    window.scene.background = three::Background::Color(0x630012);

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
        let e1_sprite = state.sprite_components.get_mut(e1).unwrap();
        e1_sprite.set_scale(0.2);
        e1_sprite.scene_add(&mut window);
    }

    let WALT_INTERVAL: f32 = 0.2;
    let mut walter_timer: f32 = 0.0;

    let mut now = SystemTime::now();
    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        let elapsed = match now.elapsed() {
            Ok(elapsed) => elapsed.as_secs_f32(),
            Err(_) => 1000.0 / 60.0
        };
        now = SystemTime::now();
        
        walter_timer += elapsed;
        if walter_timer > WALT_INTERVAL {
            let e1_pos = state.position_components.get_mut(e1).unwrap();
            let e1_sprite = state.sprite_components.get_mut(e1).unwrap();
            e1_pos.x = rng.gen::<f32>() * 2.0 - 1.0;
            e1_pos.y = rng.gen::<f32>() * 2.0 - 1.0;
            e1_sprite.update_pos([e1_pos.x, e1_pos.y, 0.0]);
            walter_timer -= WALT_INTERVAL;
        }


        window.render(&camera);
    }

}
