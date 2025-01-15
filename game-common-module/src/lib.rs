#![no_std]

pub mod constants;
pub mod data;
pub mod nft_attributes;

#[multiversx_sc::module]
pub trait GameCommonModule {

    /// Check if a token is a required token
    fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &ManagedBuffer) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == *required_token_ticker
    }

}