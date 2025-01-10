# Resource Mint Contract

A MultiversX smart contract that allows users to stake configurable ESDT Fungible tokens (e.g., WINTER) and mint any number of other ESDT Fungible tokens (e.g., WOOD, FOOD, STONE, GOLD) based on their stake amount and time intervals.
A different contract deployment is required for minting each resource token as each contract instance is configured with different parameters.

The contract is designed to work automatically once deployed and configured by an owner. Users can stake tokens and claim their minted resources. Anyone can call the mint resources endpoint at any time, triggering the minting logic acording to the configured parameters, although this is intended to be called automatically at regular intervals by a scheduled task. If necessary, it could be changed to an only_owner endpoint.

## Configuration

Key parameters that can be configured:

- **Stake token**: Identifier (ticker) for stakeable tokens
- **Resource token**: Name and Identifier (ticker) for minting resource tokens
- **Mint stake threshold**: Amount of stake needed for minting one unit of resource tokens
- **Mint rounds interval**: Number of rounds between resource mints (currenty one round equals 6 seconds)
- **Mint if claimed option**: Only mint new resources if the user has claimed all previously minted resources

## Contract Structure

The contract is organized into several modules:

- [`lib.rs`](src/lib.rs): Main contract implementation with public endpoints
- [`admin.rs`](src/admin.rs): Only-owner Admin endpoints
- [`views.rs`](src/views.rs): View endpoints
- [`storage.rs`](src/storage.rs): Storage mappers for state management
- [`data.rs`](src/data.rs): Contract data structures
- [`constants.rs`](src/constants.rs): Contract constants and error messages

## Public Endpoints

### [`stakeTokens`](src/lib.rs)

```rust
#[payable]
#[endpoint(stakeTokens)]
fn stake_tokens(&self, for_user: OptionalValue<ManagedAddress>)
```

- Allows users to stake tokens
- Parameters:
  - `for_user`: Optional address to stake for a different address than the caller, used in the [Game Interface contract](../game-interface-contract/README.md)
- Tokens must match the configured stake token ticker
- Each stake is recorded with the current round number

### [`mintResources`](src/lib.rs)

```rust
#[endpoint(mintResources)]
fn mint_resources()
```

- Calculates and mints new resources based on stakes
- Resources are minted according to stake amount and rounds passed
- Requires contract to have local mint role

### [`claimResources`](src/lib.rs)

```rust
#[endpoint(claimResources)]
fn claim_resources(&self, for_user: OptionalValue<ManagedAddress>)
```

- Allows users to claim their minted resources
- Parameters:
  - `for_user`: Optional address to claim for a different address than the caller, used in the [Game Interface contract](../game-interface-contract/README.md)
- Updates user's claimed resources state
- Transfers the resources directly to the user's address

## Admin Endpoints

### [`issueResourceToken`](src/admin.rs)

```rust
#[only_owner]
#[payable]
#[endpoint(issueResourceToken)]
fn issue_resource_token(token_name: ManagedBuffer, token_ticker: ManagedBuffer, initial_supply: OptionalValue<BigUint>)
```

- Issues the resource token
- Requires 0.05 EGLD payment
- Sets up token properties

### [`setContractLocalMintRole`](src/admin.rs)

```rust
#[only_owner]
#[endpoint(setContractLocalMintRole)]
fn set_contract_local_mint_role()
```

- Sets local mint role for the contract's resource token
- Required for minting new resources

### Configuration Endpoints

- [`setMintRoundsInterval`](src/admin.rs): Change rounds between mints

```rust
#[only_owner]
#[endpoint(setMintRoundsInterval)]
fn set_mint_rounds_interval(mint_rounds: u64)
```

- [`setStakeThreshold`](src/admin.rs): Change stake amount required per resource. Specify as BigUint, including decimals

```rust
#[only_owner]
#[endpoint(setStakeThreshold)]
fn set_stake_threshold(stake_amount: BigUint)
```

- [`setOptionMintIfClaimed`](src/admin.rs): Toggle minting only after claiming true/false. Default false.

```rust
#[only_owner]
#[endpoint(setOptionMintIfClaimed)]
fn set_option_mint_if_claimed(mint_if_claimed: bool)
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
   stakeTokens(for_user: optional<ManagedAddress>)
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
   claimResources(for_user: optional<ManagedAddress>)
   ```

## *Specific Contract Deployment Parameters*

### WOOD Contract

 ```rust
   init(
     stake_token_ticker: string, "WINTER-",
     mint_stake_threshold: BigUint, "100000000000" // 1000 * 10^8 DECIMALS
     mint_rounds_interval: u64 "600"
   )

   issueResourceToken(
     token_name: string, "WOOD Resources",
     token_ticker: string, "WOOD"
   )
   ```

### FOOD Contract

 ```rust
   init(
     stake_token_ticker: string, "WINTER-",
     mint_stake_threshold: BigUint, "100000000000" // 1000 * 10^8 DECIMALS
     mint_rounds_interval: u64 "1200"
   )

   issueResourceToken(
     token_name: string, "FOOD Resources",
     token_ticker: string, "FOOD"
   )
   ```

### STONE Contract

 ```rust
   init(
     stake_token_ticker: string, "WINTER-",
     mint_stake_threshold: BigUint, "100000000000" // 1000 * 10^8 DECIMALS
     mint_rounds_interval: u64 "1800"
   )

   issueResourceToken(
     token_name: string, "STONE Resources",
     token_ticker: string, "STONE"
   )
   ```

### GOLD Contract

 ```rust
   init(
     stake_token_ticker: string, "WINTER-",
     mint_stake_threshold: BigUint, "100000000000" // 1000 * 10^8 DECIMALS
     mint_rounds_interval: u64 "2400"
   )

   issueResourceToken(
     token_name: string, "GOLD Resources",
     token_ticker: string, "GOLD"
   )
   ```
