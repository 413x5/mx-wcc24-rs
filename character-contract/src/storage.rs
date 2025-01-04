use multiversx_sc::imports::*;


#[multiversx_sc::module]
pub trait StorageModule {

    /// NFT token id (collection)
    #[view(getNftTokenId)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> NonFungibleTokenMapper;

    /// Citizens to mint for each user
    #[view(getCitizensToMint)]
    #[storage_mapper("citizensToMint")]
    fn citizens_to_mint(&self) -> MapMapper<ManagedAddress, ManagedVec<u64>>;

    /// Last minted NFT nonce
    #[view(getLastMintedNftNonce)]
    #[storage_mapper("lastMintedNftNonce")]
    fn last_minted_nft_nonce(&self) -> SingleValueMapper<u64>;
}
