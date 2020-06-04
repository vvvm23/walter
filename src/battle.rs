use rand;
use rand::Rng;

use crate::ecs as ecs;
use crate::rendering as rendering;

use std::rc::Rc;
use std::{thread, time};
use std::cmp::Ordering;

use ecs::Entity;

use ggez::graphics as graphics;
use ggez::Context as Context;

// enum to represent the result of a battle
pub enum BattleResult {
    Win, // Victory to player
    Loss, // Loss to the player, but game is not over
    GameOver, // Loss to the player. Game terminated.
    Retreat, // Loss to the player. Retreat from battle
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

// Combining fighter and id in order to sort id by associated agility
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
// TODO: split this up into more functions in general <02-06-20, vvvm23> //
// TODO: load background on start, perhaps a battle struct? <04-06-20, vvvm23> //
// Main battle loop.
pub fn battle_loop(world: &mut ecs::World, ctx: &mut Context, mut blufor: Vec<u16>, mut opfor: Vec<u16>) -> BattleResult {
    let background: graphics::Image = graphics::Image::new(ctx, "/forest.png").unwrap();
    let mut draw_param = graphics::DrawParam::default();
    loop {
        // Sort the fighters by speed
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

            graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
            graphics::draw(ctx, &background, draw_param);
            rendering::draw_friendly_stats(world, ctx, &vec![0,1]);
            graphics::present(ctx);

            // If the current entity is dead, just skip
            // TODO: make sure it cant be a target as well <02-06-20, vvvm23> //
            if !world.health_components.get(&id).unwrap().alive {
                continue;
            }

            let fighter: &ecs::FighterComponent = world.fighter_components.get(&id).unwrap();   
            // Check whether the current turn is controllable by the player
            let is_control: bool = match fighter.faction {
                ecs::Faction::Player => true,
                _ => false,
            };

            // Get the move and target from either the player or AI
            let (selected_move, selected_target) = match is_control {
                true => player_choice(world, id, &blufor, &opfor),
                false => ai_handover(world, id, &blufor, &opfor),
            };
    
            // Set the current move
            let fighter: &mut ecs::FighterComponent = world.fighter_components.get_mut(&id).unwrap();
            fighter.current_move = Some(selected_move);
            let fighter: &ecs::FighterComponent = world.fighter_components.get(&id).unwrap();

            // Either execute a single move, or against multiple targets
            if let TargetOrAOE::Target(tid) = selected_target {
                execute_move(world, *id, tid);
            } else if let TargetOrAOE::AOE(at) = selected_target {
                let targets: &Vec<u16> = match at {
                    ecs::AreaTarget::Ally => match is_control {
                        true => &blufor,
                        false => &opfor,
                    },
                    ecs::AreaTarget::Enemy => match is_control {
                        true => &opfor,
                        false => &blufor,
                    },
                    ecs::AreaTarget::All => &all,
                };
                execute_aoe(world, *id, targets);
            }

            // Check if either team has been defeated
            if check_dead(world, &blufor) {
                return BattleResult::GameOver; // Lose if blufor is dead, even if opfor dead
            }
            if check_dead(world, &opfor) {
                return BattleResult::Win;
            }
            println!("");
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
    BattleResult::Win // default is to win.
}

// Helper function to check if a set of entities is dead, based on their ids
fn check_dead(world: &mut ecs::World, team: &Vec<u16>) -> bool {
    for i in team {
        if world.health_components.get(&i).unwrap().alive {
            return false;
        }
    }
    true 
}

// Enum to hold whether an action targets a single target or is multi-target
#[derive(Copy, Clone)]
enum TargetOrAOE {
    Target(u16),
    AOE(ecs::AreaTarget),
}

// Function to get the player's move and target choice
fn player_choice(world: &mut ecs::World, source_id: &u16, blufor: &Vec<u16>, opfor: &Vec<u16>) -> (Rc<ecs::Move>, TargetOrAOE) {
    // TODO: replace this with user prompts <02-06-20, vvvm23> //
    ai_handover(world, source_id, blufor, opfor)
}

// Function that simply hands over to the appropriate ai function based on fighter.ai
fn ai_handover(world: &mut ecs::World, source_id: &u16, blufor: &Vec<u16>, opfor: &Vec<u16>) -> (Rc<ecs::Move>, TargetOrAOE) {
    match world.fighter_components.get(source_id).unwrap().ai {
        ecs::AI::Random => ai_random(world, source_id, blufor, opfor),
        _ => ai_random(world, source_id, blufor, opfor),
    }
}

// Function for AI::Random. Completely random move and target
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

// Enum to return the effect of the move on a target
pub enum MoveResult {
    Effective,
    NoEffect,
}

// Helper function to execute move on a vector of targets
fn execute_aoe(world: &mut ecs::World, source_id: u16, targets: &Vec<u16>) {
    for t in targets {
        execute_move(world, source_id, *t);
    }
}

// TODO: use of entity struct <31-05-20, vvvm23> //
// Function to execute the current move in the source on the target
fn execute_move(world: &mut ecs::World, source_id: u16, target_id: u16) -> MoveResult {
    let mut rng = rand::thread_rng(); // TODO: should this be here?

    // If there is no health component, the target cannot take damage.
    if !world.health_components.contains_key(&target_id) {
        println!("Target cannot take damage!");
        return MoveResult::NoEffect;
    }

    let target_health: &ecs::HealthComponent = world.health_components.get(&target_id).unwrap();
    let target_fighter: &ecs::FighterComponent = world.fighter_components.get(&target_id).unwrap();
    let source_health: &ecs::HealthComponent =  world.health_components.get(&source_id).unwrap();
    let source_fighter: &ecs::FighterComponent = world.fighter_components.get(&source_id).unwrap();
    
    // If no move is selected, return
    if let None = source_fighter.current_move {
        println!("Source does not have a selected move");
        return MoveResult::NoEffect;
    }
    // Could panic if None, but should be handled by above statement
    let current_move: Rc<ecs::Move> = source_fighter.current_move.clone().unwrap().clone();
    println!("{}", current_move.use_message);

    // Get the base hp power
    let hp_power: u16 = match current_move.hp_power {
        None => 0,
        Some(i) => i,
    };
    // Increase power based on source attack and target defence
    let mut hp_power: f32 = hp_power as f32;
    hp_power *= match current_move.is_attack {
        true => (source_fighter.attack as f32 / target_fighter.defence as f32), // attacking move
        false => (source_fighter.support as f32 / 100.0), // healing move
    };

    // If the move can crit, roll for crit and update hp
    if current_move.crit {
        let roll: f32 = rng.gen::<f32>();
        let threshold: f32 = current_move.crit_chance + source_fighter.crit + 0.1;
        if roll < threshold { // Crit
            println!("$source dealt a critical hit!");
            hp_power *= 1.5;
        }
    }
    
    // Apply some variation to the damage
    let variation: f32 = rng.gen::<f32>() / 5.0 + 0.9;
    let hp_power: u16 = (hp_power * variation).ceil() as u16; // +- 10% damage variation

    // Get sp damage and apply modifiers like before (no crit though)
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

    // Get move costs
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

    // Roll for accuracy. On a miss, break early
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
