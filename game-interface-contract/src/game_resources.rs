use multiversx_sc::imports::*;

use game_common_module::constants::*;
use crate::data::*;

#[multiversx_sc::module]
pub trait ResourcesModule:
    crate::common::CommonModule +
    crate::storage::StorageModule +
    game_common_module::GameCommonModule
{

    /// Calls the resource contracts to mint any available base resources
    #[endpoint(mintResources)]
    fn mint_resources(&self){

        // Mint any available wood resources
        if !self.wood_mint_contract_address().is_empty() {
            self.resource_contract_mint(self.wood_mint_contract_address().get());
        }

        // Mint any available food resources
        if !self.food_mint_contract_address().is_empty() {
            self.resource_contract_mint(self.food_mint_contract_address().get());
        }

        // Mint any available stone resources
        if !self.stone_mint_contract_address().is_empty() {
            self.resource_contract_mint(self.stone_mint_contract_address().get());
        }

        // Mint any available gold resources
        if !self.gold_mint_contract_address().is_empty() {
            self.resource_contract_mint(self.gold_mint_contract_address().get());
        }

    }

    /// Calls the resource contracts to claim any unclaimed base resources
    #[endpoint(claimResources)]
    fn claim_resources(&self) {

        // Send the resources to the calling user
        let user = self.blockchain().get_caller();

        // Claim any available wood resources
        if !self.wood_mint_contract_address().is_empty() {
            self.resource_contract_claim(self.wood_mint_contract_address().get(), &user);
        }

        // Claim any available food resources
        if !self.food_mint_contract_address().is_empty() {
            self.resource_contract_claim(self.food_mint_contract_address().get(), &user);
        }

        // Claim any available stone resources
        if !self.stone_mint_contract_address().is_empty() {
            self.resource_contract_claim(self.stone_mint_contract_address().get(), &user);
        }

        // Claim any available gold resources
        if !self.gold_mint_contract_address().is_empty() {
            self.resource_contract_claim(self.gold_mint_contract_address().get(), &user);
        }
    }

    /// Calls the resource contract to mint any available base resources
    fn resource_contract_mint(&self, resource_contract_address: ManagedAddress) {
        
        self.tx()
            .to(&resource_contract_address)
            .raw_call(RESOURCE_CONTRACT_MINT_RESOURCES_ENDPOINT_NAME)
            .sync_call();
    }

    /// Calls the resource contract to claim any available base resources
    fn resource_contract_claim(&self, resource_contract_address: ManagedAddress, user: &ManagedAddress) {

        self.tx()
            .to(&resource_contract_address)
            .raw_call(RESOURCE_CONTRACT_CLAIM_RESOURCES_ENDPOINT_NAME)
            .argument(&user)
            .sync_call();
    }

    /// Calls the resource transform contract to create ORE tokens
    #[endpoint(createOre)]
    fn create_ore(&self, ore_units: u64) {
        self.require_resource_transform_contract_address();

        let user = self.blockchain().get_caller();
        let deposits = self.get_deposits().get(&user).unwrap_or_default();

        // Find stone deposit
        let find_stone_deposit = deposits.iter().find(|deposit| self.is_required_token_str(&deposit.token_id, STONE_TICKER));
        match find_stone_deposit {
            None => require!(false, "No stone deposited. Need at least {}.", STONE_AMMOUNT_FOR_ORE),
            Some(stone_deposit) => {
                // Calculate the amount of stone needed
                let stone_amount = BigUint::from(STONE_AMMOUNT_FOR_ORE * ore_units);
                require!(stone_deposit.balance >= stone_amount, "Not enough stone deposited. Need at least {}.", stone_amount);

                // Create the payment for the amount of stone to the resource transform contract
                let stone_token_payment = EsdtTokenPayment::new(stone_deposit.token_id.clone(), 0u64, stone_amount.clone());

                // Call the resource transform contract
                self.tx()
                    .to(self.resource_transform_contract_address().get())
                    .with_esdt_transfer(stone_token_payment)
                    .raw_call(RESOURCE_TRANSFORM_CONTRACT_CREATE_ORE_ENDPOINT_NAME)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().create_ore_callback(&user, stone_deposit.token_id.clone(), stone_amount))
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
                // Get user deposits and update spent stone
                let mut deposits = self.get_deposits().get(&user).unwrap_or_default();
                let mut i = 0;
                while i < deposits.len() {
                    if deposits.get(i).token_id == stone_token {
                        deposits.get_mut(i).balance -= stone_amount;
                        break;
                    }
                    i += 1;
                }

                // Update deposits to storage
                self.get_deposits().insert(user.clone(), deposits);

                // Get back token transfered and update user deposits
                let back_transfers = self.blockchain().get_back_transfers();
                for payment in back_transfers.esdt_payments.iter() {
                    // These should be the ORE tokens
                    let received_token = &payment.token_identifier;
                    let received_amount = &payment.amount;

                    // Update user deposits
                    let mut deposits = self.get_deposits().get(&user).unwrap_or_default();
                    let mut i = 0;
                    let mut found = false;
                    while i < deposits.len() {
                        if deposits.get(i).token_id == *received_token {
                            deposits.get_mut(i).balance += received_amount;
                            found = true;
                            break;
                        }
                        i += 1;
                    }
                    // If the token is not in the user's deposits, add it
                    if !found {
                        deposits.push(DepositInfo {
                            token_id: received_token.clone(),
                            token_nonce: 0,
                            balance: received_amount.clone(),
                        });
                    }
                    // Update user deposits in storage
                    self.get_deposits().insert(user.clone(), deposits);
                }
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the call fails deposits remain the same
            },
        }
    }

}