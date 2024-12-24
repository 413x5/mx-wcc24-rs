# Resource Mint Contract

A MultiversX smart contract that allows users to stake tokens and mint resources based on their stake amount and time intervals.

## Overview

The Resource Mint Contract enables a staking and resource minting mechanism where:

- Users can stake specific tokens (e.g., WINTER tokens)
- Resources are minted based on stake amount and time intervals
- Users can claim their minted resources
- Contract owner can configure various parameters

## Contract Structure

The contract is organized into several modules:

- [`lib.rs`](src/lib.rs): Main contract implementation with public endpoints
- [`data.rs`](src/data.rs): Contract data structures
- [`storage.rs`](src/storage.rs): Storage mappers and state management
- [`admin.rs`](src/admin.rs): Admin-only functions
- [`views.rs`](src/views.rs): View functions
- [`constants.rs`](src/constants.rs): Contract constants and error messages

## Configuration

Key parameters that can be configured:

- Stake token ticker: Identifier for stakeable tokens
- Mint stake threshold: Amount of stake needed per resource
- Mint rounds interval: Number of rounds between resource mints
- Mint if claimed option: Only mint new resources after claiming

## Public Endpoints

### [`stakeTokens`](src/lib.rs)

```rust
#[payable("*")]
#[endpoint(stakeTokens)]
fn stake_tokens(&self)
```

- Allows users to stake tokens
- Tokens must match the configured stake token ticker
- Each stake is recorded with the current round number

### [`mintResources`](src/lib.rs)

```rust
#[endpoint(mintResources)]
fn mint_resources(&self)
```

- Calculates and mints new resources based on stakes
- Resources are minted according to stake amount and rounds passed
- Requires contract to have local mint role

### [`claimResources`](src/lib.rs)

```rust
#[endpoint(claimResources)]
fn claim_resources(&self)
```

- Allows users to claim their minted resources
- Updates user's claimed resources state

## Admin Endpoints

### [`issueResourceToken`](src/admin.rs)

```rust
#[only_owner]
#[endpoint(issueResourceToken)]
fn issue_resource_token(&self)
```

- Issues the resource token
- Requires 0.05 EGLD payment
- Sets up token properties

### [`setResourceTokenLocalMintRole`](src/admin.rs)

```rust
#[only_owner]
#[endpoint(setResourceTokenLocalMintRole)]
fn set_resource_token_local_mint_role(&self)
```

- Sets local mint role for the resource token
- Required for minting new resources

### Configuration Endpoints

- [`setMintRoundsInterval`](src/admin.rs): Set rounds between mints

```rust
#[only_owner]
#[endpoint(setMintRoundsInterval)]
fn set_mint_rounds_interval(&self, mint_rounds: u64)
```

- [`setStakeThreshold`](src/admin.rs): Set stake amount required per resource. Specify as BigUint, including decimals

```rust
#[only_owner]
#[endpoint(setStakeThreshold)]
fn set_stake_threshold(&self, stake_amount: BigUint)
```

- [`setOptionMintIfClaimed`](src/admin.rs): Toggle minting only after claiming

```rust
#[only_owner]
#[endpoint(setOptionMintIfClaimed)]
fn set_option_mint_if_claimed(&self, mint_if_claimed: bool)
```

## Storage

The contract maintains several [storage mappers](src/storage.rs):

- Stake information per user
- Minted and claimed resources per user
- Contract configuration (intervals, thresholds)
- Resource token information

## Usage

1. As owner, [build and deploy the contract](../README.md#building-the-contracts) with initial parameters:
   - Stake token ticker
   - Mint stake threshold (specify as BigUint, including decimals)
   - Mint rounds interval

   ```rust
   init(
     stake_token_ticker: string,
     mint_stake_threshold: BigUint,
     mint_rounds_interval: u64
   )
   ```

2. As owner, issue the resource token:

   Call the issueResourceToken endpoint with the following parameters:

   ```rust
   issueResourceToken(
     token_name: string,
     token_ticker: string,
     initial_supply: optional<BigUint> // default mints 1 token
   )
   ```

3. As owner, set the local mint role for the resource token:

   Call the setResourceTokenLocalMintRole endpoint:

   ```rust
   setResourceTokenLocalMintRole()
   ```

4. Users can stake tokens by calling the stakeTokens endpoint and sending tokens with the ticker configured for staking:

   ```rust
   stakeTokens()
   ```

5. Anyone can call the mintResources endpoint to mint new resources at the interval set in the contract:

   ```rust
   mintResources() 
   ```

6. To see available resources to claim, use the view function getUserUnclaimedResources and provide the user address as a parameter:

   ```rust
   getUserUnclaimedResources(address: ManagedAddress)
   ```

7. Users can claim any available resources by calling the claimResources endpoint:

   ```rust
   claimResources()
   ```
