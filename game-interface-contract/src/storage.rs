use multiversx_sc::imports::*;

use crate::data::*;

#[multiversx_sc::module]
pub trait StorageModule {

    /// Users deposits in the contract
    #[view(getDeposits)]
    #[storage_mapper("deposits")]
    fn get_deposits(&self) -> MapMapper<ManagedAddress, ManagedVec<DepositInfo<Self::Api>>>;

    /// Address of the character contract
    #[view(characterContractAddress)]
    #[storage_mapper("character_contract_address")]
    fn character_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// Address of the resource transform contract
    #[view(resourceTransformContractAddress)]
    #[storage_mapper("resource_transform_contract_address")]
    fn resource_transform_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

}