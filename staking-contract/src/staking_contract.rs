#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

const ISSUE_FEE: u64 = 50_000_000_000_000_000; // 0.05 EGLD (0.05 * 10^18 decimals)

const STAKE_UNLOCK_EPOCHS: u64 = 5; // 5 epochs
const STAKE_TOKENID_PREFIX: &str = "WINTER-";

const REWARD_TOKEN_NAME: &str = "SnowMan";
const REWARD_TOKEN_TICKER: &str = "SNOW";
const REWARD_TOKEN_DECIMALS: usize = 8;

/// Stake info structure for each token and stake
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct StakeInfo<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub unlock_epoch: u64,
}

/// Reward distribution structure for each staker and reward amount
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct RewardDistribution<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub amount: BigUint<M>,
}

/// Staking contract
#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
    

    // Admin endpoints

    /// Issue the reward token
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueRewardToken)]
    fn issue_reward_token(&self, initial_supply: OptionalValue<BigUint>) {
        // Check if reward token has already been issued
        require!(self.reward_token_id().is_empty(), "Reward token has already been issued.");
        
        // Check the EGLD payment is enough for issuing the token and token amount is greater than 0.
        // If more EGLD are sent, the difference will be returned to the caller in the callback
        let payment = self.call_value().egld();
        let issue_cost = BigUint::from(ISSUE_FEE);
        require!(
            *payment >= issue_cost,
            "Must send at least 0.05 EGLD for the issue cost. Any extra funds will be returned.");

        // Initial supply
        let supply = match initial_supply {
            OptionalValue::Some(amount) => {
                require!(amount > BigUint::zero(), "Intitial supply to issue must be greater than 0.");
                amount
            },
            // Reward token default initial supply of 1
            OptionalValue::None => BigUint::from(1u64)*BigUint::from(10u64).pow(REWARD_TOKEN_DECIMALS as u32), 
        };

        // Set token properties
        let mut properties = FungibleTokenProperties::default();
        properties.num_decimals = REWARD_TOKEN_DECIMALS;

        let caller = self.blockchain().get_caller();

        // Send the issue transaction with callback
        self.send()
            .esdt_system_sc_tx()
            .issue_fungible(
                issue_cost,
                REWARD_TOKEN_NAME,
                REWARD_TOKEN_TICKER,
                supply,
                properties
            )
            .with_callback(self.callbacks().issue_reward_token_callback(&caller, &payment))
            .async_call_and_exit()
    }

    /// Callback for the issue transaction to update the contract state with the issued token
    /// Send any EGLD paid back to the caller
    #[callback]
    fn issue_reward_token_callback(
        &self,
        caller: &ManagedAddress,
        payment: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {        
        match result {
            // If the issue was successful
            ManagedAsyncCallResult::Ok(()) => {
                // get the returned token
                let (token_identifier, _returned_tokens) = 
                    self.call_value().single_fungible_esdt();
                // Set reward token id
                self.reward_token_id().set(token_identifier);

                // return any extra EGLD amount to the caller
                let issue_fee = BigUint::from(ISSUE_FEE);
                if payment > &issue_fee {
                    let difference = payment - &issue_fee;
                    self.tx().to(caller).egld(difference).transfer();
                }
            },
            // If the issue failed
            ManagedAsyncCallResult::Err(_) => {
                // return the EGLD payment to the caller
                self.tx().to(caller).egld(payment).transfer();
            },
        }
    }

    /// Sets the local mint role for the reward token
    #[only_owner]
    #[endpoint(setRewardTokenLocalMintRole)]
    fn set_reward_token_local_mint_role(&self) {
        require!(!self.reward_token_id().is_empty(), "Reward token not set. Call issue_reward_token first.");

        self.send()
            .esdt_system_sc_tx()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.reward_token_id().get(),
                [EsdtLocalRole::Mint].iter().cloned(),
            )
            .with_callback(self.callbacks().set_local_mint_role_callback())
            .async_call_and_exit();
    }

    /// Callback for the set local mint role transaction
    #[callback]
    fn set_local_mint_role_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // Reward token has local mint role
                self.reward_token_has_local_mint_role().set(true);
            },
            ManagedAsyncCallResult::Err(_) => {
            },
        }
    }


    // Public endpoints

    /// Stake tokens
    #[payable("*")]
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), "No ESDT tokens received.");

        // Check that all received tokens are stakeable
        for payment in payments.iter() {
            let token_id = &payment.token_identifier;

            self.require_expected_token(token_id, STAKE_TOKENID_PREFIX);
        }

        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();

        // Get or create user's stakes list
        let mut user_stakes = self.stake_info().get(&caller).unwrap_or_default();

        // Store each payment as an individual stake
        for payment in payments.iter() {
            // Create stake info
            let stake_info = StakeInfo {
                token_id: payment.token_identifier.clone(),
                amount: payment.amount.clone(),
                unlock_epoch: current_epoch + STAKE_UNLOCK_EPOCHS,
            };

            // Add stake to user's stakes
            user_stakes.push(stake_info);
        }

        // Store updated stakes
        self.stake_info().insert(caller, user_stakes);
    }



    /// Distribute rewards to all stakers
    #[endpoint(distributeRewards)]
    fn distribute_rewards(&self) {      
        require!(!self.reward_token_id().is_empty(), "Reward token not set. Call issue_reward_token first.");
        require!(self.reward_token_has_local_mint_role().get(), 
        "Reward token does not have local mint role. Call set_reward_token_local_mint_role first.");

        let current_epoch = self.blockchain().get_block_epoch();
        let last_reward_epoch = self.get_last_reward_epoch(current_epoch);

        // Check if 24h (1 epoch) has passed since last reward
        require!(
            current_epoch > last_reward_epoch,
            "Rewards can only be distributed once every epoch (24h). Last distribution was at epoch {}.", last_reward_epoch
        );
        
        // Calculate rewards for all stakers based on the epoch of their stakes and last reward epoch
        let mut rewards = ManagedVec::<Self::Api, RewardDistribution<Self::Api>>::new();
        let mut total_rewards = BigUint::zero();
        
        for (address, stakes) in self.stake_info().iter() {
            let mut address_total_reward = BigUint::zero();
            
            for stake in stakes.iter() {
                let stake_epoch = stake.unlock_epoch - STAKE_UNLOCK_EPOCHS;
                
                // If the last reward epoch is before or the same as the stake epoch, reward is from stake epoch to current epoch
                let reward_epochs = if last_reward_epoch <= stake_epoch {current_epoch - stake_epoch}
                // If the last reward epoch is after the stake epoch, reward is from last reward epoch to the current epoch
                else if last_reward_epoch < current_epoch {current_epoch - last_reward_epoch}
                // If the last reward epoch is the same as the stake epoch, reward is 0
                else {0u64};
                
                
                if reward_epochs > 0u64 {
                    // Calculate reward for this stake: 1% of stake amount per reward epoch
                    let stake_reward = &stake.amount / 100u32 * reward_epochs;
                    if stake_reward > 0u64 {
                        address_total_reward += stake_reward;
                    }
                }
            }
            
            if address_total_reward > 0u64 {
                total_rewards += &address_total_reward;
                // Use reward address if set, otherwise use staker address
                let reward_address = self.get_reward_address(&address);
                rewards.push(RewardDistribution {
                    address: reward_address,
                    amount: address_total_reward,
                });
            }
        }

        // Get reward token id    
        let reward_token_id = self.reward_token_id().get();
        
        // If there are rewards to distribute
        if total_rewards > 0u64 {
            // Mint total rewards in one transaction
            self.send().esdt_local_mint(&reward_token_id, 0, &total_rewards);
            
            // Distribute rewards to each staker
            for reward_distribution in rewards.iter() {
                self.send().direct_esdt(&reward_distribution.address, &reward_token_id, 0, &reward_distribution.amount);
            }
        }
        
        self.last_reward_epoch().set(current_epoch);
    }


    /// Sets the reward address for a user
    #[endpoint(setRewardAddress)]
    fn set_reward_address(&self, address: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.reward_address(&caller).set(address);
    }  

    
    // Private functions

    /// Check if a token is a required token
    fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == ManagedBuffer::from(required_token_ticker.as_bytes())
    }

    /// Check if a token is a required token and terminates if not
    fn require_expected_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) {
        let expected_token = ManagedBuffer::from(required_token_ticker.as_bytes());
        require!(self.is_required_token(check_token_id, required_token_ticker), "Invalid token {}. Expected {}.", check_token_id, expected_token);
    }

    /// Returns the last epoch in which rewards were distributed
    /// If no rewards have been distributed, returns the epoch of the first stake
    /// If there are no stakes, returns the current epoch
    fn get_last_reward_epoch(&self, current_epoch: u64) -> u64 {
        let last_reward_epoch = if !self.last_reward_epoch().is_empty() {
            self.last_reward_epoch().get()
        } else {
            // If no rewards have been distributed, start from the first stake epoch
            let mut first_stake_epoch = current_epoch;
            for (_, stakes) in self.stake_info().iter() {
                for stake in stakes.iter() {
                    let stake_epoch = stake.unlock_epoch - STAKE_UNLOCK_EPOCHS;
                    if stake_epoch < first_stake_epoch {
                        first_stake_epoch = stake_epoch;
                    }
                }
            }
            first_stake_epoch
        };
        last_reward_epoch
    }

    /// Gets the reward address for a user, returns user address if not set
    #[view(getRewardAddress)]
    fn get_reward_address(&self, address: &ManagedAddress) -> ManagedAddress {
        if !self.reward_address(address).is_empty() {
            self.reward_address(address).get()
        } else {
            address.clone()
        }
    }

  


    // Storage

    /// Stores user stakes
    #[view(getStakeInfo)]
    #[storage_mapper("stake_info")]
    fn stake_info(&self) -> MapMapper<ManagedAddress, ManagedVec<StakeInfo<Self::Api>>>;
    
    /// Stores the last reward epoch
    #[view(getLastRewardEpoch)]
    #[storage_mapper("last_reward_epoch")]
    fn last_reward_epoch(&self) -> SingleValueMapper<u64>;

    /// Stores the reward token id
    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /// Stores if the reward token has the local mint role successfully set
    #[storage_mapper("reward_token_has_local_mint_role")]
    fn reward_token_has_local_mint_role(&self) -> SingleValueMapper<bool>;

    /// Stores the reward address for each user
    #[storage_mapper("reward_address")]
    fn reward_address(&self, address: &ManagedAddress) -> SingleValueMapper<ManagedAddress>;
}
