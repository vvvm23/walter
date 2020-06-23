use rand;
use rand::Rng;

use crate::component::battle;
use crate::ecs::{Entity, World};
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

// TODO: AOE?
// TODO: maybe we actually want to precompute some things, such as crits for animation purposes
pub enum Action {
    Move(Arc<Entity>, Arc<battle::Move>, Arc<Entity>),
    Down(Arc<Entity>),
}

enum AOEOrSingle {
    Single(Arc<Entity>),
    AOE(battle::AOETarget),
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

    pub fn partition_entities(&self, source: Arc<Entity>, world: Arc<RwLock<World>>) -> (Vec<Arc<Entity>>, Vec<Arc<Entity>>) {
        let world = world.read().unwrap();
        let source_faction = world.fighter_components.get(&source).unwrap().read().unwrap().faction;
        let blufor: Vec<Arc<Entity>> = Vec::new();
        let opfor: Vec<Arc<Entity>> = Vec::new();
    
        for e in &self.entities {
            let target_faction = world.fighter_components.get(&source).unwrap().read().unwrap().faction;
            match source_faction {
                target_faction => blufor.push(Arc::clone(&e)),
                _ => opfor.push(Arc::clone(&e)),
            }
        }

        (blufor, opfor)
    }
} 

pub fn ai_handover(source: Arc<Entity>, instance: Arc<RwLock<BattleInstance>>, world: Arc<RwLock<World>>) -> (Arc<battle::Move>, AOEOrSingle) {
    match world.read().unwrap().fighter_components.get(&source).unwrap().read().unwrap().ai {
        battle::AI::Random => ai_random(source, instance, world),
    }
}

pub fn ai_random(source: Arc<Entity>, instance: Arc<RwLock<BattleInstance>>, world: Arc<RwLock<World>>) -> (Arc<battle::Move>, AOEOrSingle) {
    let mut rng = rand::thread_rng();
    let source_fighter = world.read().unwrap().fighter_components.get(&source).unwrap().write().unwrap();
    let nb_moves = source_fighter.moves.len() as u8;
    let random_pick = rng.gen_range(0, nb_moves) as usize;
    let random_move = Arc::clone(&source_fighter.moves[random_pick]);
    
    match random_move.target {
        battle::MoveTarget::AOE(t) => (random_move, AOEOrSingle::AOE(t)),
        battle::MoveTarget::Single(battle::SingleTarget::User) => (random_move, AOEOrSingle::Single(Arc::clone(&source))),
        battle::MoveTarget::Single(t) => {
            match t {
                battle::SingleTarget::Enemy => ,
                _ => ,
            }
        }
    }
}
    
