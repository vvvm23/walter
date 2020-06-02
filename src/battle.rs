use rand;
use rand::Rng;

use crate::ecs as ecs;
use std::rc::Rc;
use std::cmp::Ordering;

use ecs::Entity;

pub enum BattleResult {
    Win,
    Loss,
    GameOver,
    Retreat,
}

/*
 *  Main Battle Loop
 *  
 *  Inputs  -> player team entity ids as a vector
 *          -> enemy team entity ids as a vector
 *          -> ecs::World struct
 *
 *  Outputs -> BattleResult enum. Can be handled by caller
 *
 *  Plan:
 *      1.  Sort entity ids by speeds of their corresponding StatsComponent s
 *      2.  Execute any special events such as dialogue, status effects, abilities
 *      3.  Run AI procedure for each entity in turn
 *      4.  Execute that Move
 *      5.  Remove entity from turn cycle if they lose all health or run
 *      6.  Loop until a side wins or runs
 *
*/

struct IdFighter<'a> {
    id: u16,
    fighter: &'a ecs::FighterComponent,
 }

impl<'a> Ord for IdFighter<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fighter.cmp(&other.fighter)
    }
}
impl<'a> Eq for IdFighter<'a> {
    //fn eq(&self, other: &Self) -> bool {
        //self.agility == other.agility
    //}
}
impl<'a> PartialOrd for IdFighter<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for IdFighter<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.fighter == other.fighter
    }
}

// TODO: entity sorting in another function <31-05-20, vvvm23> //
// TODO: struct to contain id and fighter lists for teams <31-05-20, vvvm23> // 
pub fn battle_loop(world: &mut ecs::World, mut blufor: Vec<u16>, mut opfor: Vec<u16>) -> BattleResult {

    let mut fighters: Vec<IdFighter> = Vec::new();

    for i in blufor {
        fighters.push(IdFighter {
            id: i,
            fighter: world.fighter_components.get(&i).unwrap().clone(),
        });
    }
    for i in opfor {
        fighters.push(IdFighter {
            id: i,
            fighter: world.fighter_components.get(&i).unwrap().clone(),
        });
    }
    fighters.sort();
    blufor = Vec::new();
    opfor = Vec::new();
    let mut all: Vec<u16> = Vec::new();

    for f in fighters {
        match f.fighter.faction {
            ecs::Faction::Ally => blufor.push(f.id),
            ecs::Faction::Player => blufor.push(f.id),
            _ => opfor.push(f.id),
        };
        all.push(f.id);
    }

    for id in &all {
        let fighter: &ecs::FighterComponent = world.fighter_components.get(&id).unwrap();
        let is_control: bool = match fighter.faction {
            ecs::Faction::Player => true,
            _ => false,
        };

        if is_control {
            // Prompt user to select choice
        } else {
            // Let AI function choose function
            ai_handover(world, id, &blufor, &opfor);
        }

    }

    

    BattleResult::Win // default is to win.
}

#[derive(Copy, Clone)]
enum TargetOrAOE {
    Target(u16),
    AOE(ecs::AreaTarget),
}

fn ai_handover(world: &mut ecs::World, source_id: &u16, blufor: &Vec<u16>, opfor: &Vec<u16>) -> (Rc<ecs::Move>, TargetOrAOE) {
    match world.fighter_components.get(source_id).unwrap().ai {
        ecs::AI::Random => ai_random(world, source_id, blufor, opfor),
        _ => ai_random(world, source_id, blufor, opfor),
    }
}

fn ai_random(world: &mut ecs::World, source_id: &u16, blufor: &Vec<u16>, opfor: &Vec<u16>) -> (Rc<ecs::Move>, TargetOrAOE) {
    let mut rng = rand::thread_rng();
    let nb_moves: u8 = world.fighter_components.get(source_id).unwrap().moves.len() as u8;
    let random_pick: usize = rng.gen_range(0, nb_moves) as usize;
    let random_move: Rc<ecs::Move> = world.fighter_components.get(source_id).unwrap().moves[random_pick].clone();

    if random_move.aoe {
        let aoe_target: ecs::AreaTarget = random_move.aoe_target.unwrap();
        return (random_move, TargetOrAOE::AOE(aoe_target));
    }

    let random_target: u16 = match random_move.is_attack {
        true => {
            let nb_targets: u8 = opfor.len() as u8;
            let random_pick: usize = rng.gen_range(0, nb_targets) as usize;
            opfor[random_pick]
        },
        false => {
            let nb_targets: u8 = blufor.len() as u8;
            let random_pick: usize = rng.gen_range(0, nb_targets) as usize;
            blufor[random_pick]
        },
    };
    (random_move, TargetOrAOE::Target(random_target))

}

