use multiversx_sc::imports::*;


#[multiversx_sc::module]
pub trait StorageModule {

    /// Character NFT token id (collection)
    #[view(getCharactersNftCollection)]
    #[storage_mapper("nftTokenId")]
    fn characters_nft_collection(&self) -> NonFungibleTokenMapper;

    /// Citizens to mint for each user
    #[view(getCitizensToMint)]
    #[storage_mapper("citizensToMint")]
    fn citizens_to_mint(&self) -> MapMapper<ManagedAddress, ManagedVec<u64>>;

    /// Mint citizen seconds
    #[view(getMintCitizenSeconds)]
    #[storage_mapper("mintCitizenSeconds")]
    fn mint_citizen_seconds(&self) -> SingleValueMapper<u64>;

    /// Last minted NFT nonce
    #[view(getLastMintedNftNonce)]
    #[storage_mapper("lastMintedNftNonce")]
    fn last_minted_nft_nonce(&self) -> SingleValueMapper<u64>;

    /// Tools NFT token id (collection)
    #[view(getToolsNftCollection)]  
    #[storage_mapper("toolsCollectionId")]
    fn tools_nft_collection(&self) -> SingleValueMapper<TokenIdentifier>;
}
