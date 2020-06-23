use crate::component::rendering;
use crate::component::battle;
use crate::system;
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
        assert!(self.lookup_texture.contains_key(&path), format!("Requested image file has not beem loaded! {}", path));
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

pub fn ally_stats_rendering_system(world: Arc<RwLock<World>>, ctx: &mut Context) -> GameResult {
    const TEXT_PAD: f32 = 10.0;
    const INTERVAL: usize = 20;
    let world = world.read().unwrap();
    let ins = world.battle_instance.as_ref().unwrap();
    let ins = Arc::clone(ins);
    let ins = ins.read().unwrap();

    for (i, e) in ins.entities.iter().enumerate() {
        let fighter = Arc::clone(world.fighter_components.get(e).unwrap());
        let fighter = fighter.read().unwrap();

        if let battle::Faction::Enemy = fighter.faction {
            continue;
        }

        draw_container(1200.0 - TEXT_PAD,
                       100.0 - TEXT_PAD + (i*INTERVAL) as f32,
                       300.0, 150.0,
                       [0.2, 0.2, 0.2, 1.0].into(),
                       ctx)?;
        
        let name_text: graphics::Text = graphics::Text::new(format!("{0: <10} LVL: {1}", fighter.display_name, fighter.level));
        graphics::draw(ctx, &name_text, (na::Point2::new(1200.0, 100.0+(i*INTERVAL) as f32), graphics::WHITE))?;
        
        let health_text: graphics::Text = graphics::Text::new(format!("{0: <5} {1} / {2}", "HP:", fighter.hp, fighter.max_hp));
        graphics::draw(ctx, &health_text, (na::Point2::new(1200.0, 100.0+20.0+(i*INTERVAL) as f32), graphics::WHITE))?;

        let sp_text: graphics::Text = graphics::Text::new(format!("{0: <5} {1} / {2}", "SP:", fighter.sp, fighter.max_sp));
        graphics::draw(ctx, &sp_text, (na::Point2::new(1200.0, 100.0+40.0+(i*INTERVAL) as f32), graphics::WHITE))?;

        if let Some(sprite) = fighter.profile_sprite.as_ref() {
            let draw_param = graphics::DrawParam::default()
                .dest(na::Point2::new(1350.0, 100.0 + (i*INTERVAL) as f32));

            graphics::draw(ctx, &**sprite, draw_param)?;
        }
    }

    Ok(())
}

pub fn draw_container(x: f32, y: f32, xs: f32, ys: f32, colour: graphics::Color, ctx: &mut Context) -> GameResult {
    const BORDER_SIZE: f32 = 3.0;
    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {x: 0.0, y: 0.0, w: xs, h: ys},
        graphics::WHITE,
    ).unwrap();
    let draw_param = graphics::DrawParam::default()
        .dest(na::Point2::new(x, y));
    graphics::draw(ctx, &mesh, draw_param)?;

    let mesh: graphics::Mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {x: BORDER_SIZE, y: BORDER_SIZE, w: xs - 2.0*BORDER_SIZE, h: ys - 2.0*BORDER_SIZE},
        colour,
    ).unwrap();

    let draw_param = graphics::DrawParam::default()
        .dest(na::Point2::new(x, y));
    graphics::draw(ctx, &mesh, draw_param)?;
    Ok(())

}

pub fn draw_fps(ctx: &mut Context) -> GameResult {
    let fps_text = graphics::Text::new(format!("FPS: {}", ggez::timer::fps(ctx) as u32));
    let draw_param = graphics::DrawParam::default()
        .dest(na::Point2::new(20.0, 20.0));
    draw_container(15.0, 15.0, 70.0, 30.0, [0.0, 0.0, 0.0, 1.0].into(), ctx)?;
    graphics::draw(ctx, &fps_text, draw_param)?;

    Ok(())
}

pub fn textbox_rendering_system(world: Arc<RwLock<World>>, ctx: &mut Context) -> GameResult {
    let world = world.read().unwrap();
    for (e, textbox) in world.text_box_components.iter() {
        let textbox = textbox.read().unwrap();
        assert!(world.position_components.contains_key(e), "Entity with TextBoxComponent does not have PositionComponent!");
        let position = Arc::clone(world.position_components.get(e).unwrap());
        let position = position.read().unwrap();
        textbox.draw(position.x, position.y, ctx)?;
    }

    Ok(())
}
