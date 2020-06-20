use crate::component::battle;
use crate::ecs::{Entity};
use std::sync::{Arc, RwLock};

// Battle system plan
//
// 1. Get next entity to move
// 2. Select a move and target(s) in battle thread
// 3. Push Move Action to action queue
// 4. In main thread perform animation, audio for move (main launches another thread to execute
//    animation)
// 5. In battle thread apply effects of move
// 6. If something else happens, push this action as a result
// 7. In main thread handle sound and animation for resulting action
// 8. Rotate entity vector

// Alternate strategy
//
// All battle logic in another thread, including animation
// Rendering still occurs in main thread
// We send requests for audio to be played to main thread and it will execute.
// Not sure how to synchronise between threads, just waiting is unstable

pub enum Action {
    Move(Arc<Entity>, Arc<battle::Move>, Arc<Entity>),
    Down(Arc<Entity>),
}

// not sure if needed yet, may just be able to infer 
pub enum BattleState {
    Started,
    Available,
    WaitingPlayer,
    WaitingEvent,
}

pub struct BattleInstance {
    pub enemy_name: String,
    pub win_message: Option<String>,
    pub loss_message: Option<String>,

    // background: Arc<ggez::graphics::Image>,
    // music: Arc<ggez::sound::SoundData>

    pub entities: Vec<Arc<Entity>>,
    pub actions: Vec<Action>,
    pub state: BattleState,
}

impl BattleInstance {
    pub fn new(enemy_name: &str) -> BattleInstance {
        BattleInstance {
            enemy_name: enemy_name.to_string(),
            win_message: None,
            loss_message: None,
            entities: Vec::new(),
            actions: Vec::new(),
            state: BattleState::Started,
        }
    }

    pub fn add_entities(&mut self, es: &mut Vec<Arc<Entity>>) {
        self.entities.append(es);
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
} 

