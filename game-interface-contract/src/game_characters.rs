use multiversx_sc::imports::*;
use multiversx_sc::types::EsdtTokenPayment;

use crate::constants::*;


#[multiversx_sc::module]
pub trait CharactersModule: 
    crate::storage::StorageModule +
    crate::common::CommonModule
{

    /// Calls the character contract to mint a citizen using resources from user's deposits
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self) {
        self.require_character_contract_address();

        // Get the wood and food required
        let wood_quantity = BigUint::from(MINT_CITIZEN_WOOD_QUANTITY);
        let food_quantity = BigUint::from(MINT_CITIZEN_FOOD_QUANTITY);

        // Get the user deposits
        let user = self.blockchain().get_caller();
        let deposits = self.get_deposits().get(&user).unwrap_or_default();

        // Find wood and food deposits
        let find_wood_deposit = deposits.iter().find(|deposit| self.is_required_token(&deposit.token, WOOD_TOKEN_TICKER));
        let find_food_deposit = deposits.iter().find(|deposit| self.is_required_token(&deposit.token, FOOD_TOKEN_TICKER));

        match (find_wood_deposit, find_food_deposit) {
            (None, None) => require!(false, "No wood or food deposited. Need at least {} and {}.", wood_quantity, food_quantity),
            (None, Some(_)) => require!(false, "No wood deposited. Need at least {}.", wood_quantity),
            (Some(_), None) => require!(false, "No food deposited. Need at least {}.", food_quantity),
            (Some(wood_deposit), Some(food_deposit)) => 
            {   
                // Check deposit amounts
                require!(wood_deposit.balance >= wood_quantity, "Not enough wood deposited. Need at least {}.", wood_quantity);
                require!(food_deposit.balance >= food_quantity, "Not enough food deposited. Need at least {}.", food_quantity);

                // Create the payment for the amount of wood and food to the character contract
                let wood_token_payment = EsdtTokenPayment::new(wood_deposit.token.clone(), 0u64, wood_quantity.clone());
                let food_token_payment = EsdtTokenPayment::new(food_deposit.token.clone(), 0u64, food_quantity.clone());

                let mut payments : ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
                payments.push(wood_token_payment);
                payments.push(food_token_payment);

                // Call the character contract to mint the citizen
                self.tx()
                    .to(self.character_contract_address().get())
                    .with_multi_token_transfer(payments)
                    .raw_call(CHARACTER_CONTRACT_MINT_CITIZEN_ENDPOINT_NAME)
                    // Register the NFT mint to the user, not the this contract
                    .argument(&user)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().mint_citizen_callback(&user, wood_deposit.token, food_deposit.token))
                    .async_call_and_exit();
            },
        }
    }

    /// Callback for the mint citizen transaction to update deposit amounts
    #[callback]
    fn mint_citizen_callback(&self, 
        user: &ManagedAddress, 
        wood_token: TokenIdentifier, 
        food_token: TokenIdentifier,
        #[call_result] result: ManagedAsyncCallResult<()>) {

        match result {
            ManagedAsyncCallResult::Ok(_) => {
                // Get user deposits and update spent food and wood
                let deposits = self.get_deposits().get(&user).unwrap_or_default();

                for mut deposit in deposits.iter() {
                    if deposit.token == wood_token {
                        deposit.balance -= MINT_CITIZEN_WOOD_QUANTITY;
                        continue;
                    }
                    if deposit.token == food_token {
                        deposit.balance -= MINT_CITIZEN_FOOD_QUANTITY;
                        continue;
                    }
                }
                // Update deposits to storage
                self.get_deposits().insert(user.clone(), deposits);
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the transaction fails, deposits are not updated
            }
        }
    }

    /// Calls the character contract to claim a citizen after the minting period
    #[endpoint(claimCitizen)]
    fn claim_citizen(&self) {
        self.require_character_contract_address();

        let user = self.blockchain().get_caller();
        // Call the character contract to mint the citizen and transfer the NFT to the user
        self.tx()
            .to(self.character_contract_address().get())
            .raw_call(CHARACTER_CONTRACT_CLAIM_CITIZEN_ENDPOINT_NAME)
            // Claim the NFT to the user address, not this contract
            .argument(&user)
            .async_call_and_exit();
    }

    /// Calls the character contract to upgrade a citizen to a soldier
    /// by specifying the NFT nonce and the NFT owner address if different than the caller
    #[endpoint(upgradeCitizenToSoldier)]
    fn upgrade_citizen_to_soldier(&self, 
            citizen_nft_nonce: u64, 
            nft_owner_address: OptionalValue<ManagedAddress>) {

        self.require_character_contract_address();
        // Get the gold and ore required
        let gold_amount = BigUint::from(UPGRADE_CITIZEN_TO_SOLDIER_GOLD_QUANTITY);
        let ore_amount = BigUint::from(UPGRADE_CITIZEN_TO_SOLDIER_ORE_QUANTITY);

        let user = self.blockchain().get_caller();
        let deposits = self.get_deposits().get(&user).unwrap_or_default();
        // Find gold and ore deposits
        let find_gold_deposit = deposits.iter().find(|deposit| self.is_required_token(&deposit.token, GOLD_TOKEN_TICKER));
        let find_ore_deposit = deposits.iter().find(|deposit| self.is_required_token(&deposit.token, ORE_TOKEN_TICKER));

        match (find_gold_deposit, find_ore_deposit) {
            (None, None) => require!(false, "No gold or ore deposited. Need at least {} and {}.", gold_amount, ore_amount),
            (None, Some(_)) => require!(false, "No gold deposited. Need at least {}.", gold_amount),
            (Some(_), None) => require!(false, "No ore deposited. Need at least {}.", ore_amount),
            (Some(gold_deposit), Some(ore_deposit)) => 
            {
                // Check deposit amounts
                require!(gold_deposit.balance >= gold_amount, "Not enough gold deposited. Need at least {}.", gold_amount);
                require!(ore_deposit.balance >= ore_amount, "Not enough ore deposited. Need at least {}.", ore_amount);
                // Create the payment for the amount of gold and ore to the character contract
                let gold_token_payment = EsdtTokenPayment::new(gold_deposit.token.clone(), 0u64, gold_amount.clone());
                let ore_token_payment = EsdtTokenPayment::new(ore_deposit.token.clone(), 0u64, ore_amount.clone());
                
                let mut payments : ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
                payments.push(gold_token_payment);
                payments.push(ore_token_payment);

                // Set the NFT owner address
                let nft_owner = match nft_owner_address.into_option() {
                    Some(address) => address,
                    None => user.clone(),
                };
                // Call the character contract to upgrade the citizen NFT to a soldier
                self.tx()
                    .to(self.character_contract_address().get())
                    .with_multi_token_transfer(payments)
                    .raw_call(CHARACTER_CONTRACT_UPGRADE_CITIZEN_TO_SOLDIER_ENDPOINT_NAME)
                    .argument(&citizen_nft_nonce)
                    .argument(&nft_owner)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().upgrade_citizen_callback(&user, gold_deposit.token, ore_deposit.token))
                    .async_call_and_exit();
            },
        }
    }

    /// Callback for the upgrade citizen to soldier transaction to update deposit amounts
    #[callback]
    fn upgrade_citizen_callback(&self, 
        user: &ManagedAddress, 
        gold_token: TokenIdentifier, 
        ore_token: TokenIdentifier,
        #[call_result] result: ManagedAsyncCallResult<()>) {

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // Get user deposits and update spent gold and ore
                let deposits = self.get_deposits().get(&user).unwrap_or_default();

                for mut deposit in deposits.iter() {
                    if deposit.token == gold_token {
                        deposit.balance -= UPGRADE_CITIZEN_TO_SOLDIER_GOLD_QUANTITY;
                        continue;
                    }
                    if deposit.token == ore_token {
                        deposit.balance -= UPGRADE_CITIZEN_TO_SOLDIER_ORE_QUANTITY;
                        continue;
                    }
                }
                // Update deposits to storage
                self.get_deposits().insert(user.clone(), deposits);
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the transaction fails, deposits are not updated
            },
        }
    }

}