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
use data::*;



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
    #[endpoint(depositTokens)]
    fn deposit_tokens(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), ERR_NO_ESDT_TOKENS_RECEIVED);
        
        // Add the received tokens to the user's deposits
        let user = self.blockchain().get_caller();
        let mut user_deposits = self.get_deposits().get(&user).unwrap_or_default();
    
        for payment in payments.iter() {
            let mut found = false;
            let mut i = 0;
            while i < user_deposits.len() {
                if user_deposits.get(i).token_id == payment.token_identifier {
                    user_deposits.get_mut(i).balance += &payment.amount;
                    found = true;
                    break;
                }
                i += 1;
            }
            
            if !found {
                let new_deposit = DepositInfo {
                    token_id: payment.token_identifier.clone(),
                    token_nonce: payment.token_nonce,
                    balance: payment.amount.clone(),
                };
                user_deposits.push(new_deposit);
            }
        }
        // Update the user's deposits
        self.get_deposits().insert(user, user_deposits);
    }

    /// Endpoint to deposit character NFT in the game interface contract
    #[payable]
    #[endpoint(depositCharacterNft)]
    fn deposit_character_nft(&self) {

        self.require_characters_collection_id();

        let transfer = self.call_value().single_esdt();

        require!(transfer.token_type() == EsdtTokenType::NonFungible, "Only one NFT is accepted.");

        let token_id = &transfer.token_identifier;
        let token_nonce = transfer.token_nonce;

        self.require_expected_token(token_id, &self.characters_collection_id().get().into_managed_buffer());

        let user = self.blockchain().get_caller();

        let mut user_deposits = self.get_deposits().get(&user).unwrap_or_default();

        let new_deposit = DepositInfo {
            token_id: token_id.clone(),
            token_nonce,
            balance: BigUint::from(1u64),
        };
        user_deposits.push(new_deposit);

        self.get_deposits().insert(user, user_deposits);
    }

    /// Endpoint to deposit tool NFT in the game interface contract
    #[payable]
    #[endpoint(depositToolNft)]
    fn deposit_tool_nft(&self) {

        self.require_tools_collection_id();

        let transfer = self.call_value().single_esdt();
        require!(transfer.token_type() == EsdtTokenType::NonFungible, "Only one NFT is accepted.");

        let token_id = &transfer.token_identifier;
        let token_nonce = transfer.token_nonce;

        self.require_expected_token(token_id, &self.tools_collection_id().get().into_managed_buffer());

        let user = self.blockchain().get_caller();

        let mut user_deposits = self.get_deposits().get(&user).unwrap_or_default();

        let new_deposit = DepositInfo {
            token_id: token_id.clone(),
            token_nonce,
            balance: BigUint::from(1u64),
        };
        user_deposits.push(new_deposit);

        self.get_deposits().insert(user, user_deposits);
    }


}
