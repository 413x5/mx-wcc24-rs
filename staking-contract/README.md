# Staking Contract

A MultiversX smart contract that allows users to stake WINTER tokens and earn SNOW rewards.

## Features

### Token Staking

- Users can stake any ESDT token with the ticker name starting with "WINTER"
- Each stake is locked for 5 epochs
- Multiple tokens can be staked in a single transaction
- All stakes are tracked individually per user

### Reward System

- Rewards are distributed in SNOW tokens
- Stakers earn 1% of their staked amount per epoch
- Rewards are distributed once per epoch (24h)
- Automatic distribution can be implemented in a cron job by calling the `distribute_rewards` endpoint every 24h

### Stake Management

- Each stake stores:
  - Token ID (Ticker)
  - Staked amount
  - Unlock epoch
- Stakes are organized by user address

### View Functions

- `get_stake_info`: Get all stakes with user's addresses, their amounts and unlock epochs
- `get_reward_token_id`: Get the ID of the reward token (SNOW)
- `get_last_reward_epoch`: Get the last epoch when rewards were distributed

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

### Reward Management

```rust
#[endpoint(distribute_rewards)]
fn distribute_rewards(&self)
```

- Distributes SNOW rewards to all stakers
- Can only be called once per epoch (24h)
- Automatically calculates and mints rewards for each staker
- Rewards are 1% of staked amount per eligible epoch
- Automatic distribution can be implemented in a cron job by calling it every 24h

### Get Stake Info

```rust
#[view(get_stake_info)]
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

### RewardDistribution

```rust
pub struct RewardDistribution<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub amount: BigUint<M>,
}
```

- Used internally for reward distribution calculations
- Tracks reward amounts per staker address

## How to Use

Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

### Prerequisites

- Have WINTER tokens in your wallet
- If necessary, you can issue tokens with WINTER ticker in your wallet using the `Issue Token` tab in the [MultiversX Web Wallet](https://devnet-wallet.multiversx.com/issue-token)
- Have enough EGLD for transaction fees

### Steps

1. Contract Setup (Owner Only):
   - Call `issue_reward_token` with 0.05 EGLD to issue the SNOW reward token
   - Call `set_reward_token_local_mint_role` to set up minting permissions

2. To stake tokens:
   - Call `stake_token_winter` by sending an amount of WINTER tokens

3. To distribute rewards:
   - Call `distribute_rewards` once per epoch to distribute SNOW rewards to all stakers
   - Automatic distribution can be implemented in a cron job by calling `distribute_rewards` every 24h

4. To query information:
   - Use `get_stake_info` to view all the staked tokens info
   - Use `get_reward_token_id` to get the SNOW token identifier
   - Use `get_last_reward_epoch` to check when rewards were last distributed

## Implementation

See: [src/staking_contract.rs](src/staking_contract.rs)
