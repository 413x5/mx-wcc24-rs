#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

// Cost and decimal constants
const ISSUE_FEE: u64 = 50_000_000_000_000_000; // 0.05 EGLD
const RESOURCE_TOKEN_DECIMALS: usize = 0; // In this case resources don't have decimals

// Error messages
const ERR_RESOURCE_TOKEN_ALREADY_ISSUED: &str = "Resource token already issued.";
const ERR_TOKEN_NAME_EMPTY: &str = "Token name must not be empty.";
const ERR_TOKEN_TICKER_EMPTY: &str = "Token ticker must not be empty.";
const ERR_INSUFFICIENT_ISSUE_COST: &str = "Must send 0.05 EGLD for issue cost.";
const ERR_INITIAL_SUPPLY_ZERO: &str = "Initial supply to issue must be greater than 0.";
const ERR_RESOURCE_TOKEN_NOT_ISSUED: &str = "Resource token not issued.";
const ERR_CONTRACT_NO_MINT_ROLE: &str = "Contract does not have mint role.";
const ERR_MINT_ROUNDS_INTERVAL_ZERO: &str = "Mint rounds interval must be greater than 0.";
const ERR_MINT_STAKE_THRESHOLD_ZERO: &str = "Mint stake threshold must be greater than 0.";
const ERR_STAKE_TOKEN_NOT_SET: &str = "Stake token not set.";
const ERR_NO_ESDT_TOKENS_RECEIVED: &str = "No ESDT tokens received.";
const ERR_INVALID_STAKE_TOKEN: &str = "Sent tokens are not valid for staking.";

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct StakeInfo<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub round: u64,
}

