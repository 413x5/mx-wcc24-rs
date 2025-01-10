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
- Automatic distribution can be implemented in a cron job by calling the `distributeRewards` endpoint every 24h
- Users can set a custom address to receive their rewards using `setRewardAddress`

### Stake Management

- Each stake stores:
  - Token ID (Ticker)
  - Staked amount
  - Unlock epoch
- Stakes are organized by user address

### View Functions

- `getStakeInfo`: Get all stakes with user's addresses, their amounts and unlock epochs
- `getRewardTokenId`: Get the ID of the reward token (SNOW)
- `getLastRewardEpoch`: Get the last epoch when rewards were distributed
- `getRewardAddress`: Get the address where a user's rewards are sent (returns user's address if no custom address is set)

## Contract Endpoints

### Stake Token

```rust
#[payable]
#[endpoint(stakeTokenWinter)]
fn stake_token_winter(&self)
```

- Accepts any number of ESDT token payments
- Validates that each token ID must start with "WINTER"
- Tokens are locked for 5 epochs from the current epoch
- Multiple tokens can be staked in a single transaction

### Reward Management

```rust
#[endpoint(distributeRewards)]
fn distribute_rewards(&self)
```

- Distributes SNOW rewards to all stakers
- Can only be called once per epoch (24h)
- Automatically calculates and mints rewards for each staker
- Rewards are 1% of staked amount per eligible epoch
- Sends rewards to each staker's configured reward address or their staking address

```rust
#[endpoint(setRewardAddress)]
fn set_reward_address(&self, address: ManagedAddress)
```

- Sets a custom address where the user's rewards will be sent
- All future rewards will be sent to this address instead of the staking address
- If not set, rewards are sent to the user's staking address

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

### RewardDistribution

```rust
pub struct RewardDistribution<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub amount: BigUint<M>,
}
```

- Used internally to track reward distribution
- Stores the recipient address (either staking address or custom reward address)
- Stores the reward amount to be distributed

## How to Use

Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

### Prerequisites

- Have WINTER tokens in your wallet
- If necessary, you can issue tokens with WINTER ticker in your wallet using the `Issue Token` tab in the [MultiversX Web Wallet](https://devnet-wallet.multiversx.com/issue-token)
- Have enough EGLD for transaction fees

### Steps

1. Contract Setup (Owner Only):
   - Call `issueRewardToken` with 0.05 EGLD to issue the SNOW reward token
   - Specify 100000000 in the `Contract Transaction Gas Limit` field to have enough gas for the issue transaction
   - Call `setRewardTokenLocalMintRole` to set up minting permissions

2. To stake tokens:
   - Call `stakeTokenWinter` by sending an amount of WINTER tokens

3. To manage reward distribution:
   - Call `distributeRewards` once per epoch to distribute SNOW rewards
   - Automatic distribution can be implemented in a cron job
   - Call `setRewardAddress` with a custom address to receive rewards at a different address

4. To query information:
   - Use `getStakeInfo` to view all the staked tokens info
   - Use `getRewardTokenId` to get the SNOW token identifier
   - Use `getLastRewardEpoch` to check when rewards were last distributed
   - Use `getRewardAddress` to get the address where a user's rewards are sent

## Implementation

See: [src/staking_contract.rs](src/staking_contract.rs)
