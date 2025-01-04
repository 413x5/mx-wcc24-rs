// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            3
// Async Callback:                       1
// Total number of exported functions:   6

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    resource_transform_contract
    (
        init => init
        upgrade => upgrade
        issueAndSetRolesOreToken => issue_and_set_roles_ore_token
        createOre => create_ore
        getOreTokenId => ore_token_id
    )
}

multiversx_sc_wasm_adapter::async_callback! { resource_transform_contract }
