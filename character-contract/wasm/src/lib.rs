// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            7
// Async Callback:                       1
// Total number of exported functions:  10

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    character_contract
    (
        init => init
        upgrade => upgrade
        mintCitizen => mint_citizen
        claimCitizen => claim_citizen
        upgradeCitizenToSoldier => upgrade_citizen_to_soldier
        registerCharactersCollection => register_characters_collection
        getNftTokenId => nft_token_id
        getCitizensToMint => citizens_to_mint
        getLastMintedNftNonce => last_minted_nft_nonce
    )
}

multiversx_sc_wasm_adapter::async_callback! { character_contract }
