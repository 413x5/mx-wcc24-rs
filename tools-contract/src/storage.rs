use multiversx_sc::imports::*;


#[multiversx_sc::module]
pub trait StorageModule {

    /// Tools NFT collection
    #[view(getToolsNftCollection)]
    #[storage_mapper("tools_nft_collection")]
    fn tools_nft_collection(&self) -> NonFungibleTokenMapper;

    /// Shields to mint for each user
    #[view(getShieldsToMint)]
    #[storage_mapper("shieldsToMint")]
    fn shields_to_mint(&self) -> MapMapper<ManagedAddress, ManagedVec<u64>>;

    /// Swords to mint for each user
    #[view(getSwordsToMint)]
    #[storage_mapper("swordsToMint")]
    fn swords_to_mint(&self) -> MapMapper<ManagedAddress, ManagedVec<u64>>;

    /// Last minted NFT nonce
    #[view(getLastMintedNftNonce)]
    #[storage_mapper("lastMintedNftNonce")]
    fn last_minted_nft_nonce(&self) -> SingleValueMapper<u64>;

}