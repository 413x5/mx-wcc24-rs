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
    staking_contract
    (
        init => init
        upgrade => upgrade
        issueRewardToken => issue_reward_token
        setRewardTokenLocalMintRole => set_reward_token_local_mint_role
        stakeTokenWinter => stake_token_winter
        distributeRewards => distribute_rewards
        setRewardAddress => set_reward_address
        getRewardAddress => get_reward_address
        getStakeInfo => stake_info
        getLastRewardEpoch => last_reward_epoch
        getRewardTokenId => reward_token_id
    )
}

multiversx_sc_wasm_adapter::async_callback! { staking_contract }
