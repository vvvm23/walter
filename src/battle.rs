use crate::ecs as ecs;
use std::rc::Rc;

use ecs::Entity;

pub enum MoveResult {
    Effective,
    NoEffect,
}

// TODO: wait a minute.. didn't I say I would have a source fighter and a target health? what the
// hell is this? <30-05-20, vvvm23> //
pub fn execute_move(world: &mut ecs::World, source_id: u16, target_id: u16) -> MoveResult {
    if !world.health_components.contains_key(&target_id) {
        println!("Target cannot take damage!");
        return MoveResult::NoEffect;
    }

    let target_health: &ecs::HealthComponent = world.health_components.get(&target_id).unwrap();
    let target_fighter: &ecs::FighterComponent = world.fighter_components.get(&target_id).unwrap();
    let source_health: &ecs::HealthComponent =  world.health_components.get(&target_id).unwrap();
    let source_fighter: &ecs::FighterComponent = world.fighter_components.get(&source_id).unwrap();
    
    if let None = source_fighter.current_move {
        println!("No current move.");
        return MoveResult::NoEffect;
    }
    // Could panic if None, but should be handled by above statement
    let current_move: Rc<ecs::Move> = source_fighter.current_move.clone().unwrap().clone();

    let hp_power: u16 = match current_move.hp_power {
        None => 0,
        Some(i) => i,
    };
    let sp_power: u16 = match current_move.sp_power {
        None => 0,
        Some(i) => i,
    };
    //let target_status: Vec<ecs::StatusEffect> = match &current_move.target_status {
        //None => Vec::new(),
        //Some(i) => i,
    //};
    //let source_status: Vec<ecs::StatusEffect> = match &current_move.source_status {
        //None => Vec::new(),
        //Some(i) => i,
    //};
    let hp_cost: u16 = match current_move.hp_cost {
        None => 0,
        Some(i) => i,
    };
    let sp_cost: u16 = match current_move.sp_cost {
        None => 0,
        Some(i) => i,
    };

    println!("{}", current_move.use_message);

    // TODO: modify power and effects based on entity stats, equipment, status effects, etc. <29-05-20, vvvm23> //
    // TODO: calculate to hit chance using base chance, accuracy and evasion <29-05-20, vvvm23> //
    // TODO: calculate crit chance using base crit chance and other factors <29-05-20, vvvm23> //
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

    // decrease sp and hp on a miss or hit
    {
        // TODO: move this to correct place, so always lose even on miss or null <29-05-20, vvvm23> //
        // Again, take mutable reference in scope so we dont have two mutable references to same vector.
        let source_health: &mut ecs::HealthComponent =  world.health_components.get_mut(&target_id).unwrap();
        let source_fighter: &mut ecs::FighterComponent = world.fighter_components.get_mut(&source_id).unwrap();
        source_health.decrease_health(hp_cost);
        source_fighter.decrease_sp(sp_cost);
    }
    MoveResult::Effective
}
