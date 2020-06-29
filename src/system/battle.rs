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
#[derive(Debug)]
pub enum Action {
    Move(Arc<Entity>, Arc<battle::Move>, Arc<Entity>),
    Down(Arc<Entity>),
}

// not sure if needed yet, may just be able to infer 
/// Defines current state of a battle loop such as blocking behaviour
pub enum BattleState {
    Started,
    Available,
    WaitingPlayer,
    WaitingEvent,
}

/// Defines current state of a battle
pub struct BattleInstance {
    pub enemy_name: String,
    pub win_message: Option<String>,
    pub loss_message: Option<String>,

    // background: Arc<ggez::graphics::Image>,
    // music: Arc<ggez::sound::SoundData>

    pub entities: Vec<Arc<Entity>>,
    pub entity_index: u8, // current entity that is the source
    pub actions: Vec<Action>, // action queue
    pub state: BattleState,
}

impl BattleInstance {
    pub fn new(enemy_name: &str) -> BattleInstance {
        BattleInstance {
            enemy_name: enemy_name.to_string(),
            win_message: None,
            loss_message: None,
            entities: Vec::new(),
            entity_index: 0,
            actions: Vec::new(),
            state: BattleState::Started,
        }
    }

    /// Add an entity to the instance
    pub fn add_entities(&mut self, es: &mut Vec<Arc<Entity>>) {
        self.entities.append(es);
    }

    /// Append a new action to the queue
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Helper function to partition entities into blufor and opfor, depending on the source's
    /// perspective.
    pub fn partition_entities(&self, source: Arc<Entity>, world: Arc<RwLock<World>>) -> (Vec<Arc<Entity>>, Vec<Arc<Entity>>) {
        let world = world.read().unwrap();
        let source_faction = &world.fighter_components.get(&source).unwrap().read().unwrap().faction;
        let mut blufor: Vec<Arc<Entity>> = Vec::new();
        let mut opfor: Vec<Arc<Entity>> = Vec::new();
    
        for e in &self.entities {
            let target_faction = &world.fighter_components.get(&source).unwrap().read().unwrap().faction;
            match source_faction == target_faction {
                true => blufor.push(Arc::clone(&e)),
                false => opfor.push(Arc::clone(&e)),
            }
        }

        (blufor, opfor)
    }
} 

/// Stores the result of a move to be executed later (typically after animation)
pub struct MoveResult {
    pub hit: bool,
    pub hp_cost: u16, pub sp_cost: u16,
    pub hp: u16, pub damaging: bool,
}

/// This function will calculate the damage, costs, etc. of an action 
pub fn calculate_effect(world: Arc<RwLock<World>>, source: Arc<Entity>, selected_move: Arc<battle::Move>, target: Arc<Entity>) -> MoveResult {
    let mut rng = rand::thread_rng();
    let roll: f32 = rng.gen();

    let hit = roll <= selected_move.accuracy; 
    match hit {
        true => {
            // random deviation +-10%
            let roll: f32 = rng.gen(); // [0.0, 1.0]
            let roll = roll / 5.0; // [0.0, 0.2]
            let roll = roll - 0.1; // [-0.1, 0.1]
            let dev = 1.0 + roll; // dev: [0.9, 1.1] aka +-10%

            MoveResult { 
                hit: true, 
                hp_cost: selected_move.hp_cost, 
                sp_cost: selected_move.sp_cost, 
                hp: match selected_move.power { None => 0, Some(i) => (i as f32*dev) as u16 },
                damaging: selected_move.damaging
            }
        },
        false => MoveResult {hit: false, hp_cost: 0, sp_cost: 0, hp: 0, damaging: selected_move.damaging}, // if it does not hit, reflect this in result
    }
}

/// This function will actually execute a MoveResult on the target
/// We can also generate our own MoveResult without using calculate effect to do scripted events
/// Or to force a hit, in the case of guaranteed connections
pub fn execute_effect(world: Arc<RwLock<World>>, source: Arc<Entity>, target: Arc<Entity>, result: MoveResult) {
    let world = world.read().unwrap();
    let mut source_fighter = world.fighter_components.get(&source).unwrap().write().unwrap();
    let mut target_fighter = world.fighter_components.get(&target).unwrap().write().unwrap();

    source_fighter.dec_sp(result.sp_cost);
    source_fighter.dec_hp(result.hp_cost);

    match result.damaging {
        true => target_fighter.dec_hp(result.hp),
        false => target_fighter.inc_hp(result.hp),
    }
}

