use multiversx_sc::imports::*;

use crate::constants::*;
use crate::data::*;

#[multiversx_sc::module]
pub trait ResourcesModule:
    crate::common::CommonModule +
    crate::storage::StorageModule
{
    /// Calls the resource transform contract to create ORE tokens
    #[endpoint(createOre)]
    fn create_ore(&self, ore_units: u64) {
        self.require_resource_transform_contract_address();

        let user = self.blockchain().get_caller();
        let deposits = self.get_deposits().get(&user).unwrap_or_default();

        // Find stone deposit
        let find_stone_deposit = deposits.iter().find(|deposit| self.is_required_token(&deposit.token, STONE_TOKEN_TICKER));
        match find_stone_deposit {
            None => require!(false, "No stone deposited. Need at least {}.", STONE_AMMOUNT_FOR_ORE),
            Some(stone_deposit) => {
                // Calculate the amount of stone needed
                let stone_amount = BigUint::from(STONE_AMMOUNT_FOR_ORE * ore_units);
                require!(stone_deposit.balance >= stone_amount, "Not enough stone deposited. Need at least {}.", stone_amount);

                // Create the payment for the amount of stone to the resource transform contract
                let stone_token_payment = EsdtTokenPayment::new(stone_deposit.token.clone(), 0u64, stone_amount.clone());

                // Call the resource transform contract
                self.tx()
                    .to(self.resource_transform_contract_address().get())
                    .with_esdt_transfer(stone_token_payment)
                    .raw_call(RESOURCE_TRANSFORM_CONTRACT_CREATE_ORE_ENDPOINT_NAME)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().create_ore_callback(&user, stone_deposit.token, stone_amount))
                    .async_call_and_exit();
            }
        }
    }

    /// Callback for the create ORE transaction to update deposit amounts
    #[callback]
    fn create_ore_callback(&self, 
        user: &ManagedAddress, 
        stone_token: TokenIdentifier, 
        stone_amount: BigUint,  
        #[call_result] result: ManagedAsyncCallResult<()>) {
        
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                // Get user deposits
                let mut deposits = self.get_deposits().get(&user).unwrap_or_default();

                // Find and update spent stone deposit
                let find_stone_deposit = deposits.iter().find(|deposit| deposit.token == stone_token);
                match find_stone_deposit {
                    None => {},
                    Some(mut stone_deposit) => {
                        stone_deposit.balance -= stone_amount.clone();
                    }
                }                        

                // Get back token transfered and update user deposits
                let back_transfers = self.blockchain().get_back_transfers();
                for payment in back_transfers.esdt_payments.iter() {
                    // These should be the ORE tokens
                    let received_token = payment.token_identifier;
                    let received_amount = payment.amount;

                    // Update user deposits
                    let find_deposit = deposits.iter().find(|deposit| deposit.token == received_token);
                    match find_deposit {
                        None => {
                            // If no deposit found, create a new one
                            let new_deposit = DepositInfo {
                                token: received_token.clone(),
                                balance: received_amount,
                            };
                            // Add new deposit
                            deposits.push(new_deposit);
                        },
                        Some(mut deposit) => {
                            // If deposit found, update balance
                            deposit.balance += received_amount;       
                        }
                    }
                }
                // Update user deposits in storage
                self.get_deposits().insert(user.clone(), deposits);
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the call fails deposits remain the same
            },
        }
    }

}