mod ecs;
mod battle;
mod rendering;
use log;
use log::{info, error, debug};
use env_logger;
use std::rc::Rc;
use std::time::SystemTime;

fn main() {
    env_logger::init();
    
    info!("This is an informative message.");
    
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
        .add_null(&mut state);

    println!("{:?}", state.null_components.get(e1));
    println!("{:?}", state.position_components.get(e1));
    println!("{:?}", state.fighter_components.get(e1));
    assert_eq!(true, e1.has_component(&state, ecs::ComponentType::Null));
    e1.remove_component(&mut state, ecs::ComponentType::Null);
    assert_eq!(false, e1.has_component(&state, ecs::ComponentType::Null));

    state.delete_entity(e1);

    println!("{:?}", state.null_components.get(e1));
    println!("{:?}", state.position_components.get(e1));

    let mut window = three::window::Window::builder("walter 0.0")
        .dimensions(1200.0, 900.0)
        .vsync(true)
        .build();

    window.scene.background = three::Background::Color(0x630012);

    let cam_centre = [0.0, 0.0];
    let cam_yex = 1.0;
    let cam_zrange = -5.0 .. 5.0;
    let camera = window.factory.orthographic_camera(cam_centre, cam_yex, cam_zrange);

    let mut now = SystemTime::now();
    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        let elapsed = match now.elapsed() {
            Ok(elapsed) => elapsed.as_secs_f32(),
            Err(_) => 1000.0 / 60.0
        };
        now = SystemTime::now();
        
        println!("{:?}", 1.0 / elapsed);

        window.render(&camera);
    }

}
