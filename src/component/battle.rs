use crate::component::Component;
use std::sync::{Arc, RwLock};

pub enum Faction {
    Ally,
    Enemy,
}

pub enum AI {
    Random,
}

pub enum AOETarget {
    All,
    Ally,
    Enemy,
}

pub enum SingleTarget {
    Ally,
    Enemy,
    User,
}

pub enum MoveTarget {
    Single(SingleTarget),
    AOE(AOETarget),
}

pub struct Move {
    name: String,
    description: String,
    use_message: String,

    hp_cost: u16, sp_cost: u16,
    power: Option<u16>, accuracy: f32,

    target: MoveTarget,
}

pub struct FighterComponent {
    display_name: String,
    level: u8,
    faction: Faction,
    moves: Vec<Arc<Move>>,

    max_hp: u16,    hp: u16,
    max_sp: u16,    sp: u16,

    attack: u16,    defence: u16,
    agility: u16,   luck: u16,
}

impl FighterComponent {

}
