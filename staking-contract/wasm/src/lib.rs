// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    staking_contract
    (
        init => init
        upgrade => upgrade
        getStakeInfo => stake_info
        stake_token_winter => stake_token_winter
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
