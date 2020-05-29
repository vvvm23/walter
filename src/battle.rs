use crate::ecs as ecs;

use ecs::Entity;

enum MoveResult {
    Effective,
    NoEffect,
}

fn execute_move(world: &mut ecs::World, source: &mut Entity, target: &mut Entity, selected_move: ecs::Move) -> MoveResult {
    if !world.health_components.contains_key(target.id) {
        println!("Target cannot take damage!");
        return MoveResult::NoEffect;
    }
    
    let target_health: &mut ecs::HealthComponent = world.health_components.get_mut(target.id).unwrap();
    let source_fighter: &mut ecs::FighterComponent = world.fighter_components.get(target.id).unwrap();

    let hp_power: u16 = match selected_move.hp_power {
        None => 0,
        Some(i) => i,
    };
    let sp_power: u16 = match selected_move.sp_power {
        None => 0,
        Some(i) => i,
    };
    let attack_status: Vec<StatusEffect> = match selected_move.attack_status {
        None => 0,
        Some(i) => i,
    };
    let defence_status: Vec<StatusEffect> = match selected_move.defence_status {
        None => 0,
        Some(i) => i,
    }

    // TODO: modify power and effects based on entity stats, equipment, status effects, etc. <29-05-20, vvvm23> //

    target_health.decrease_health(hp_power);
    source_fighter.decrease_sp(selected_move.sp_cost.unwrap())

    MoveResult::Effective
}


