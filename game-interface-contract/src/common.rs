use multiversx_sc::imports::*;

use crate::constants::*;

#[multiversx_sc::module]
pub trait CommonModule: 
    crate::storage::StorageModule
{

    /// Require the character contract address is set
    fn require_character_contract_address(&self) {
        require!(!self.character_contract_address().is_empty(), ERR_CHARACTER_CONTRACT_ADDRESS_NOT_SET);
    }

    /// Require the resource transform contract address is set
    fn require_resource_transform_contract_address(&self) {
        require!(!self.resource_transform_contract_address().is_empty(), ERR_RESOURCE_TRANSFORM_CONTRACT_ADDRESS_NOT_SET);
    }

    /// Require the tools contract address is set
    fn require_tools_contract_address(&self) {
        require!(!self.tools_contract_address().is_empty(), ERR_TOOLS_CONTRACT_ADDRESS_NOT_SET);
    }

    /// Require the characters collection is set
    fn require_characters_collection_id(&self) {
        require!(!self.characters_collection_id().is_empty(), ERR_CHARACTER_COLLECTION_NOT_SET);
    }

    /// Require the tools collection is set
    fn require_tools_collection_id(&self) {
        require!(!self.tools_collection_id().is_empty(), ERR_TOOLS_COLLECTION_NOT_SET);
    }

    /// Require the game arena contract address is set
    fn require_game_arena_contract_address(&self) {
        require!(!self.game_arena_contract_address().is_empty(), ERR_GAME_ARENA_CONTRACT_ADDRESS_NOT_SET);
    }

}