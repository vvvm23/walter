mod ecs;
mod battle;
use log;
use log::{info, error, debug};
use env_logger;
use std::rc::Rc;

fn main() {
    env_logger::init();
    println!("Aloha World!");
    
    info!("This is an informative message.");
    
    let mut state = ecs::State::new();
    let e1 = state.new_entity();
    let fc = battle::FighterComponent::new(e1.clone(), 100, 999, 999, 99, 99, 99, 99)
        .add_move(&Rc::new(battle::Move::new("Megidolaon", "Colossal Almighty damage to all enemies.")
            .set_hp_cost(0)
            .set_sp_cost(48)
            .set_power(100)));
        //.add_move(&Rc::new(battle::Move {
            //name: "Megidolaon".to_string(),
            //description: "Severe Almighty damage to all enemies.".to_string(),
            //hpc: 0, spc: 48, power: 100,
        //}));
    e1.add_fighter(&mut state, fc)
        .add_position(&mut state, 0.0, 0.0)
        .add_null(&mut state);

    println!("{:?}", state.null_components.get(e1));
    println!("{:?}", state.position_components.get(e1));
    println!("{:?}", state.fighter_components.get(e1));
    println!("{:?}", state.entity_allocator.free);
    assert_eq!(true, e1.has_component(&mut state, ecs::ComponentType::Null));
    e1.remove_component(&mut state, ecs::ComponentType::Null);
    assert_eq!(false, e1.has_component(&mut state, ecs::ComponentType::Null));

    state.delete_entity(e1);

    println!("{:?}", state.null_components.get(e1));
    println!("{:?}", state.position_components.get(e1));
    println!("{:?}", state.entity_allocator.free);
}
