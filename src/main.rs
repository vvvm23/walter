mod ecs;
use log;
use log::{info, error, debug};
use env_logger;

fn main() {
    env_logger::init();
    println!("Aloha World!");
    
    info!("This is an informative message.");

    let mut x = ecs::GenerationalIndexAllocator::new(256);

    loop {
        let i1 = x.allocate();
        println!("{:?}", i1);
        x.deallocate(i1);
    }

}
