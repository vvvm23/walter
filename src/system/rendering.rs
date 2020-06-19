use crate::component::rendering;
use crate::ecs::World;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub struct TextureAtlas {
    lookup_texture: HashMap<String, Arc<ggez::graphics::Image>>,
}

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        TextureAtlas {
            lookup_texture: HashMap::new(),
        }
    } 

    pub fn load(&mut self, ctx: &mut ggez::Context, path: &str) {
        self.lookup_texture.insert(
            path.to_string(),
            Arc::new(ggez::graphics::Image::new(ctx, path).unwrap())
        );
    }

    pub fn get(&self, path: &str) -> Arc<ggez::graphics::Image> {
        let path = path.to_string();
        assert!(self.lookup_texture.contains_key(&path));
        Arc::clone(self.lookup_texture.get(&path).unwrap())
    }
}

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

pub fn sprite_rendering_system(world: Arc<RwLock<World>>, ctx: &mut Context) -> GameResult {
    let world = world.read().unwrap();
    for (e, sprite) in world.sprite_components.iter() {
        let sprite = sprite.read().unwrap();
        let point = match world.position_components.contains_key(e) {
            true => {
                let p = world.position_components.get(e).unwrap();
                let p = p.read().unwrap();
                na::Point2::new(p.x, p.y)
            },
            false => na::Point2::new(0.0, 0.0),
        };

        let draw_param = graphics::DrawParam::default()
            .dest(point)
            .scale(sprite.scale);

        graphics::draw(ctx, &*sprite.texture, draw_param)?;
    }

    Ok(())
}

pub fn background_rendering_system(world: Arc<RwLock<World>>, ctx: &mut Context) -> GameResult {
    let world = world.read().unwrap();
    for (e, back) in world.background_components.iter() {
        let back = back.read().unwrap();
        let draw_param = graphics::DrawParam::default()
            .scale(back.scale);
    
        graphics::draw(ctx, &*back.texture, draw_param)?;
    }

    Ok(())
}
