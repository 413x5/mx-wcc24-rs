#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

const STAKE_EPOCHS: u64 = 5;    // 5 epochs
const STAKE_TOKENID_PREFIX: &str = "WINTER";

/// Stake info structure for each token and staked amount
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct StakeInfo<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub unlock_epoch: u64,
}

/// Staking contract
#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Stores user stakes
    #[view(getStakeInfo)]
    #[storage_mapper("stake_info")]
    fn stake_info(&self) -> MapMapper<ManagedAddress, ManagedVec<StakeInfo<Self::Api>>>;

    /// Stakes configured tokens in the contract
    #[payable("*")]
    #[endpoint(stake_token_winter)]
    fn stake_token_winter(&self) {
        let payments = self.call_value().all_esdt_transfers();
        require!(!payments.is_empty(), "No ESDT tokens received.");

        let stake_token_prefix = ManagedBuffer::from(STAKE_TOKENID_PREFIX.as_bytes());

        // Check that all received tokens are stakeable
        for payment in payments.iter() {
            let token_id = payment.token_identifier;
            let token_id_buffer = token_id.as_managed_buffer();

            require!(
                token_id_buffer.len() >= stake_token_prefix.len() &&
                token_id_buffer.copy_slice(0, stake_token_prefix.len()).unwrap_or_default() == stake_token_prefix,
                "{} tokens are not allowed to be staked. Only send {} tokens to be staked.",
                 token_id, stake_token_prefix
            );
        }

        // Stake tokens
        let caller = self.blockchain().get_caller();
        let current_epoch = self.blockchain().get_block_epoch();

        // Get or create user's stakes vector
        let mut user_stakes = self.stake_info().get(&caller).unwrap_or_default();

        // Store each payment as an individual stake
        for payment in payments.iter() {
            // Create stake info
            let stake_info = StakeInfo {
                token_id: payment.token_identifier.clone(),
                amount: payment.amount.clone(),
                unlock_epoch: current_epoch + STAKE_EPOCHS,
            };

            // Add stake to user's stakes
            user_stakes.push(stake_info);
        }

        // Store updated stakes
        self.stake_info().insert(caller, user_stakes);
    }
}
