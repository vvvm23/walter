use crate::ecs as ecs;

use ecs::{PositionComponent, RotationComponent, VelocityComponent, RotationalVelocityComponent};
use ggez::{Context, GameResult};

// System to update position of components based on velocity
pub fn velocity_system(world: &mut ecs::World) {
    for (id, c) in world.velocity_components.iter() {
        if (world.position_components.contains_key(id)) {
            let pc: &mut ecs::PositionComponent = world.position_components.get_mut(id).unwrap();
            pc.translate_component(c);
            println!("{}: {}, {}", id, pc.x, pc.y);
        }
    }
}

// System to update rotation based on rotational velocity
pub fn rot_velocity_system(world: &mut ecs::World) {
    for (id, c) in world.rotational_velocity_components.iter() {
        if (world.rotation_components.contains_key(id)) {
            let rc: &mut ecs::RotationComponent = world.rotation_components.get_mut(id).unwrap();
            rc.rot += c.drot;
        }
    }
}

