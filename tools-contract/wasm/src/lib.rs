// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  12

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    tools_contract
    (
        init => init
        upgrade => upgrade
        mintShield => mint_shield
        claimShield => claim_shield
        mintSword => mint_sword
        claimSword => claim_sword
        getToolsNftCollection => tools_nft_collection
        getShieldsToMint => shields_to_mint
        getSwordsToMint => swords_to_mint
        getLastMintedNftNonce => last_minted_nft_nonce
        registerToolsCollection => register_tools_collection
    )
}

multiversx_sc_wasm_adapter::async_callback! { tools_contract }
