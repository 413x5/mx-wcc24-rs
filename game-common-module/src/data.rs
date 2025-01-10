#[allow(unused_imports)]
use multiversx_sc::imports::*;


/// Character object to hold character attributes
pub struct Character {
    pub rank: u8,
    pub attack: u8,
    pub defence: u8,
}

impl Character {
    /// Create a new citizen character
    pub fn new_citizen() -> Self {
        Self {
            rank: 0,
            attack: 0,
            defence: 0,
        }
    }
    /// Create a new soldier character
    pub fn new_soldier() -> Self {
        Self {
            rank: 1,
            attack: 0,
            defence: 0,
        }
    }
    /// Check if the character is a citizen
    pub fn is_citizen(&self) -> bool {
        self.rank == 0
    }

    /// Check if the character is a soldier
    pub fn is_soldier(&self) -> bool {
        self.rank == 1
    }

    /// Upgrade the character with the given tool
    pub fn upgrade(&mut self, tool: &Tool) {
        self.attack += tool.attack;
        self.defence += tool.defence;
    }
}

/// Tool object to hold tool attributes
pub struct Tool {
    pub tool_type: u8,
    pub attack: u8,
    pub defence: u8,
}

impl Tool {
    /// Create a new shield tool
    pub fn new_shield() -> Self {
        Self {
            tool_type: 1,
            attack: 0,
            defence: 1,
        }
    }

    /// Create a new sword tool
    pub fn new_sword() -> Self {
        Self {
            tool_type: 2,
            attack: 1,
            defence: 0,
        }
    }

    /// Check if the tool is a shield
    pub fn is_shield(&self) -> bool {
        self.tool_type == 1
    }

    /// Check if the tool is a sword
    pub fn is_sword(&self) -> bool {
        self.tool_type == 2
    }

}