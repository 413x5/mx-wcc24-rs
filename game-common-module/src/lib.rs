#![no_std]

use multiversx_sc::imports::*;

pub mod constants;
pub mod data;
pub mod nft_attributes;

#[multiversx_sc::module]
pub trait GameCommonModule {

    /// Check if a token is a required token
    fn is_required_token_str(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == ManagedBuffer::from(required_token_ticker.as_bytes())
    }

    /// Check if a token is a required token
    fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &ManagedBuffer) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == *required_token_ticker
    }

    /// Check if a token is a required NFT
    fn is_required_nft(&self, check_token_id: &TokenIdentifier, check_token_nonce: u64, required_token_ticker: &ManagedBuffer, required_token_nonce: u64) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == *required_token_ticker && check_token_nonce == required_token_nonce
    }

    /// Check if a token is a required token and terminates if not
    fn require_expected_token_str(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) {
        let expected_token = ManagedBuffer::from(required_token_ticker.as_bytes());
        require!(self.is_required_token_str(check_token_id, required_token_ticker), "Invalid token {}. Expected {}.", check_token_id, expected_token);
    }

    /// Check if a token is a required token and terminates if not
    fn require_expected_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &ManagedBuffer) {
        require!(self.is_required_token(check_token_id, required_token_ticker), "Invalid token {}. Expected {}.", check_token_id, required_token_ticker);
    }

}