use crate::component::physics as physics;
use crate::ecs::World;

// TODO: take delta to calculate proper change
pub fn velocity_system(world: &mut World) {
    for (e, vel) in world.velocity_components.iter() {
        assert!(world.position_components.contains_key(e), "Entity had velocity but no position!");
        let pos = world.position_components.get_mut(e).unwrap();
        let mut pos = pos.write().unwrap();
        let vel = vel.read().unwrap();
        pos.x += vel.dx;
        pos.y += vel.dy;
        println!("{} {}", pos.x, pos.y);
    }
}
