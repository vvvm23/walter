mod ecs;
use log;
use log::{info, error, debug};
use env_logger;

fn main() {
    env_logger::init();
    println!("Aloha World!");
    
    info!("This is an informative message.");
    
    let mut x = ecs::State::new();

    for _ in 0.. {
        x.new_entity();
    }

}
