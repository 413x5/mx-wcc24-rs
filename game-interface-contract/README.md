# Game Interface Contract

A MultiversX smart contract that serves as a unified interface for interacting with the other game contracts. It manages resource deposits and provides simplified access to character and resource transformation operations. The purpose of this contract is to provide a single interface for players to interact with the game.

It handles MultiESDT token transfers and BackTransfers from the other contracts.

## Overview

The contract implements a game interface system where players can:

- Deposit game resources (WOOD, FOOD, STONE, GOLD, ORE)
- Use deposited resources to mint Citizens
- Transform deposited STONE into ORE tokens
- Manage character upgrades and transformations

## Contract Structure

The contract is organized into several modules:

- [`lib.rs`](src/lib.rs): Main contract implementation with deposit management
- [`game_characters.rs`](src/game_characters.rs): Character-related operations (minting, upgrading)
- [`game_resources.rs`](src/game_resources.rs): Resource transformation operations
- [`admin.rs`](src/admin.rs): Admin endpoints for contract configuration
- [`storage.rs`](src/storage.rs): Storage mappers for state management
- [`common.rs`](src/common.rs): Common functionality shared between modules
- [`constants.rs`](src/constants.rs): Contract constants
- [`data.rs`](src/data.rs): Data structures
- [`views.rs`](src/views.rs): View functions

## Configuration

Key parameters:

- **Resource Management**:
  - Tracks deposits for each user
  - Supports all game tokens (WOOD, FOOD, STONE, GOLD, ORE)

- **Contract Dependencies**:
  - Character Contract
  - Resource Transform Contract

## Public Endpoints

### Resource Management

```rust
#[payable("*")]
#[endpoint(depositResources)]
fn deposit_resources(&self)
```

- Accepts any game resource tokens
- Tracks deposits per user
- Required for all game operations

### Character Operations

```rust
#[endpoint(mintCitizen)]
fn mint_citizen(&self)
```

- Mints a new Citizen using deposited resources
- Automatically uses resources from user's deposits
- Requires:
  - 10 WOOD tokens
  - 15 FOOD tokens
- Initiates the minting period (3600 seconds)

```rust
#[endpoint(claimCitizen)]
fn claim_citizen(&self)
```

- Claims a Citizen NFT after the minting period
- Must be called after mintCitizen and waiting period
- NFT is sent directly to the caller's address

```rust
#[endpoint(upgradeCitizenToSoldier)]
fn upgrade_citizen_to_soldier(
    &self,
    citizen_nft_nonce: u64,
    nft_owner_address: OptionalValue<ManagedAddress>
)
```

- Upgrades a Citizen to a Soldier using deposited resources
- Parameters:
  - `citizen_nft_nonce`: The nonce of the Citizen NFT to upgrade
  - `nft_owner_address`: Optional address if the NFT owner is different from caller
- Requires deposited:
  - 5 GOLD tokens
  - 5 ORE tokens
- Burns the resources and upgrades the NFT in place

### Resource Transformation

```rust
#[endpoint(createOre)]
fn create_ore(&self, ore_units: u64)
```

- Creates ORE tokens from deposited STONE
- Requires 20 STONE tokens per ORE unit
- Amount specified by ore_units parameter

## Admin Endpoints

```rust
#[only_owner]
#[endpoint(setCharacterContractAddress)]
fn set_character_contract_address(&self, address: ManagedAddress)

#[only_owner]
#[endpoint(setResourceTransformContractAddress)]
fn set_resource_transform_contract_address(&self, address: ManagedAddress)
```

- Sets addresses for required contract dependencies
- Must be configured before the contract can be used

## Error Cases

The contract handles various error cases including:

- Insufficient deposited resources
- Missing contract dependencies
- Invalid token types
- Failed contract interactions

## How to Use

1. Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

    Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

2. Configure contract dependencies:

   ```rust
   #[only_owner]
   setCharacterContractAddress(address: ManagedAddress)
   setResourceTransformContractAddress(address: ManagedAddress)
   ```

3. Deposit resources:

   ```rust
   #[payable("*")]
   depositResources()
   ```

   Send any combination of:
   - WOOD tokens
   - FOOD tokens
   - STONE tokens
   - GOLD tokens
   - ORE tokens

4. Use deposited resources:

   Mint Citizens:

   ```rust
   mintCitizen()
   ```

   Requires deposited:
   - 10 WOOD tokens
   - 15 FOOD tokens

   Claim Citizen:

   ```rust
   claimCitizen()
   ```

   Create ORE:

   ```rust
   createOre(ore_units: u64)
   ```

   Requires deposited:
   - 20 STONE tokens per ORE unit

   Upgrade Citizen to Soldier:

   ```rust
   upgradeCitizenToSoldier(citizen_nft_nonce: u64, nft_owner_address: OptionalValue<ManagedAddress>)
   ```

   Requires deposited:
   - 5 GOLD tokens
   - 5 ORE tokens
