// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           38
// Async Callback:                       1
// Total number of exported functions:  41

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    game_interface_contract
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        setCharacterContractAddress => set_character_contract_address
        setResourceTransformContractAddress => set_resource_transform_contract_address
        setToolsContractAddress => set_tools_contract_address
        setGameArenaContractAddress => set_game_arena_contract_address
        setWoodMintContractAddress => set_wood_mint_contract_address
        setFoodMintContractAddress => set_food_mint_contract_address
        setStoneMintContractAddress => set_stone_mint_contract_address
        setGoldMintContractAddress => set_gold_mint_contract_address
        setCharactersCollectionId => set_characters_collection
        setToolsCollectionId => set_tools_collection
        clearDeposits => clear_deposits
        clearDeposit => clear_deposit
        setDepositBalance => set_deposit_balance
        getDeposits => get_deposits
        characterContractAddress => character_contract_address
        resourceTransformContractAddress => resource_transform_contract_address
        toolsContractAddress => tools_contract_address
        gameArenaContractAddress => game_arena_contract_address
        woodMintContractAddress => wood_mint_contract_address
        foodMintContractAddress => food_mint_contract_address
        stoneMintContractAddress => stone_mint_contract_address
        goldMintContractAddress => gold_mint_contract_address
        charactersCollectionId => characters_collection_id
        toolsCollectionId => tools_collection_id
        mintCitizen => mint_citizen
        claimCitizen => claim_citizen
        upgradeCitizenToSoldier => upgrade_citizen_to_soldier
        upgradeSoldier => upgrade_soldier
        mintResources => mint_resources
        claimResources => claim_resources
        createOre => create_ore
        mintShield => mint_shield
        claimShield => claim_shield
        mintSword => mint_sword
        claimSword => claim_sword
        createGame => create_game
        acceptGame => accept_game
    )
}

multiversx_sc_wasm_adapter::async_callback! { game_interface_contract }
