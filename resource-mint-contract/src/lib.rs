#![no_std]

use multiversx_sc::imports::*;

pub mod data;
pub mod storage;
pub mod admin;
pub mod views;
pub mod constants;

use constants::*;
use data::*;


#[multiversx_sc::contract]
pub trait ResourceMintContract:
    storage::StorageModule +
    admin::AdminModule +
    views::ViewsModule
{
    /// Set up initial contract state
    /// 
    /// # Arguments
    /// * `stake_token_ticker` - Stake token ticker
    /// * `mint_stake_threshold` - Stake amount required to mint one resource
    /// * `mint_rounds_interval` - Number of rounds between resource mints
    #[init]
    fn init(&self, stake_token_ticker: ManagedBuffer, mint_stake_threshold: BigUint, mint_rounds_interval: u64) {
        self.stake_token_ticker().set_if_empty(stake_token_ticker);
        self.mint_stake_threshold().set_if_empty(mint_stake_threshold);
        self.mint_rounds_interval().set_if_empty(mint_rounds_interval);
    }

    /// Contract upgrade logic if necessary
    #[upgrade]
    fn upgrade(&self) {}

    /// Endpoint for staking tokens
    /// 
    /// # Arguments
    /// * `for_user` - User address optional, if not specified the caller address will be used
    #[payable("*")]
    #[endpoint(stakeTokens)]
    fn stake_tokens(&self, for_user: OptionalValue<ManagedAddress>) {
        require!(!self.stake_token_ticker().is_empty(), ERR_STAKE_TOKEN_NOT_SET);
        let stake_token_ticker = self.stake_token_ticker().get();

        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), ERR_NO_ESDT_TOKENS_RECEIVED);

        // Check that all received tokens are stakeable
        for payment in payments.iter() {
            let token_id = payment.token_identifier;
            let token_id_buffer = token_id.as_managed_buffer();

            require!(
                // Check that all received token IDs start with the specified stake token ticker
                token_id_buffer.copy_slice(0, stake_token_ticker.len()).unwrap_or_default() == stake_token_ticker,
                ERR_INVALID_STAKE_TOKEN
            );
        }

        let user = match for_user {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };

        let current_round = self.blockchain().get_block_round();
        let mut user_stakes = self.stakes_info().get(&user).unwrap_or_default();

        // Process each payment and store as an individual stake
        for payment in payments.iter() {
            let token_id = payment.token_identifier;
            let amount = payment.amount;

            let stake_info = StakeInfo {
                token: token_id,
                amount,
                round: current_round,
            };
            user_stakes.push(stake_info);
        }
        self.stakes_info().insert(user, user_stakes);
    }

    /// Endpoint for minting resources
    #[endpoint(mintResources)]
    fn mint_resources(&self) {
        require!(
            !self.resource_token_id().is_empty(),
            ERR_RESOURCE_TOKEN_NOT_ISSUED
        );
        require!(
            self.contract_has_local_mint_role().get(),
            ERR_CONTRACT_NO_MINT_ROLE
        );
        
        // Calculate resources to mint
        self.calculate_resources_to_mint();
        let new_resources_to_mint = self.resources_to_mint().get();

        // Mint new resources if any
        if new_resources_to_mint > BigUint::zero() {
            let resource_token_id = self.resource_token_id().get();
            let amount_to_mint = BigUint::from(new_resources_to_mint) * BigUint::from(10u64).pow(RESOURCE_TOKEN_DECIMALS as u32);
            self.send().esdt_local_mint(
                &resource_token_id,
                0,
                &amount_to_mint
            );

            self.resources_to_mint().clear();
        }
    }

    /// Endpoint for claiming resources
    /// 
    /// # Arguments
    /// * `for_user` - User address optional, if not specified the caller address will be used
    #[endpoint(claimResources)]
    fn claim_resources(&self, for_user: OptionalValue<ManagedAddress>) {

        let user = match for_user {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };

        let user_available = self.user_unclaimed_resources(&user);

        // Send any available resources to the caller
        if user_available > BigUint::zero() {
            let resource_token_id = self.resource_token_id().get();
            self.send().direct_esdt(
                &user,
                &resource_token_id,
                0,
                &user_available
            );
            // Update user state
            self.user_has_unclaimed_resources(&user).set(false);
            let user_claimed = self.user_claimed_resources().get(&user).unwrap_or_default();
            self.user_claimed_resources().insert(user, user_claimed + user_available);
        }
    }

    /// Calculate resources to mint based on stake and rounds passed
    fn calculate_resources_to_mint(&self) { 
        require!(self.mint_rounds_interval().get() > 0, ERR_MINT_ROUNDS_INTERVAL_ZERO);
        require!(self.mint_stake_threshold().get() > 0, ERR_MINT_STAKE_THRESHOLD_ZERO);

        let current_round = self.blockchain().get_block_round();
        let mint_rounds_interval = self.mint_rounds_interval().get();
        let mint_stake_threshold = self.mint_stake_threshold().get();
        let last_mint_round = self.get_last_mint_round();

        let mut new_resources_to_mint = BigUint::zero();
        let mut latest_mint_round = last_mint_round;
        let mut start_mint_round = last_mint_round;
        let mut end_mint_round = start_mint_round + mint_rounds_interval;
        
        // Iterate through all potential passed rounds intervals while not passing the current round
        // This makes sure we can call calculate at any time with the same result
        while end_mint_round <= current_round {
            for user in self.stakes_info().keys() {

                // Skip users that have unclaimed resources if option is set
                if self.option_mint_if_claimed().get() && self.user_has_unclaimed_resources(&user).get() {
                    continue;
                }

                // Iterate through total user stake amount per mint round interval
                let mut total_user_stake_amount = BigUint::from(0u64);
                let user_stakes = self.stakes_info().get(&user).unwrap_or_default();
                for stake in user_stakes.iter() {
                    if stake.round < end_mint_round {
                        total_user_stake_amount += stake.amount.clone();
                    }
                }

                let stake_amount = total_user_stake_amount.clone();
                let stake_per_resource = mint_stake_threshold.clone();

                // Calculate resources to mint based on stake amount
                if stake_amount >= stake_per_resource {
                    // Round (total stake / stake amount per resource) to get number of resources to mint
                    let user_resources_to_mint = BigUint::from((stake_amount / stake_per_resource).to_u64().unwrap_or_default());
                    if user_resources_to_mint > BigUint::zero() {
                        // Add to total new resources to mint
                        new_resources_to_mint += user_resources_to_mint.clone();
                        // Add to user state with new minted resources
                        let user_minted = self.user_minted_resources().get(&user).unwrap_or_default();
                        let total_user_minted = user_minted + user_resources_to_mint;
                        self.user_minted_resources().insert(user, total_user_minted);
                    }
                }
            }
            // Move to next round interval
            latest_mint_round = end_mint_round;
            start_mint_round += mint_rounds_interval;
            end_mint_round += mint_rounds_interval;
        }

        // Update state
        if latest_mint_round > last_mint_round {
            self.last_resource_mint_round().set(latest_mint_round);

            // Update users unclaimed resources flag
            for user in self.stakes_info().keys() {
                if self.user_unclaimed_resources(&user) > BigUint::zero() {
                    self.user_has_unclaimed_resources(&user).set(true);
                }
            }
        }
        // Update resources to mint, to be used in the minting of new resource tokens
        let unminted_resources = if !self.resources_to_mint().is_empty() { self.resources_to_mint().get() } else { BigUint::zero() };
        self.resources_to_mint().set(unminted_resources + new_resources_to_mint);
    }


    /// Get last mint round
    fn get_last_mint_round(&self) -> u64 {
        // If no previous mint round is set initially
        if self.last_resource_mint_round().is_empty() || self.last_resource_mint_round().get() == 0 {
            let mut first_round = self.blockchain().get_block_round();

            for (_, stakes) in self.stakes_info().iter() {
                for stake in stakes.iter() {
                    // Calculate a mint round based on the first stake found
                    if stake.round < first_round {
                        first_round = stake.round;
                    }
                }
             }
             first_round
            } 
        // If a previous mint round is set, return it
        else { self.last_resource_mint_round().get() }
    }
}
