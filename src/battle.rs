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

    pub fn set_hp_cost(mut self, hp: u16) -> Self {
        self.hpc = Some(hp);
        self
    }

    pub fn set_sp_cost(mut self, sp: u16) -> Self {
        self.spc = Some(sp);
        self
    }

    pub fn set_power(mut self, power: u16) -> Self {
        self.power = Some(power);
        self
    }
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
    pub fn new(
        owner: Entity, 
        level: u8, 
        max_hp: u32, max_sp: u32,
        attack: u32, defence: u32,
        agility: u32, luck: u32) -> FighterComponent {
            FighterComponent {
                owner: owner,
                level: level,

                max_hp: max_hp, hp: max_hp,
                max_sp: max_sp, sp: max_sp,

                attack: attack, defence: defence,
                agility: agility, luck: luck,
                moves: vec![]
            }
    }

    pub fn add_move(mut self, m: &Rc<Move>) -> Self {
        self.moves.push(Rc::clone(m));
        self
    }
}
