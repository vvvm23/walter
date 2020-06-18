use crate::component::rendering;
use crate::ecs::World;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::sync::{Arc, RwLock};

pub fn primitive_rendering_system(world: Arc<RwLock<World>>, ctx: &mut Context) -> GameResult {
    let world = world.read().unwrap();
    for (e, prim) in world.primitive_components.iter() {
        let point = match world.position_components.contains_key(e) {
            true => {
                let p = world.position_components.get(e).unwrap();
                let p = p.read().unwrap();
                na::Point2::new(p.x, p.y)
            },
            false => na::Point2::new(0.0, 0.0),
        };

        let mesh = prim.read().unwrap().build_mesh(ctx);
        let draw_param = graphics::DrawParam::default()
            .dest(point);
        graphics::draw(ctx, &mesh, draw_param)?;
    }

    Ok(())
}
