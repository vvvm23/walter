use crate::component::Component;
use std::sync::{Arc, RwLock};

/// All possible battle factions
#[derive(Debug, PartialEq)]
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
#[derive(Debug, Clone, Copy)]
pub enum AOETarget {
    All,
    Ally,
    Enemy,
}

/// All possible targets against a single
#[derive(Debug, Clone, Copy)]
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
    pub name: String,
    pub description: String,
    pub use_message: String,

    pub hp_cost: u16, pub sp_cost: u16,
    pub power: Option<u16>, pub damaging: bool,
    pub accuracy: f32,

    pub target: MoveTarget, 
}

impl Move {
    /// Define a new move 
    pub fn new(
        name: &str, description: &str, use_message: &str,
        hp_cost: u16, sp_cost: u16,
        power: u16, damaging: bool,
        accuracy: f32,
        target: MoveTarget,
    ) -> Arc<Move> {
        Arc::new(
            Move {
                name: name.to_string(), description: description.to_string(), use_message: use_message.to_string(),
                hp_cost: hp_cost, sp_cost: sp_cost,
                power: match power == 0 { true => None, false => Some(power) },
                damaging: damaging,
                accuracy: accuracy,
                target: target,
            }
        )
    }
}

/// Component that gives an entity the ability to take part in battles
#[derive(Debug)]
pub struct FighterComponent {
    pub display_name: String,
    pub level: u8,
    pub faction: Faction,
    pub ai: AI,
    pub moves: Vec<Arc<Move>>,

    pub max_hp: u16,    pub hp: u16,
    pub max_sp: u16,    pub sp: u16,

    pub attack: u16,    pub defence: u16,
    pub agility: u16,   pub luck: u16,

    pub profile_sprite: Option<Arc<ggez::graphics::Image>>,
}

impl FighterComponent {
    /// Generate a new FighterComponent with empty move set
    pub fn new(
        display_name: &str,
        level: u8,
        faction: Faction,
        ai: AI,
        hp: u16, sp: u16,
        moves: Vec<Arc<Move>>,
        attack: u16, defence: u16,
        agility: u16, luck: u16,
        profile_sprite: Option<Arc<ggez::graphics::Image>>,
    ) -> Component {
        Component::FighterComponent (
            FighterComponent {
                display_name: display_name.to_string(),
                level: level,
                faction: faction,
                ai: ai,
                moves: moves,

                max_hp: hp, hp: hp,
                max_sp: sp, sp: sp,
                attack: attack, defence: defence,
                agility: agility, luck: luck,

                profile_sprite: profile_sprite,
            }
        )
    }
}
