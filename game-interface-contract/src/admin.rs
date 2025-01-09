#[allow(unused_imports)]
use multiversx_sc::imports::*;

use crate::data::*;

#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule

{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Clear deposits
    #[only_owner]
    #[endpoint(clearDeposits)]
    fn clear_deposits(&self) {
        self.get_deposits().clear();
    }

    /// Set deposit balance
    #[only_owner]
    #[endpoint(setDepositBalance)]
    fn set_deposit_balance(&self, token_id: TokenIdentifier, token_nonce: u64, new_balance: BigUint) {
        let user = self.blockchain().get_caller();
        let mut user_deposits = self.get_deposits().get(&user).unwrap_or_default();
        let mut found = false;

        let mut i = 0;
        while i < user_deposits.len() {
            if user_deposits.get(i).token_id == token_id && user_deposits.get(i).token_nonce == token_nonce {
                user_deposits.get_mut(i).balance = new_balance.clone();
                found = true;
                break;
            }
            i += 1;
        }

        if !found {
            user_deposits.push(DepositInfo {
                token_id: token_id.clone(),
                token_nonce,
                balance: new_balance.clone(),
            });
        }
        self.get_deposits().insert(user, user_deposits);
    }

    /// Set character contract address
    #[only_owner]
    #[endpoint(setCharacterContractAddress)]
    fn set_character_contract_address(&self, address: ManagedAddress) {
        self.character_contract_address().set(address);
    }

    /// Set resource transform contract address
    #[only_owner]
    #[endpoint(setResourceTransformContractAddress)]
    fn set_resource_transform_contract_address(&self, address: ManagedAddress) {
        self.resource_transform_contract_address().set(address);
    }

    /// Set tools contract address
    #[only_owner]
    #[endpoint(setToolsContractAddress)]
    fn set_tools_contract_address(&self, address: ManagedAddress) {
        self.tools_contract_address().set(address);
    }

    /// Set wood mint contract address
    #[only_owner]
    #[endpoint(setWoodMintContractAddress)]
    fn set_wood_mint_contract_address(&self, address: ManagedAddress) {
        self.wood_mint_contract_address().set(address);
    }

    /// Set food mint contract address
    #[only_owner]
    #[endpoint(setFoodMintContractAddress)]
    fn set_food_mint_contract_address(&self, address: ManagedAddress) {
        self.food_mint_contract_address().set(address);
    }

    /// Set stone mint contract address
    #[only_owner]
    #[endpoint(setStoneMintContractAddress)]
    fn set_stone_mint_contract_address(&self, address: ManagedAddress) {
        self.stone_mint_contract_address().set(address);
    }

    /// Set gold mint contract address
    #[only_owner]
    #[endpoint(setGoldMintContractAddress)]
    fn set_gold_mint_contract_address(&self, address: ManagedAddress) {
        self.gold_mint_contract_address().set(address);
    }


    #[only_owner]
    #[endpoint(setCharactersCollectionId)]
    fn set_characters_collection(&self, collection_id: TokenIdentifier) {
        self.characters_collection_id().set(collection_id);
    }

    #[only_owner]
    #[endpoint(setToolsCollectionId)]
    fn set_tools_collection(&self, collection_id: TokenIdentifier) {
        self.tools_collection_id().set(collection_id);
    }

}