use crate::component::Component;
use std::sync::{Arc, RwLock};

/// All possible battle factions
#[derive(Debug)]
pub enum Faction {
    Ally,
    Enemy,
}

/// All possible AI systems
#[derive(Debug)]
pub enum AI {
    Random,
}

/// All possible Area-of-Effect targets
#[derive(Debug)]
pub enum AOETarget {
    All,
    Ally,
    Enemy,
}

/// All possible targets against a single
#[derive(Debug)]
pub enum SingleTarget {
    Ally,
    Enemy,
    User,
}

/// Defines how a move targets
#[derive(Debug)]
pub enum MoveTarget {
    Single(SingleTarget),
    AOE(AOETarget),
}

/// Structure to store Move data
#[derive(Debug)]
pub struct Move {
    name: String,
    description: String,
    use_message: String,

    hp_cost: u16, sp_cost: u16,
    power: Option<u16>, accuracy: f32,

    target: MoveTarget,
}

impl Move {
    /// Define a new move 
    pub fn new(
        name: &str, description: &str, use_message: &str,
        hp_cost: u16, sp_cost: u16,
        power: u16, accuracy: f32,
        target: MoveTarget,
    ) -> Arc<Move> {
        Arc::new(
            Move {
                name: name.to_string(), description: description.to_string(), use_message: use_message.to_string(),
                hp_cost: hp_cost, sp_cost: sp_cost,
                power: match power == 0 { true => None, false => Some(power) },
                accuracy: accuracy,
                target: target,
            }
        )
    }
}

/// Component that gives an entity the ability to take part in battles
#[derive(Debug)]
pub struct FighterComponent {
    display_name: String,
    level: u8,
    faction: Faction,
    pub moves: Vec<Arc<Move>>,

    max_hp: u16,    hp: u16,
    max_sp: u16,    sp: u16,

    attack: u16,    defence: u16,
    agility: u16,   luck: u16,
}

impl FighterComponent {
    /// Generate a new FighterComponent with empty move set
    pub fn new(
        display_name: &str,
        level: u8,
        faction: Faction,
        hp: u16, sp: u16,
        moves: Vec<Arc<Move>>,
        attack: u16, defence: u16,
        agility: u16, luck: u16,
    ) -> Component {
        Component::FighterComponent (
            FighterComponent {
                display_name: display_name.to_string(),
                level: level,
                faction: faction,
                moves: moves,

                max_hp: hp, hp: hp,
                max_sp: sp, sp: sp,
                attack: attack, defence: defence,
                agility: agility, luck: luck,
            }
        )
    }
}
