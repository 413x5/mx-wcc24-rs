# Staking Contract

A MultiversX smart contract that allows users to stake WINTER tokens.

## Features

### Token Staking

- Users can stake any ESDT token with the ticker name starting with "WINTER"
- Each stake is locked for 5 epochs
- Multiple tokens can be staked in a single transaction
- All stakes are tracked individually per user

### Stake Management

- Each stake stores:
  - Token ID (Ticker)
  - Staked amount
  - Unlock epoch
- Stakes are organized by user address

### View Functions

- `getStakeInfo`: Get all stakes with user's addresses, their amounts and unlock epochs

## Contract Endpoints

### Stake Token

```rust
#[payable("*")]
#[endpoint(stake_token_winter)]
fn stake_token_winter(&self)
```

- Accepts any number of ESDT token payments
- Validates that each token ID must start with "WINTER"
- Tokens are locked for 5 epochs from the current epoch
- Multiple tokens can be staked in a single transaction

### Get Stake Info

```rust
#[view(getStakeInfo)]
fn stake_info(&self) -> MapMapper<ManagedAddress, ManagedVec<StakeInfo<Self::Api>>>
```

- Returns all stakes for all users
- Each stake contains:
  - Token ID
  - Staked amount
  - Unlock epoch
- Stakes are organized by user address

## Data Structures

### StakeInfo

```rust
pub struct StakeInfo<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub unlock_epoch: u64,
}
```

- Stores information about a single stake
- Used to track individual stakes in the contract
- Grouped by user address in storage

## How to Use

Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

### Prerequisites

- Have WINTER tokens in your wallet.
- If necessary, you can issue tokens with WINTER ticker in your wallet using the `Issue Token` tab in the [MultiversX Web Wallet](https://devnet-wallet.multiversx.com/issue-token)
- Have enough EGLD for transaction fees

1. To stake tokens:
   - Call `stake_token_winter` by sending an amount of WINTER tokens

2. To query information:
   - Use `getStakeInfo` to view all the staked tokens info

## Implementation

See: [src/staking_contract.rs](src/staking_contract.rs)
