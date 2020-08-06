mod ecs;
mod battle;
mod rendering;
use log;
use log::{info, error, debug};
use env_logger;
use std::rc::Rc;

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

}
