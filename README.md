# MultiversX Winter Coding Challenge 2024 
### Rust implementation


## Environment Installation

1. Install the [MultiversX Smart Contract Rust SDK](https://docs.multiversx.com/developers/meta/sc-meta)

## Building the contract(s)

2. Use sc-meta in the terminal to build all contracts:
```bash
cd mx-wcc24-rs
sc-meta all build
```
## Deploying and Testing Contracts

Use the MultiversX Utility App [available here](https://utils.multiversx.com/).

- Login with your developer wallet on Devnet
- If necessary, you can get funds from web wallet's [Faucet](https://devnet-wallet.multiversx.com/faucet)
- Go to the [SC Interaction page](https://utils.multiversx.com/smart-contract)
- Deploy the contracts by selecting the `Deploy Contract` tab and use the contract .wasm file from the `/output` folder. 
- Note the deployed contract's address after the deployment transaction is confirmed. 
- You can also look at the transactions list in your wallet to find the contract's address.
- After deployment, select the `Load ABI` tab and select the contract .abi.json file from the `/output` folder
- Make sure your contract's address is entered in the `Contract Address` field
- You are now ready to interact with the contract!


## Contracts

### [Token Manager Contract](token-manager-contract/README.md)

A MultiversX smart contract that allows users to issue and manage ESDT Fungible tokens (SNOW).

