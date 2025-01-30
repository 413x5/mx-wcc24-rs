use multiversx_sc::imports::*;

use crate::data::*;

#[multiversx_sc::module]
pub trait StorageModule {

    /// Users deposits in the contract
    #[view(getDeposits)]
    #[storage_mapper("deposits")]
    fn get_deposits(&self, user: &ManagedAddress) -> VecMapper<DepositInfo<Self::Api>>;

    /// Address of the character contract
    #[view(characterContractAddress)]
    #[storage_mapper("character_contract_address")]
    fn character_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the resource transform contract
    #[view(resourceTransformContractAddress)]
    #[storage_mapper("resource_transform_contract_address")]
    fn resource_transform_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the tools contract
    #[view(toolsContractAddress)]
    #[storage_mapper("tools_contract_address")]
    fn tools_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the game arena contract
    #[view(gameArenaContractAddress)]
    #[storage_mapper("game_arena_contract_address")]
    fn game_arena_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the Wood mint contract
    #[view(woodMintContractAddress)]
    #[storage_mapper("wood_mint_contract_address")]
    fn wood_mint_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the Food mint contract
    #[view(foodMintContractAddress)]
    #[storage_mapper("food_mint_contract_address")]
    fn food_mint_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the Stone mint contract
    #[view(stoneMintContractAddress)]
    #[storage_mapper("stone_mint_contract_address")]
    fn stone_mint_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the Gold mint contract
    #[view(goldMintContractAddress)]
    #[storage_mapper("gold_mint_contract_address")]
    fn gold_mint_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Id of the characters collection
    #[view(charactersCollectionId)]
    #[storage_mapper("characters_collection_id")]
    fn characters_collection_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /// Id of the tools collection
    #[view(toolsCollectionId)]
    #[storage_mapper("tools_collection_id")]
    fn tools_collection_id(&self) -> SingleValueMapper<TokenIdentifier>;

}