#[multiversx_sc::contract]
pub trait ResourceMintContract {
    #[init]
    fn init(&self, /*stake_token_ticker: ManagedBuffer, mint_stake_amount: u64, mint_rounds: u64*/) {
        // self.stake_token_ticker().set_if_empty(stake_token_ticker);
        // self.stake_amount().set_if_empty(mint_stake_amount);
        // self.mint_rounds().set_if_empty(mint_rounds);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // Admin endpoints

    #[only_owner]
    #[endpoint(setMintRoundsInterval)]
    fn set_mint_rounds_interval(&self, mint_rounds: u64) {
        self.mint_rounds_interval().set(mint_rounds);
    }

    #[only_owner]
    #[endpoint(setStakeTokenTicker)]
    fn set_stake_token_ticker(&self, stake_token_ticker: ManagedBuffer) {
        self.stake_token_ticker().set(stake_token_ticker);
    }

    #[only_owner]
    #[endpoint(setStakeThreshold)]
    fn set_stake_threshold(&self, stake_amount: BigUint) {
        self.mint_stake_threshold().set(stake_amount);
    }

    #[only_owner]
    #[endpoint(setOptionMintIfClaimed)]
    fn set_option_mint_if_claimed(&self, mint_if_claimed: bool) {
        self.option_mint_if_claimed().set(mint_if_claimed);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueResourceToken)]
    fn issue_resource_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer, initial_supply: OptionalValue<BigUint>) {
        require!(self.resource_token_id().is_empty(), ERR_RESOURCE_TOKEN_ALREADY_ISSUED);
        require!(!token_name.is_empty(), ERR_TOKEN_NAME_EMPTY);
        require!(!token_ticker.is_empty(), ERR_TOKEN_TICKER_EMPTY);

        let payment = self.call_value().egld_value();
        let issue_cost = BigUint::from(ISSUE_FEE);
        require!(
            *payment >= issue_cost,
            ERR_INSUFFICIENT_ISSUE_COST
        );

        // Initial supply
        let supply = match initial_supply {
            OptionalValue::Some(amount) => {
                require!(amount > BigUint::zero(), ERR_INITIAL_SUPPLY_ZERO);
                amount
            },
            // Reward token default initial supply of 1
            OptionalValue::None => BigUint::from(1u64)*BigUint::from(10u64).pow(RESOURCE_TOKEN_DECIMALS as u32), 
        };

        self.send()
            .esdt_system_sc_tx()
            .issue_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                &supply,
                FungibleTokenProperties {
                    num_decimals: RESOURCE_TOKEN_DECIMALS,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .with_callback(self.callbacks().issue_callback())
            .async_call_and_exit()
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                let (token_identifier, _returned_tokens) = 
                    self.call_value().single_fungible_esdt();
                self.resource_token_id().set(token_identifier.clone());
            },
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_value();
                if *returned > 0 {
                    self.send().direct_egld(&caller, &returned);
                }
            },
        }
    }

    /// Sets the local mint role for the resource token
    #[only_owner]
    #[endpoint(setResourceTokenLocalMintRole)]
    fn set_resource_token_local_mint_role(&self) {
        require!(!self.resource_token_id().is_empty(), ERR_RESOURCE_TOKEN_NOT_ISSUED);

        self.send()
            .esdt_system_sc_tx()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.resource_token_id().get(),
                [EsdtLocalRole::Mint].iter().cloned(),
            )
            .with_callback(self.callbacks().resource_mint_role_callback())
            .async_call_and_exit();
    }

    /// Callback for the set local mint role transaction
    #[callback]
    fn resource_mint_role_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // Resource token has local mint role
                self.resource_token_has_local_mint_role().set(true);
            },
            ManagedAsyncCallResult::Err(_) => {
            },
        }
    }

    // Private functions

    fn get_last_update_round(&self) -> u64 {
        if self.last_resource_update_round().is_empty() || self.last_resource_update_round().get() == 0 {
            // If no previous mint round is set, calculate a mint round from first stake round
            let mut first_stake_round = self.blockchain().get_block_round();
            for (_, stakes) in self.stakes_info().iter() {
                for stake in stakes.iter() {
                    if stake.round < first_stake_round {
                        first_stake_round = stake.round;
                    }
                }
             }
             first_stake_round
            } 
        else { self.last_resource_update_round().get() }
    }

    fn calculate_resources_to_mint(&self) { 
        require!(self.mint_rounds_interval().get() > 0, ERR_MINT_ROUNDS_INTERVAL_ZERO);
        require!(self.mint_stake_threshold().get() > 0, ERR_MINT_STAKE_THRESHOLD_ZERO);

        let current_round = self.blockchain().get_block_round();
        let mint_rounds_interval = self.mint_rounds_interval().get();
        let mint_stake_threshold = self.mint_stake_threshold().get();
        let last_update_round = self.get_last_update_round();

        let mut new_resources_to_mint = BigUint::zero();
        let mut latest_update_round = last_update_round;
        let mut start_mint_round = last_update_round;
        let mut end_mint_round = start_mint_round + mint_rounds_interval;
        
        while end_mint_round <= current_round {
            
            for user in self.stakes_info().keys() {

                // If mint new resources only after all minted are claimed and user has unclaimed resources
                if self.option_mint_if_claimed().get() && self.user_has_unclaimed_resources(&user).get() {
                    // don't calculate new resources for this user, continue to the next user
                    continue;
                }

                // Calculate total stake amount for this interval
                let mut total_user_stake_amount = BigUint::from(0u64);
                let user_stakes = self.stakes_info().get(&user).unwrap_or_default();
                for stake in user_stakes.iter() {
                    if stake.round < end_mint_round {
                        total_user_stake_amount += stake.amount.clone();
                    }
                }

                let stake_amount = total_user_stake_amount.clone();
                let stake_per_mint = mint_stake_threshold.clone();

                // Calculate resources to mint based on stake amount
                if stake_amount >= stake_per_mint {
                    // In this case we want only the integer part of the division, so we round the result by converting to u64
                    let user_resources_to_mint = BigUint::from((stake_amount / stake_per_mint).to_u64().unwrap_or_default());
                    // Update user's total minted resources if they earned any
                    if user_resources_to_mint > BigUint::zero() {
                        new_resources_to_mint += user_resources_to_mint.clone();
                        let user_minted = self.user_minted_resources().get(&user).unwrap_or_default();
                        let total_user_minted = user_minted + user_resources_to_mint;
                        self.user_minted_resources().insert(user, total_user_minted);
                    }
                }
            
            }
        latest_update_round = end_mint_round;
        start_mint_round += mint_rounds_interval;
        end_mint_round += mint_rounds_interval;
        }

        // Update state

        if latest_update_round > last_update_round {
            self.last_resource_update_round().set(latest_update_round);

            for user in self.stakes_info().keys() {
                if self.user_unclaimed_resources(&user) > BigUint::zero() {
                    self.user_has_unclaimed_resources(&user).set(true);
                }
            }
        }
        let unminted_resources = if !self.resources_to_mint().is_empty() { self.resources_to_mint().get() } else { BigUint::zero() };
        self.resources_to_mint().set(unminted_resources + new_resources_to_mint);
    }

    // Public endpoints

    #[payable("*")]
    #[endpoint(stakeTokens)]
    fn stake_tokens(&self) {
        require!(!self.stake_token_ticker().is_empty(), ERR_STAKE_TOKEN_NOT_SET);
        let stake_token_ticker = self.stake_token_ticker().get();

        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), ERR_NO_ESDT_TOKENS_RECEIVED);

        // Check that all received tokens are stakeable
        for payment in payments.iter() {
            let token_id = payment.token_identifier;
            let token_id_buffer = token_id.as_managed_buffer();

            require!(
                token_id_buffer.copy_slice(0, stake_token_ticker.len()).unwrap_or_default() == stake_token_ticker,
                ERR_INVALID_STAKE_TOKEN
            );
        }

        let caller = self.blockchain().get_caller();
        let current_round = self.blockchain().get_block_round();
        // Get or create user's stakes list
        let mut user_stakes = self.stakes_info().get(&caller).unwrap_or_default();

        // Process each payment
        for payment in payments.iter() {
            let token_id = payment.token_identifier;
            let amount = payment.amount;

            // Add new stake
            let stake_info = StakeInfo {
                token: token_id,
                amount,
                round: current_round,
            };
            user_stakes.push(stake_info);
        }
        // Store updated stakes
        self.stakes_info().insert(caller, user_stakes);
    }


    #[endpoint(mintResources)]
    fn mint_resources(&self) {
        require!(
            !self.resource_token_id().is_empty(),
            ERR_RESOURCE_TOKEN_NOT_ISSUED
        );
        require!(
            self.resource_token_has_local_mint_role().get(),
            ERR_CONTRACT_NO_MINT_ROLE
        );
        
        // Update resources to mint
        self.calculate_resources_to_mint();

        let total_resources_to_mint = self.resources_to_mint().get();

        // Mint total resources if any
        if total_resources_to_mint > BigUint::zero() {
            let resource_token_id = self.resource_token_id().get();
            let amount_to_mint = BigUint::from(total_resources_to_mint) * BigUint::from(10u64).pow(RESOURCE_TOKEN_DECIMALS as u32);
            self.send().esdt_local_mint(
                &resource_token_id,
                0,
                &amount_to_mint
            );

            self.resources_to_mint().clear();
        }
    }


    #[endpoint(claimResources)]
    fn claim_resources(&self) {

        let caller = self.blockchain().get_caller();
        let user_available = self.user_unclaimed_resources(&caller);

        if user_available > BigUint::zero() {
            let resource_token_id = self.resource_token_id().get();
            self.send().direct_esdt(
                &caller,
                &resource_token_id,
                0,
                &user_available
            );
            self.user_has_unclaimed_resources(&caller).set(false);
            let user_claimed = self.user_claimed_resources().get(&caller).unwrap_or_default();
            self.user_claimed_resources().insert(caller, user_claimed + user_available);
        }
    }

    // Views

    #[view(getRound)]
    fn get_round(&self) -> u64 {
        self.blockchain().get_block_round()
    }

    #[view(getUserUnclaimedResources)]
    fn user_unclaimed_resources(&self, address: &ManagedAddress<Self::Api>) -> BigUint {
        let user_minted = self.user_minted_resources().get(&address).unwrap_or_default();
        let user_claimed = self.user_claimed_resources().get(&address).unwrap_or_default();
        user_minted - user_claimed
    }
    
    // Storage

    #[view(getMintRoundsInterval)]
    #[storage_mapper("mintRoundsInterval")]
    fn mint_rounds_interval(&self) -> SingleValueMapper<u64>;

    #[view(getMintStakeThreshold)]
    #[storage_mapper("mintStakeThreshold")]
    fn mint_stake_threshold(&self) -> SingleValueMapper<BigUint>;

    #[view(getStakeTokenTicker)]
    #[storage_mapper("stakeTokenTicker")]
    fn stake_token_ticker(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getOptionMintIfClaimed)]
    #[storage_mapper("optionMintIfClaimed")]
    fn option_mint_if_claimed(&self) -> SingleValueMapper<bool>;

    #[view(getStakeInfo)]
    #[storage_mapper("stakesInfo")]
    fn stakes_info(&self) -> MapMapper<ManagedAddress, ManagedVec<StakeInfo<Self::Api>>>;

    #[view(getUserMintedResources)]
    #[storage_mapper("userMintedResources")]
    fn user_minted_resources(&self) -> MapMapper<ManagedAddress, BigUint>;

    #[view(getUserClaimedResources)]
    #[storage_mapper("userClaimedResources")]
    fn user_claimed_resources(&self) -> MapMapper<ManagedAddress, BigUint>;

    #[view(getUserHasUnclaimedResources)]
    #[storage_mapper("userHasUnclaimedResources")]
    fn user_has_unclaimed_resources(&self, address: &ManagedAddress<Self::Api>) -> SingleValueMapper<bool>;

    #[view(getResourcesToMint)]
    #[storage_mapper("resourcesToMint")]
    fn resources_to_mint(&self) -> SingleValueMapper<BigUint>;

    #[view(getLastResourceUpdateRound)]
    #[storage_mapper("lastResourceUpdateRound")]
    fn last_resource_update_round(&self) -> SingleValueMapper<u64>;

    #[view(getResourceTokenId)]
    #[storage_mapper("resourceTokenId")]
    fn resource_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /// Stores if the resource token has the local mint role successfully set
    #[storage_mapper("resource_token_has_local_mint_role")]
    fn resource_token_has_local_mint_role(&self) -> SingleValueMapper<bool>;
}