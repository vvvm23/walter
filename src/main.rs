mod ecs;
use log;
use log::{info, error, debug};
use env_logger;

fn main() {
    env_logger::init();
    println!("Aloha World!");
    
    let mut game_state = ecs::State::new();
    let x = &mut game_state.entity_allocator;

    for _ in 0..5 {
        x.new_entity();
    }
    
    info!("This is an informative message.")

}
