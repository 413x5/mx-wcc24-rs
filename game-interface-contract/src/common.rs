use multiversx_sc::imports::*;

use crate::constants::*;

#[multiversx_sc::module]
pub trait CommonModule: 
    crate::storage::StorageModule
{
    /// Check if a token is a required token
    fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == ManagedBuffer::from(required_token_ticker.as_bytes())
    }

    /// Check if a token is a required token and terminates if not
    fn require_expected_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) {
        let expected_token = ManagedBuffer::from(required_token_ticker.as_bytes());
        require!(self.is_required_token(check_token_id, required_token_ticker), "Invalid token {}. Expected {}.", check_token_id, expected_token);
    }

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

}