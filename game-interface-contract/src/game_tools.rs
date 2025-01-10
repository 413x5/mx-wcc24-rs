use multiversx_sc::imports::*;

use game_common_module::constants::*;

#[multiversx_sc::module]
pub trait ToolsModule:
    crate::common::CommonModule +
    crate::storage::StorageModule +
    game_common_module::GameCommonModule
{
    // Shield endpoints

    /// Calls the tools contract to mint a shield using resources from user's deposits
    #[endpoint(mintShield)]
    fn mint_shield(&self) {
        self.require_tools_contract_address();

        // Get the ore and gold required
        let ore_quantity = BigUint::from(MINT_SHIELD_ORE_QUANTITY);

        // Get the user deposits
        let user = self.blockchain().get_caller();
        let deposits = self.get_deposits().get(&user).unwrap_or_default();  

        // Find ore and gold deposits
        let find_ore_deposit = &deposits.iter().find(|deposit| self.is_required_token_str(&deposit.token_id, ORE_TICKER));

        match find_ore_deposit {
            None => require!(false, "No ore deposited. Need at least {}.", ore_quantity),
            Some(ore_deposit) => 
            {   
                // Check deposit amounts
                require!(ore_deposit.balance >= ore_quantity, "Not enough ore deposited. Need at least {}.", ore_quantity);

                // Create the payment for the amount of ore to the character contract
                let ore_token_payment = EsdtTokenPayment::new(ore_deposit.token_id.clone(), 0u64, ore_quantity);

                // Call the tools contract to mint the citizen
                self.tx()
                    .to(self.tools_contract_address().get())
                    .with_esdt_transfer(ore_token_payment)
                    .raw_call(TOOLS_CONTRACT_MINT_SHIELD_ENDPOINT_NAME)
                    // Set the receiver address for the NFT to the user address
                    .argument(&user)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().mint_shield_callback(&user, ore_deposit.token_id.clone()))
                    .async_call_and_exit();
            }
        }
    }

    /// Callback for the mint shield transaction to update deposit amounts
    #[callback]
    fn mint_shield_callback(
        &self,
        user: &ManagedAddress,
        ore_token: TokenIdentifier,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                // Get user deposits and update spent ore
                let mut deposits = self.get_deposits().get(&user).unwrap_or_default();
                let mut i = 0;
                while i < deposits.len() {
                    if deposits.get(i).token_id == ore_token {
                        deposits.get_mut(i).balance -= MINT_SHIELD_ORE_QUANTITY;
                        break;
                    }
                    i += 1;
                }

                // Update deposits to storage
                self.get_deposits().insert(user.clone(), deposits);
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the transaction fails, deposits are not updated
            },
        }
    }

    /// Calls the tools contract to claim a shield after the minting period
    #[endpoint(claimShield)]
    fn claim_shield(&self) {
        self.require_tools_contract_address();

        let user = self.blockchain().get_caller();
        // Call the tools contract to mint the shield and transfer the NFT to the user
        self.tx()
            .to(self.tools_contract_address().get())
            .raw_call(TOOLS_CONTRACT_CLAIM_SHIELD_ENDPOINT_NAME)
            // Claim the NFT to the user address
            .argument(&user)
            .async_call_and_exit();
    }


    // Sword endpoints

    /// Calls the tools contract to mint a sword using resources from user's deposits
    #[endpoint(mintSword)]
    fn mint_sword(&self) {
        self.require_tools_contract_address();

        // Get the ore and gold required
        let ore_quantity = BigUint::from(MINT_SWORD_ORE_QUANTITY);
        let gold_quantity = BigUint::from(MINT_SWORD_GOLD_QUANTITY);

        // Get the user deposits
        let user = self.blockchain().get_caller();
        let deposits = self.get_deposits().get(&user).unwrap_or_default();  

        // Find ore and gold deposits
        let find_ore_deposit = &deposits.iter().find(|deposit| self.is_required_token_str(&deposit.token_id, ORE_TICKER));
        let find_gold_deposit = &deposits.iter().find(|deposit| self.is_required_token_str(&deposit.token_id, GOLD_TICKER));

        match (find_ore_deposit, find_gold_deposit) {
            (None, None) => require!(false, "No ore or gold deposited. Need at least {} and {}.", ore_quantity, gold_quantity),
            (None, Some(_)) => require!(false, "No ore deposited. Need at least {}.", ore_quantity),
            (Some(_), None) => require!(false, "No gold deposited. Need at least {}.", gold_quantity),
            (Some(ore_deposit), Some(gold_deposit)) => 
            {   
                // Check deposit amounts
                require!(ore_deposit.balance >= ore_quantity, "Not enough ore deposited. Need at least {}.", ore_quantity);
                require!(gold_deposit.balance >= gold_quantity, "Not enough gold deposited. Need at least {}.", gold_quantity);

                // Create the payment for the amount of ore and gold to the character contract
                let ore_token_payment = EsdtTokenPayment::new(ore_deposit.token_id.clone(), 0u64, ore_quantity.clone());
                let gold_token_payment = EsdtTokenPayment::new(gold_deposit.token_id.clone(), 0u64, gold_quantity.clone());

                let mut payments : ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
                payments.push(ore_token_payment);
                payments.push(gold_token_payment);

                // Call the tools contract to mint the citizen
                self.tx()
                    .to(self.tools_contract_address().get())
                    .with_multi_token_transfer(payments)
                    .raw_call(TOOLS_CONTRACT_MINT_SWORD_ENDPOINT_NAME)
                    // Set the receiver address for the NFT to the user address
                    .argument(&user)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().mint_sword_callback(&user, ore_deposit.token_id.clone(), gold_deposit.token_id.clone()))
                    .async_call_and_exit();
            },
        }
    }

    /// Callback for the mint sword transaction to update deposit amounts
    #[callback]
    fn mint_sword_callback(&self, 
        user: &ManagedAddress, 
        ore_token: TokenIdentifier, 
        gold_token: TokenIdentifier,
        #[call_result] result: ManagedAsyncCallResult<()>) {

        match result {
            ManagedAsyncCallResult::Ok(_) => {
                // Get user deposits and update spent food and wood
                let mut deposits = self.get_deposits().get(&user).unwrap_or_default();

                let mut i = 0;
                while i < deposits.len() {
                    if deposits.get(i).token_id == ore_token {
                        deposits.get_mut(i).balance -= MINT_SWORD_ORE_QUANTITY;
                    }
                    if deposits.get(i).token_id == gold_token {
                        deposits.get_mut(i).balance -= MINT_SWORD_GOLD_QUANTITY;
                    }
                    i += 1;
                }
                // Update deposits to storage
                self.get_deposits().insert(user.clone(), deposits);
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the transaction fails, deposits are not updated
            }
        }
    }

    /// Calls the tools contract to claim a sword after the minting period
    #[endpoint(claimSword)]
    fn claim_sword(&self) {
        self.require_tools_contract_address();

        let user = self.blockchain().get_caller();
        // Call the tools contract to mint the sword and transfer the NFT to the user
        self.tx()
            .to(self.tools_contract_address().get())
            .raw_call(TOOLS_CONTRACT_CLAIM_SWORD_ENDPOINT_NAME)
            // Claim the NFT to the user address
            .argument(&user)
            .async_call_and_exit();
    }


}