use crate::component::physics as physics;
use crate::ecs::World;
use std::sync::{Arc, RwLock};

// TODO: take delta to calculate proper change
pub fn velocity_system(world: Arc<RwLock<World>>) {
    let world = world.read().unwrap();
    for (e, vel) in world.velocity_components.iter() {
        assert!(world.position_components.contains_key(e), "Entity had velocity but no position!");
        let pos = world.position_components.get(e).unwrap();
        let mut pos = pos.write().unwrap();
        let vel = vel.read().unwrap();
        pos.x += vel.dx;
        pos.y += vel.dy;
        //println!("{}: {} {}", e.id, pos.x, pos.y);
    }
}
