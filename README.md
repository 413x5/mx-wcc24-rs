# MultiversX Winter Coding Challenge 2024

## Rust implementation

A series of smart contracts written in [Rust](https://www.rust-lang.org/) for the [MultiversX](https://multiversx.com/) blockchain.

## Environment Installation

Install the [MultiversX Smart Contract Rust SDK](https://docs.multiversx.com/developers/meta/sc-meta)

## Building the contracts

Use sc-meta in the terminal to build all contracts:

```bash
cd mx-wcc24-rs
sc-meta all build
```

## Deploying and Testing Contracts

Use the [MultiversX Utility App](https://utils.multiversx.com/)

- Login with your developer wallet on Devnet
- If necessary, you can get funds from web wallet's [Faucet](https://devnet-wallet.multiversx.com/faucet)
- Go to the [SC Interaction page](https://utils.multiversx.com/smart-contract)
- Select the `Load ABI` tab and select the contract `[contract-name].abi.json` file from the `/output` folder
- Deploy the contracts by selecting the `Deploy Contract` tab and use the `[contract-name].wasm` file from the `/output` folder
- Note the deployed contract's address after the deployment transaction is confirmed
- You can also look at your `scDeploy` transaction in the Devnet Explorer. To find the contract's address, select the `Logs` section
- Make sure your contract's address is entered in the `Contract Address` field
- You are now ready to interact with the contract!

## Contracts

### [Token Manager Contract](token-manager-contract/README.md)

A smart contract that allows users to issue and manage ESDT Fungible tokens (SNOW).

### [Staking Contract](staking-contract/README.md)

A smart contract that allows users to stake ESDT Fungible tokens (WINTER) and earn SNOW rewards.

### [Resource Mint Contract](resource-mint-contract/README.md)

A smart contract that allows users to stake ESDT Fungible tokens (WINTER) and mint game resources (WOOD, FOOD, STONE, GOLD) based on their stake amount and time intervals.

### [Character Contract](character-contract/README.md)

A smart contract that manages character NFTs for the game. The contract handles minting Citizens using resources (WOOD, FOOD), upgrading them to Soldiers (using GOLD, ORE), and further upgrading Soldiers with Tool NFTs (Shields and Swords).

### [Resource Transform Contract](resource-transform-contract/README.md)

A smart contract that allows users to transform STONE tokens into ORE tokens through a burning mechanism, where a specific amount of STONE is burned to mint ORE tokens.

### [Game Interface Contract](game-interface-contract/README.md)

A smart contract that serves as a unified interface for interacting with the game contracts. It manages user resource deposits, character operations, and tool crafting while handling MultiESDT token transfers between contracts.

### [Tools Contract](tools-contract/README.md)

A smart contract that manages tool NFTs for the game. The contract handles minting Shields (using ORE) and Swords (using ORE and GOLD), which can be used to upgrade Soldier NFTs.

## Common Modules

### [Game Common Module](game-common-module/README.md)

A shared module used across game contracts that provides:

- Common data structures for Characters and Tools
- NFT attribute decoding
- Token validation utilities
- Shared constants and configurations
