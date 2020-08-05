mod ecs;
use log;
use log::{info, error, debug};
use env_logger;

fn main() {
    env_logger::init();
    println!("Aloha World!");
    
    info!("This is an informative message.");
    
    let mut state = ecs::State::new();
    let e1 = state.entity_allocator.allocate()
        .add_position(&mut state, 0.0, 0.0)
        .add_null(&mut state);

    println!("{:?}", state.null_components.get(e1));
    println!("{:?}", state.position_components.get(e1));

    println!("{:?}", state.null_components);
}
