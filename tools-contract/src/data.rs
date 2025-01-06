#[allow(unused_imports)]
use multiversx_sc::imports::*;


/// Tool object to hold tool attributes
pub struct Tool {
    pub tool_type: u8,
    pub attack: u8,
    pub defence: u8,
}

impl Tool {
    pub fn new_shield() -> Self {
        Self {
            tool_type: 1,
            attack: 0,
            defence: 1,
        }
    }

    pub fn new_sword() -> Self {
        Self {
            tool_type: 2,
            attack: 1,
            defence: 0,
        }
    }

    pub fn is_shield(&self) -> bool {
        self.tool_type == 1
    }

    pub fn is_sword(&self) -> bool {
        self.tool_type == 2
    }

}