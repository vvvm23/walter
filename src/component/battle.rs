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

impl Move {
    fn new(
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
    fn new(
        display_name: &str,
        level: u8,
        faction: Faction,
        hp: u16, sp: u16,
        attack: u16, defence: u16,
        agility: u16, luck: u16,
    ) -> Component {
        Component::FighterComponent (
            FighterComponent {
                display_name: display_name.to_string(),
                level: level,
                faction: faction,
                moves: Vec::new(),

                max_hp: hp, hp: hp,
                max_sp: sp, sp: sp,
                attack: attack, defence: defence,
                agility: agility, luck: luck,
            }
        )
    }

    /// Builder pattern to add move to Fighter
    fn add_move(mut self, m: Arc<Move>) -> FighterComponent {
        self.moves.push(m);
        self
    }
}
