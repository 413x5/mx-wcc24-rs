#[allow(unused_imports)]
use multiversx_sc::imports::*;


#[multiversx_sc::module]
pub trait AdminModule:
    crate::storage::StorageModule +
    crate::common::CommonModule +
    game_common_module::GameCommonModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

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

    /// Set game arena contract address
    #[only_owner]
    #[endpoint(setGameArenaContractAddress)]
    fn set_game_arena_contract_address(&self, address: ManagedAddress) {
        self.game_arena_contract_address().set(address);
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

    // For troubleshooting

    /// Clear all deposits
    #[only_owner]
    #[endpoint(clearDeposits)]
    fn clear_deposits(&self, address: ManagedAddress) {
        self.get_deposits(&address).clear();
    }

    /// Clear deposit
    #[only_owner]
    #[endpoint(clearDeposit)]
    fn clear_deposit(&self, user: ManagedAddress, token_id: TokenIdentifier, token_nonce: u64) {
        self.remove_deposit(&user, &token_id, token_nonce);
    }

    /// Set deposit balance
    #[only_owner]
    #[endpoint(setDepositBalance)]
    fn set_deposit_balance(&self, user: ManagedAddress, token_id: TokenIdentifier, token_nonce: u64, new_balance: BigUint) {
        self.update_deposit_balance(&user, &token_id, token_nonce, &new_balance);
    }


}