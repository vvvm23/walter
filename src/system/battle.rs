use crate::component::battle;
use crate::ecs::{Entity};
use std::sync::{Arc, RwLock};

pub struct BattleInstance {
    pub enemy_name: String,
    pub win_message: Option<String>,
    pub loss_message: Option<String>,

    // background: Arc<ggez::graphics::Image>,
    // music: Arc<ggez::sound::SoundData>

    pub entities: Vec<Arc<Entity>>,
}

impl BattleInstance {
    pub fn new(enemy_name: &str) -> BattleInstance {
        BattleInstance {
            enemy_name: enemy_name.to_string(),
            win_message: None,
            loss_message: None,
            entities: Vec::new(),
        }
    }

    pub fn add_entities(&mut self, es: &mut Vec<Arc<Entity>>) {
        self.entities.append(es);
    }
} 