pub enum MoveResult {
    Effective,
    NoEffect,
}

// TODO: use of entity struct <31-05-20, vvvm23> //
pub fn execute_move(world: &mut ecs::World, source_id: u16, target_id: u16) -> MoveResult {
    let mut rng = rand::thread_rng(); // TODO: should this be here?

    if !world.health_components.contains_key(&target_id) {
        println!("Target cannot take damage!");
        return MoveResult::NoEffect;
    }

    let target_health: &ecs::HealthComponent = world.health_components.get(&target_id).unwrap();
    let target_fighter: &ecs::FighterComponent = world.fighter_components.get(&target_id).unwrap();
    let source_health: &ecs::HealthComponent =  world.health_components.get(&source_id).unwrap();
    let source_fighter: &ecs::FighterComponent = world.fighter_components.get(&source_id).unwrap();
    
    if let None = source_fighter.current_move {
        println!("Source does not have a selected move");
        return MoveResult::NoEffect;
    }
    // Could panic if None, but should be handled by above statement
    let current_move: Rc<ecs::Move> = source_fighter.current_move.clone().unwrap().clone();
    println!("{}", current_move.use_message);

    let hp_power: u16 = match current_move.hp_power {
        None => 0,
        Some(i) => i,
    };
    let mut hp_power: f32 = hp_power as f32;
    hp_power *= match current_move.is_attack {
        true => (source_fighter.attack as f32 / target_fighter.defence as f32),
        false => (source_fighter.support as f32 / 100.0),
    };

    if current_move.crit {
        let roll: f32 = rng.gen::<f32>();
        let threshold: f32 = current_move.crit_chance + source_fighter.crit + 0.1;
        if roll < threshold { // Crit
            println!("$source dealt a critical hit!");
            hp_power *= 1.5;
        }
    }
    let variation: f32 = rng.gen::<f32>() / 5.0 + 0.9;
    let hp_power: u16 = (hp_power * variation).ceil() as u16; // +- 10% damage variation

    let sp_power: u16 = match current_move.sp_power {
        None => 0,
        Some(i) => i,
    };
    let mut sp_power: f32 = sp_power as f32;
    sp_power *= match current_move.is_attack {
        true => (source_fighter.attack as f32 / target_fighter.defence as f32),
        false => (source_fighter.support as f32 / 100.0),
    };
    let sp_power: u16 = sp_power as u16;

    let hp_cost: u16 = match current_move.hp_cost {
        None => 0,
        Some(i) => i,
    };
    let sp_cost: u16 = match current_move.sp_cost {
        None => 0,
        Some(i) => i,
    };

    // decrease sp and hp on a miss or hit
    {
        // Again, take mutable reference in scope so we dont have two mutable references to same vector.
        let source_health: &mut ecs::HealthComponent =  world.health_components.get_mut(&target_id).unwrap();
        let source_fighter: &mut ecs::FighterComponent = world.fighter_components.get_mut(&source_id).unwrap();
        source_health.decrease_health(hp_cost);
        source_fighter.decrease_sp(sp_cost);
    }

    // reset to immutable
    let source_fighter: &ecs::FighterComponent = world.fighter_components.get(&source_id).unwrap();
    let target_fighter: &ecs::FighterComponent = world.fighter_components.get(&target_id).unwrap();

    let roll: f32 = rng.gen::<f32>();
    let threshold: f32 = current_move.base_accuracy * (source_fighter.accuracy as f32 / target_fighter.agility as f32);
    if roll > threshold { // Miss
        println!("$target dodged the attack!");
        return MoveResult::NoEffect;
    }

    // TODO: negate certain effects <29-05-20, vvvm23> //
    // TODO: proc abilities <29-05-20, vvvm23> //
    {
        // Take mutable reference in scope so we dont have two mutable references to same vector.
        let target_health: &mut ecs::HealthComponent = world.health_components.get_mut(&target_id).unwrap();
        let target_fighter: &mut ecs::FighterComponent = world.fighter_components.get_mut(&target_id).unwrap();

        // adjust hp and sp on a hit
        if current_move.is_attack {
            target_health.decrease_health(hp_power);
            target_fighter.decrease_sp(sp_power);
            println!("Dealt {} hp damage", hp_power);
        } else {
            target_health.increase_health(hp_power);
            target_fighter.increase_sp(sp_power);
        }
    }

    MoveResult::Effective
}
