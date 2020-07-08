use crate::component::physics as physics;
use crate::ecs::World;
use std::sync::{Arc, RwLock};

// TODO: take delta to calculate proper change
pub fn velocity_system(world: Arc<RwLock<World>>, d_time: &f64) {
    let world = world.read().unwrap();
    let d_time = *d_time as f32;
    for (e, vel) in world.velocity_components.iter() {
        assert!(world.position_components.contains_key(e), "Entity had velocity but no position!");
        let pos = world.position_components.get(e).unwrap();
        let mut pos = pos.write().unwrap();
        let vel = vel.read().unwrap();
        pos.x += vel.dx * d_time;
        pos.y += vel.dy * d_time;
        //println!("{}: {} {}", e.id, pos.x, pos.y);
    }
}

pub fn idle_bob_system(world: Arc<RwLock<World>>, d_time: &f64) {
    let world = world.read().unwrap();
    let d_time = *d_time as f32;
    for (e, bob) in world.idle_bob_components.iter() {
        let mut bob = bob.write().unwrap();
        bob.update(&d_time);
    }
}
