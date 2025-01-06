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

use constants::*;
use data::*;


#[multiversx_sc::contract]
pub trait GameInterfaceContract: 
    admin::AdminModule + 
    storage::StorageModule + 
    common::CommonModule +
    game_characters::CharactersModule +
    game_resources::ResourcesModule +
    game_tools::ToolsModule
{
    /// Endpoint to deposit resources in the game contract
    #[payable("*")]
    #[endpoint(depositResources)]
    fn deposit_resources(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), ERR_NO_ESDT_TOKENS_RECEIVED);
        
        // Add the received tokens to the user's deposits
        let user = self.blockchain().get_caller();
        let mut user_deposits = self.get_deposits().get(&user).unwrap_or_default();
    
        for payment in payments.iter() {
            let existing_deposit = user_deposits.iter().find(|deposit| deposit.token == payment.token_identifier);
            match existing_deposit {
                Some(mut deposit) => {
                    deposit.balance += payment.amount.clone();
                }
                None => {
                    let new_deposit = DepositInfo {
                        token: payment.token_identifier,
                        balance: payment.amount,
                    };
                    user_deposits.push(new_deposit);
                }
            }
        }
        // Update the user's deposits
        self.get_deposits().insert(user, user_deposits);
    }

}
