#[allow(unused_imports)]
use multiversx_sc::imports::*;



#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule

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

}