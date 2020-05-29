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
    let target_fighter: &mut ecs::FighterComponent = world.fighter_components.get_mut(target.id).unwrap();
    let source_health: &mut ecs::HealthComponent =  world.health_components.get_mut(target.id).unwrap();
    let source_fighter: &mut ecs::FighterComponent = world.fighter_components.get_mut(source.id).unwrap();

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
    };
    let hp_cost: u16 = match selected_move.hp_cost {
        None => 0,
        Some(i) => i,
    };
    let sp_cost: u16 = match selected_move.sp_cost {
        None => 0,
        Some(i) => i,
    };

    // TODO: modify power and effects based on entity stats, equipment, status effects, etc. <29-05-20, vvvm23> //
    // TODO: calculate to hit chance using base chance, accuracy and evasion <29-05-20, vvvm23> //
    // TODO: calculate crit chance using base crit chance and other factors <29-05-20, vvvm23> //
    // TODO: negate certain effects <29-05-20, vvvm23> //
    // TODO: proc abilities <29-05-20, vvvm23> //

    // adjust hp and sp on a hit
    if selected_move.is_attack {
        target_health.decrease_health(hp_power);
        target_fighter.decrease_sp(sp_power);
    } else {
        target_health.increase_health(hp_power);
        target_fighter.increase_sp(sp_power);
    }

    // decrease sp and hp on a miss or hit
    // TODO: move this to correct place, so always lose even on miss or null <29-05-20, vvvm23> //
    source_health.decrease_health(hp_cost);
    source_fighter.decrease_sp(sp_cost);

    MoveResult::Effective
}
