#[allow(unused_imports)]
use multiversx_sc::imports::*;


/// Character object to hold character attributes
pub struct Character {
    pub rank: u8,
    pub attack: u8,
    pub defence: u8,
}

impl Character {
    pub fn new_citizen() -> Self {
        Self {
            rank: 0,
            attack: 0,
            defence: 0,
        }
    }

    pub fn new_soldier() -> Self {
        Self {
            rank: 1,
            attack: 0,
            defence: 0,
        }
    }

    pub fn is_citizen(&self) -> bool {
        self.rank == 0
    }

    pub fn is_soldier(&self) -> bool {
        self.rank > 0
    }

    pub fn upgrade_soldier(soldier: Character, add_attack: u8, add_defence: u8) -> Self {
        Self {
            rank: 1,
            attack: soldier.attack + add_attack,
            defence: soldier.defence + add_defence,
        }
    }
}