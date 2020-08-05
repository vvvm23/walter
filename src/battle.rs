use std::rc::Rc;
use crate::ecs::{Entity};

#[derive(Debug)]
pub struct Move {
    pub name: String,
    pub description: String,
    pub hpc: Option<u16>, pub spc: Option<u16>,
    pub power: Option<u16>,
}

impl Move {
    pub fn new(name: &str, description: &str) -> Self {
        Self { 
            name: name.to_string(),
            description: description.to_string(),
            hpc: None, spc: None,
            power: None
        }
    }

    pub fn set_hp_cost(mut self, hp: u16) -> Self { self.hpc = Some(hp); self }
    pub fn set_sp_cost(mut self, sp: u16) -> Self { self.spc = Some(sp); self }
    pub fn set_power(mut self, power: u16) -> Self { self.power = Some(power); self }
}

#[derive(Debug)]
pub struct FighterComponent {
    pub owner: Entity,                   // Entity associated with this component
    pub level: u8,                                  // Level of the Entity
    pub max_hp: u32,        pub hp: u32,            // Health points of the Entity
    pub max_sp: u32,        pub sp: u32,            // Points for special moves of the Entity

    // Various stats of the Entity
    pub attack: u32,        pub defence: u32,
    pub agility: u32,       pub luck: u32,

    pub moves: Vec<Rc<Move>>                        // Vector of Moves available to the Entity
}

impl FighterComponent {
    pub fn new(owner: Entity) -> Self {
        FighterComponent {
            owner: owner,
            level: 0,

            max_hp: 0, hp: 0,
            max_sp: 0, sp: 0,

            attack: 0, defence: 0,
            agility: 0, luck: 0,
            moves: vec![]
        }
    }

    // Builder patterns, not to be confused with setters/getters
    pub fn add_move(mut self, m: &Rc<Move>) -> Self { self.moves.push(Rc::clone(m)); self }
    pub fn set_level(mut self, level: u8) -> Self { self.level = level; self }
    pub fn set_max_hp(mut self, hp: u32) -> Self { self.max_hp = hp; self.hp = hp; self }
    pub fn set_max_sp(mut self, sp: u32) -> Self { self.max_sp = sp; self.sp = sp; self }
    pub fn set_attack(mut self, atk: u32) -> Self { self.attack = atk; self }
    pub fn set_defence(mut self, def: u32) -> Self { self.defence = def; self }
    pub fn set_agility(mut self, agi: u32) -> Self { self.agility = agi; self }
    pub fn set_luck(mut self, luk: u32) -> Self { self.luck = luk; self }
}
