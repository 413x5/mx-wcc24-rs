use multiversx_sc::imports::*;
use multiversx_sc::types::EsdtTokenPayment;

use game_common_module::constants::*;

/// Module for interacting with the game arena contract
#[multiversx_sc::module]
pub trait GameArenaModule: 
    crate::storage::StorageModule +
    crate::common::CommonModule +
    game_common_module::GameCommonModule
{

    /// Calls the game arena contract to create a game
    #[endpoint(createGame)]
    fn create_game(&self, soldier_nft_nonce: u64, fee_token_id: TokenIdentifier, fee_amount: BigUint) {
        self.require_game_arena_contract_address();

        let user = self.blockchain().get_caller();

        // Find soldier NFT
        let soldier_nft_deposit = self.get_deposit_by_token_id(&user, &self.characters_collection_id().get(), soldier_nft_nonce);
        // Find fee token deposit
        let fee_token_deposit = self.get_deposit_by_token_id(&user, &fee_token_id, 0);

        match (soldier_nft_deposit, fee_token_deposit) {
            (None, None) => require!(false, "No character NFT or fee token deposited. Need character NFT with nonce {} and fee token {}.", soldier_nft_nonce, fee_token_id),
            (None, Some(_)) => require!(false, "No character NFT deposited with nonce {}.", soldier_nft_nonce),
            (Some(_), None) => require!(false, "No fee token deposited. Need at least {} {}.", fee_amount, fee_token_id),
            (Some(soldier_nft_deposit), Some(fee_token_deposit)) => 
            {   
                // Check deposit amounts
                require!(fee_token_deposit.balance >= fee_amount, "Not enough fee token deposited. Need at least {} {}.", fee_amount, fee_token_id);

                // Get the soldier NFT and fee token
                let soldier_nft_token_id = soldier_nft_deposit.token_id.clone();
                let soldier_nft_nonce = soldier_nft_deposit.token_nonce;

                // Create the transfers for the soldier NFT and fee token to the game arena contract
                let soldier_nft_transfer = EsdtTokenPayment::new(soldier_nft_token_id, soldier_nft_nonce, BigUint::from(1u64));
                let fee_token_transfer = EsdtTokenPayment::new(fee_token_id.clone(), 0u64, fee_amount.clone());

                // Create the multi token transfer
                let mut transfers : ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
                transfers.push(soldier_nft_transfer);
                transfers.push(fee_token_transfer);

                // Call the game arena contract to create a game
                self.tx()
                    .to(self.game_arena_contract_address().get())
                    .with_multi_token_transfer(transfers)
                    .raw_call(GAME_ARENA_CONTRACT_CREATE_GAME_ENDPOINT_NAME)
                    // Set the initiator address for the game to the user address
                    .argument(&user)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().deposit_update_callback(&user, soldier_nft_nonce, &fee_token_id, &fee_amount))
                    .async_call_and_exit();
            },
        }
    }


    /// Calls the game arena contract to accept a game
    #[endpoint(acceptGame)]
    fn accept_game(&self, game_id: u64, soldier_nft_nonce: u64, fee_token_id: TokenIdentifier, fee_amount: BigUint) {
        self.require_game_arena_contract_address();

        let user = self.blockchain().get_caller();

        // Find soldier NFT
        let soldier_nft_deposit = self.get_deposit_by_token_id(&user, &self.characters_collection_id().get(), soldier_nft_nonce);
        // Find fee token deposit
        let fee_token_deposit = self.get_deposit_by_token_id(&user, &fee_token_id, 0);

        match (soldier_nft_deposit, fee_token_deposit) {
            (None, None) => require!(false, "No character NFT or fee token deposited. Need at least {} and {}.", soldier_nft_nonce, fee_amount),
            (None, Some(_)) => require!(false, "No character NFT deposited with nonce {}.", soldier_nft_nonce),
            (Some(_), None) => require!(false, "No fee token deposited. Need at least {}.", fee_amount),
            (Some(soldier_nft_deposit), Some(fee_token_deposit)) => 
            {   
                // Check deposit amounts
                require!(fee_token_deposit.balance >= fee_amount, "Not enough fee token deposited. Need at least {}.", fee_amount);

                // Get the soldier NFT and fee token
                let soldier_nft_token_id = soldier_nft_deposit.token_id.clone();
                let soldier_nft_nonce = soldier_nft_deposit.token_nonce;

                // Create the transfers for the soldier NFT and fee token to the game arena contract
                let soldier_nft_transfer = EsdtTokenPayment::new(soldier_nft_token_id, soldier_nft_nonce, BigUint::from(1u64));
                let fee_token_transfer = EsdtTokenPayment::new(fee_token_id.clone(), 0u64, fee_amount.clone());

                // Create the multi token transfer
                let mut transfers : ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
                transfers.push(soldier_nft_transfer);
                transfers.push(fee_token_transfer);

                // Call the game arena contract to create a game
                self.tx()
                    .to(self.game_arena_contract_address().get())
                    .with_multi_token_transfer(transfers)
                    .raw_call(GAME_ARENA_CONTRACT_ACCEPT_GAME_ENDPOINT_NAME)
                    // Set the game id
                    .argument(&game_id)
                    // Set the initiator address for the game to the user address
                    .argument(&user)
                    // Set the callback for updating deposit amounts if successful
                    .with_callback(self.callbacks().deposit_update_callback(&user, soldier_nft_nonce, &fee_token_id, &fee_amount))
                    .async_call_and_exit();
            },
        }
    }


    /// Callback for the create/accept game transaction to update deposit amounts
    #[callback]
    fn deposit_update_callback(&self, 
        user: &ManagedAddress, 
        soldier_nft_nonce: u64, 
        fee_token_id: &TokenIdentifier,
        fee_amount: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>) {

        match result {
            ManagedAsyncCallResult::Ok(_) => {
                self.decrease_deposit_balance(user, fee_token_id, 0, fee_amount);
                self.remove_deposit(user, &self.characters_collection_id().get(), soldier_nft_nonce);
            },
            ManagedAsyncCallResult::Err(_) => {
                // If the transaction fails, deposits are not updated
            }
        }
    }


}