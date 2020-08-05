mod ecs;
use log;
use log::{info, error, debug};
use env_logger;

fn main() {
    env_logger::init();
    println!("Aloha World!");
    
    info!("This is an informative message.");
    
    let mut x = ecs::State::new();
    let e1 = x.entity_allocator.allocate();
    x.null_components.set(e1, ecs::NullComponent{owner: e1.clone()});
    x.position_components.set(e1, ecs::PositionComponent{owner: e1.clone(), x: 0.0, y: 0.0});

    println!("{:?}", x.null_components.get(e1));
    println!("{:?}", x.position_components.get(e1));

    println!("{:?}", x.null_components);
}
