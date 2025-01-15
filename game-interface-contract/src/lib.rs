#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod constants;
pub mod data;
pub mod admin;
pub mod storage;
pub mod common;
pub mod game_characters;
pub mod game_resources;
pub mod game_tools;
pub mod game_arena;

use constants::*;


#[multiversx_sc::contract]
pub trait GameInterfaceContract: 
    admin::AdminModule + 
    storage::StorageModule + 
    common::CommonModule +
    game_characters::CharactersModule +
    game_resources::ResourcesModule +
    game_tools::ToolsModule +
    game_arena::GameArenaModule +
    game_common_module::GameCommonModule
    
{
    /// Endpoint to deposit tokens in the game interface contract
    #[payable]
    #[endpoint(deposit)]
    fn deposit(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), ERR_NO_ESDT_TOKENS_RECEIVED);
        
        // Add the received tokens to the user's deposits
        let user = self.blockchain().get_caller();
    
        for payment in payments.iter() {
            if payment.token_type() == EsdtTokenType::Fungible {
                self.add_or_increase_deposit_balance(&user, &payment.token_identifier, 0, &payment.amount);
            } 
            else 
            if payment.token_type() == EsdtTokenType::NonFungible {
                if self.is_required_token(&payment.token_identifier, &self.characters_collection_id().get().into_managed_buffer()) ||
                    self.is_required_token(&payment.token_identifier, &self.tools_collection_id().get().into_managed_buffer()) {
                    self.add_nft_deposit(&user, &payment.token_identifier, payment.token_nonce);
                } else {
                    sc_panic!("Received NFT {} is not valid. Send only Character or Tool NFTs.", payment.token_identifier);
                }
            }
            else {
                sc_panic!("Received token {} is not valid. Send only Fungible or Character and Tool NFTs.", payment.token_identifier);
            }
        }
    }

}
