use rand;
use rand::Rng;

use crate::ecs as ecs;
use std::rc::Rc;

use ecs::Entity;

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
    let source_health: &ecs::HealthComponent =  world.health_components.get(&target_id).unwrap();
    let source_fighter: &ecs::FighterComponent = world.fighter_components.get(&source_id).unwrap();
    
    // TODO: maybe combine fighter and stats <31-05-20, vvvm23> //
    if !world.stats_components.contains_key(&source_id) {
        println!("Source is missing StatsComponent");
        return MoveResult::NoEffect;
    }
    if !world.stats_components.contains_key(&target_id) {
        println!("Source is missing StatsComponent");
        return MoveResult::NoEffect;
    }

    let source_stats: &ecs::StatsComponent = world.stats_components.get(&source_id).unwrap();
    let target_stats: &ecs::StatsComponent = world.stats_components.get(&source_id).unwrap();

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
    println!("{}", hp_power);
    let mut hp_power: f32 = hp_power as f32;
    hp_power *= match current_move.is_attack {
        true => (source_stats.attack as f32 / target_stats.defence as f32),
        false => (source_stats.support as f32 / 100.0),
    };
    println!("{}", hp_power);

    if current_move.crit {
        let roll: f32 = rng.gen::<f32>();
        let threshold: f32 = current_move.crit_chance + source_stats.crit + 0.1;
        if roll < threshold { // Crit
            println!("Critical Hit");
            hp_power *= 1.5;
        }
    }
    println!("{}", hp_power);
    let variation: f32 = rng.gen::<f32>() / 5.0 + 0.9;
    let hp_power: u16 = (hp_power * variation) as u16; // +- 10% damage variation
    println!("{}", hp_power);

    let sp_power: u16 = match current_move.sp_power {
        None => 0,
        Some(i) => i,
    };
    let mut sp_power: f32 = sp_power as f32;
    sp_power *= match current_move.is_attack {
        true => (source_stats.attack as f32 / target_stats.defence as f32),
        false => (source_stats.support as f32 / 100.0),
    };
    let sp_power: u16 = sp_power as u16;


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

    let roll: f32 = rng.gen::<f32>();
    let threshold: f32 = current_move.base_accuracy * (source_stats.accuracy as f32 / target_stats.agility as f32);
    if roll > threshold { // Miss
        println!("Missed the target");
        return MoveResult::NoEffect;
    }

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
