use multiversx_sc::imports::*;

use crate::data::*;

/// Resource mint contract storage
/// Non required to be views, just for reading the contract state
#[multiversx_sc::module]
pub trait StorageModule {
    /// Token ticker for stake tokens. E.g. WINTER
    #[view(getStakeTokenTicker)]
    #[storage_mapper("stakeTokenTicker")]
    fn stake_token_ticker(&self) -> SingleValueMapper<ManagedBuffer>;

    /// Stake threshold for minting resources   
    #[view(getMintStakeThreshold)]
    #[storage_mapper("mintStakeThreshold")]
    fn mint_stake_threshold(&self) -> SingleValueMapper<BigUint>;

    /// Interval between mint rounds in number of rounds    
    #[view(getMintRoundsInterval)]
    #[storage_mapper("mintRoundsInterval")]
    fn mint_rounds_interval(&self) -> SingleValueMapper<u64>;

    /// Option to mint if user has claimed all previously minted resources
    #[view(getOptionMintIfClaimed)]
    #[storage_mapper("optionMintIfClaimed")]
    fn option_mint_if_claimed(&self) -> SingleValueMapper<bool>;

    /// User stake info
    #[view(getStakeInfo)]
    #[storage_mapper("stakesInfo")]
    fn stakes_info(&self) -> MapMapper<ManagedAddress, ManagedVec<StakeInfo<Self::Api>>>;

    /// User minted resources
    #[view(getUserMintedResources)]
    #[storage_mapper("userMintedResources")]
    fn user_minted_resources(&self) -> MapMapper<ManagedAddress, BigUint>;

    /// User claimed resources
    #[view(getUserClaimedResources)]
    #[storage_mapper("userClaimedResources")]
    fn user_claimed_resources(&self) -> MapMapper<ManagedAddress, BigUint>;

    /// User has unclaimed resources
    #[view(getUserHasUnclaimedResources)]
    #[storage_mapper("userHasUnclaimedResources")]
    fn user_has_unclaimed_resources(&self, address: &ManagedAddress<Self::Api>) -> SingleValueMapper<bool>;

    /// Resource token ID
    #[view(getResourceTokenId)]
    #[storage_mapper("resourceTokenId")]
    fn resource_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    /// Resource token has local mint role
    #[view(getContractHasLocalMintRole)]
    #[storage_mapper("contractHasLocalMintRole")]
    fn contract_has_local_mint_role(&self) -> SingleValueMapper<bool>;

    /// Last resource mint round
    #[view(getLastResourceMintRound)]
    #[storage_mapper("lastResourceMintRound")]
    fn last_resource_mint_round(&self) -> SingleValueMapper<u64>;

    /// Total resources to mint after last update
    #[storage_mapper("resourcesToMint")]
    fn resources_to_mint(&self) -> SingleValueMapper<BigUint>;
}
