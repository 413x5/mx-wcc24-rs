use multiversx_sc::imports::*;

use crate::constants::*;
use crate::data::*;


#[multiversx_sc::module]
pub trait CommonModule: 
    crate::storage::StorageModule +
    game_common_module::GameCommonModule
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

    /// Get the deposit by token id
    fn get_deposit_by_token_id(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64) -> Option<DepositInfo<Self::Api>> {
        let deposits = self.get_deposits(user);
        for deposit in deposits.iter() {
            if deposit.token_id == *token_id && deposit.token_nonce == token_nonce {
                return Some(deposit);
            }
        }
        None
    }

    /// Get the deposit by token ticker
    fn get_deposit_by_token_ticker(&self, user: &ManagedAddress, token_ticker: &str) -> Option<DepositInfo<Self::Api>> {
        let deposits = self.get_deposits(user);
        let token_ticker_buffer = ManagedBuffer::from(token_ticker);
        for deposit in deposits.iter() {
            if self.is_required_token(&deposit.token_id, &token_ticker_buffer) {
                return Some(deposit);
            }
        }
        None
    }

    /// Increase the deposit balance
    fn increase_deposit_balance(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, amount: BigUint) -> bool {
        let mut deposits = self.get_deposits(user);
        for (index, deposit) in deposits.iter().enumerate() {
            if deposit.token_id == *token_id && deposit.token_nonce == token_nonce {
                let new_deposit = DepositInfo {
                    token_id: token_id.clone(),
                    token_nonce: deposit.token_nonce,
                    balance: deposit.balance + amount,
                };
                deposits.swap_remove(index + 1); // VecMapper index start at 1 not 0
                deposits.push(&new_deposit);
                return true;
            }
        }
        false
    }

    /// Increase the deposit balance u64
    fn increase_deposit_balance_u64(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, amount: u64) -> bool {
        return self.increase_deposit_balance(user, token_id, token_nonce, BigUint::from(amount));
    }

    /// Decrease the deposit balance
    fn decrease_deposit_balance(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, amount: &BigUint) -> bool {
        let mut deposits = self.get_deposits(&user);
        for (index, deposit) in deposits.iter().enumerate() {
            if deposit.token_id == *token_id && deposit.token_nonce == token_nonce {
                let new_deposit = DepositInfo {
                    token_id: token_id.clone(),
                    token_nonce: deposit.token_nonce,
                    balance: deposit.balance - amount,
                };
                deposits.swap_remove(index + 1); // VecMapper index start at 1 not 0
                deposits.push(&new_deposit);
                return true;
            }
        }
        false
    }

    /// Decrease the deposit balance u64
    fn decrease_deposit_balance_u64(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, amount: u64) -> bool {
        return self.decrease_deposit_balance(user, token_id, token_nonce, &BigUint::from(amount));
    }

    /// Add a deposit
    fn add_deposit(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, amount: &BigUint) {
        let mut deposits = self.get_deposits(&user);
        let new_deposit = DepositInfo {
            token_id: token_id.clone(),
            token_nonce,
            balance: amount.clone(),
        };
        deposits.push(&new_deposit);
    }

    /// Add or increase deposit balance
    fn add_or_increase_deposit_balance(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, amount: &BigUint) {
        if !self.increase_deposit_balance(user, token_id, token_nonce, amount.clone()) {
            self.add_deposit(user, token_id, token_nonce, amount);
        }
    }

    /// Add a NFT deposit
    fn add_nft_deposit(&self, user: &ManagedAddress, token_id: &TokenIdentifier, nft_nonce: u64) {
        let mut deposits = self.get_deposits(user);
        let new_deposit = DepositInfo {
            token_id: token_id.clone(),
            token_nonce: nft_nonce,
            balance: BigUint::from(1u64),
        };
        deposits.push(&new_deposit);
    }

    /// Remove a NFT deposit
    fn remove_deposit(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64) -> bool {
        let mut deposits = self.get_deposits(user);
        for (index, deposit) in deposits.iter().enumerate() {
            if deposit.token_id == *token_id && deposit.token_nonce == token_nonce {
                deposits.swap_remove(index + 1); // VecMapper index start at 1 not 0
                return true;
            }
        }
        false
    }

    /// Update the deposit balance
    fn update_deposit_balance(&self, user: &ManagedAddress, token_id: &TokenIdentifier, token_nonce: u64, balance: &BigUint) {
        let mut deposits = self.get_deposits(user);
        let mut existing_deposit = false;
        for (index, deposit) in deposits.iter().enumerate() {
            if deposit.token_id == *token_id && deposit.token_nonce == token_nonce {
                existing_deposit = true;
                let new_deposit = DepositInfo {
                    token_id: token_id.clone(),
                    token_nonce: deposit.token_nonce,
                    balance: balance.clone(),
                };
                deposits.swap_remove(index + 1); // VecMapper index start at 1 not 0
                deposits.push(&new_deposit);
                break;
            }
        }

        if !existing_deposit {
            let new_deposit = DepositInfo {
                token_id: token_id.clone(),
                token_nonce,
                balance: balance.clone(),
            };
            deposits.push(&new_deposit);
        }
    }

}