/// This function will generate battle events
/// Another will handle the execution of the responses
pub fn battle_loop(world_lock: Arc<RwLock<World>>) {
    let instance = world_lock.read().unwrap();
    let instance = instance.battle_instance.as_ref().unwrap();
    let instance = instance.read().unwrap();

    let waiting = match instance.state {
        BattleState::Started => {
            // TODO: Sort by agility
            // TODO: Load certain instance attribute
            false
        },
        BattleState::Available => false,
        BattleState::WaitingEvent => true,
        BattleState::WaitingPlayer => true,
    };

    // This is technically a spin lock, kinda not epic.
    // Potential change is to rework with channels or events
    if waiting { return; }

    let source = Arc::clone(&instance.entities[instance.entity_index as usize]);
    let (random_move, random_target) = ai_handover(source, Arc::clone(&world_lock));

    drop(instance); // clear read lock (??? this feels wrong)

    let instance = world_lock.read().unwrap();
    let instance = instance.battle_instance.as_ref().unwrap();
    let mut instance = instance.write().unwrap();
    instance.state = BattleState::WaitingEvent;

    for t in &random_target {
        let source = Arc::clone(&instance.entities[instance.entity_index as usize]);
        instance.actions.push(Action::Move(Arc::clone(&source), Arc::clone(&random_move), Arc::clone(t)));
    }
}

/// Function that hands over to correct ai handler based on fighter ai enum
pub fn ai_handover(source: Arc<Entity>, world: Arc<RwLock<World>>) -> (Arc<battle::Move>, Vec<Arc<Entity>>) {
    assert!(world.read().unwrap().fighter_components.contains_key(&source), "Entity does not have FigherComponent!");
    match world.read().unwrap().fighter_components.get(&source).unwrap().read().unwrap().ai {
        battle::AI::Random => ai_random(source, Arc::clone(&world)),
    }
}

/// A completely random AI, subject only to a few checks
pub fn ai_random(source: Arc<Entity>, world: Arc<RwLock<World>>) -> (Arc<battle::Move>, Vec<Arc<Entity>>) {
    let mut rng = rand::thread_rng();
    let source_fighter = world.read().unwrap();
    let source_fighter = source_fighter.fighter_components.get(&source).unwrap().read().unwrap();

    let instance = world.read().unwrap();
    let instance = instance.battle_instance.as_ref().unwrap();
    let instance = instance.read().unwrap();

    // Filter out moves that are not possible given current sp and hp
    let random_candidates: Vec<Arc<battle::Move>> = source_fighter.moves
        .iter()
        .filter(|m| m.sp_cost <= source_fighter.sp && m.hp_cost < source_fighter.hp)
        .cloned()
        .collect();

    let nb_moves = random_candidates.len() as u8;

    if nb_moves == 0 {
        // Implement some kind of struggle or cower move when no move is possible
        todo!("No possible move!");
    }

    let random_pick = rng.gen_range(0, nb_moves) as usize;
    let random_move = Arc::clone(&random_candidates[random_pick]); 
    
    // A rather complicated set of nested matches to determine correct random targetting based on
    // available targets.
    // TODO: No possible target handling
    // TODO: Handling of not being able to target downed for certain moves
    match &Arc::clone(&random_move).target {
        // Some AOE attack
        battle::MoveTarget::AOE(t) => (random_move, match t {

            // An AOE attack on all entities in the battle. EG. Environmental changes, colossal AOE
            // damage, special moves such as Trick Room
            battle::AOETarget::All => { 
                instance.entities.clone()
            },

            // AOE attack on all allied entities. EG. Mass heals, buffs
            battle::AOETarget::Ally => {
                let (blufor, _) = instance.partition_entities(Arc::clone(&source), Arc::clone(&world));
                blufor
            },

            // AOE attack on all enemy entities. EG: Mass attack, debuffs
            battle::AOETarget::Enemy => {
                let (_, opfor) = instance.partition_entities(Arc::clone(&source), Arc::clone(&world));
                opfor
            },
        }),

        // When a move can only be used on the user itself. EG. Charge, Concentrate, Endure,
        // We can't make someone else concentrate!
        battle::MoveTarget::Single(battle::SingleTarget::User) => (random_move, vec![Arc::clone(&source)]),

        // The move can only be used on a single target (that is not just on the user). EG: Single
        // target attacks, heals, buffs, debuffs. Usually cheaper than AOE counterparts
        battle::MoveTarget::Single(t) => {
            let (blufor, opfor) = instance.partition_entities(Arc::clone(&source), Arc::clone(&world));
            let candidate_targets = match t {
                battle::SingleTarget::Enemy => opfor,
                _ => blufor,
            }; 
            let nb_targets = candidate_targets.len() as u8;
            if nb_targets == 0 {
                return (random_move, vec![Arc::clone(&source)]); // TODO: Handle this properly
            }

            let random_pick = rng.gen_range(0, nb_targets) as usize; // panic occurs here!
            let random_target = Arc::clone(&candidate_targets[random_pick]);
            (random_move, vec![Arc::clone(&random_target)])
        }
    }
}
