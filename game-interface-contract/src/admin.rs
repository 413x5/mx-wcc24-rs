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
    

    // #[only_owner]
    // #[endpoint(setDepositBalance)]
    // fn set_deposit_balance(&self, token: TokenIdentifier, new_balance: BigUint) {
    //     let user = self.blockchain().get_caller();
    //     let user_deposits = self.get_deposits().get(&user).unwrap_or_default();
    //     let mut new_deposits = ManagedVec::new();
    //     let mut found = false;

    //     for deposit in user_deposits.iter() {
    //         if deposit.token == token {
    //             new_deposits.push(DepositInfo {
    //                 token: token.clone(),
    //                 balance: new_balance.clone(),
    //             });
    //             found = true;
    //         } else {
    //             new_deposits.push(deposit);
    //         }
    //     }
        
    //     require!(found, "Deposit not found for token {}", token);
    //     self.get_deposits().insert(user, new_deposits);
    // }

    }