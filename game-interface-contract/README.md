# Game Interface Contract

A MultiversX smart contract that serves as a unified interface for interacting with the other game contracts. It manages user resource token deposits (WOOD, FOOD, STONE, GOLD, ORE) and provides simplified access to character and resource operations. The purpose of this contract is to provide a single interface to interact with all the game contracts.

It handles [MultiESDTPayment](https://docs.multiversx.com/developers/transactions/tx-payment/#multi-esdt-payment) token transfers and tokens transfered back (BackTransfers) from the game contracts.

## Overview

The contract implements a game interface where players can:

- Deposit game resources (WOOD, FOOD, STONE, GOLD, ORE)
- Use deposited resources to mint Citizens
- Transform deposited STONE into ORE tokens
- Manage character upgrades and resources
- Craft game tools (Shields and Swords) using deposited resources

## Contract Structure

The contract is organized into several modules:

- [`lib.rs`](src/lib.rs): Main contract implementation with deposit management
- [`game_characters.rs`](src/game_characters.rs): Character-related operations (citizen minting, citizen claim, soldier upgrading)
- [`game_resources.rs`](src/game_resources.rs): Resource claiming and transformation operations (ORE creation)
- [`game_tools.rs`](src/game_tools.rs): Tools-related operations (shield minting, sword minting)
- [`admin.rs`](src/admin.rs): Admin endpoints for contract configuration (set game contract addresses)
- [`storage.rs`](src/storage.rs): Storage mappers for state management
- [`common.rs`](src/common.rs): Common functionality shared between modules
- [`constants.rs`](src/constants.rs): Contract constants and game settings
- [`data.rs`](src/data.rs): Data structures for contract states
- [`views.rs`](src/views.rs): View functions (user resource balance)

## Configuration

Key parameters:

- **Resource Management**:
  - Tracks deposits for each user
  - Supports all game tokens (WOOD, FOOD, STONE, GOLD, ORE)

- **Contract Dependencies**:

  - [Character Contract](../character-contract/README.md)
  - [Resource Transform Contract](../resource-transform-contract/README.md)
  - [Tools Contract](../tools-contract/README.md)
  
  Optional (for batch minting and claiming resources):
  - [Resource Mint Contracts](../resource-mint-contract/README.md)

## Public Endpoints

### Resource Management

```rust
#[payable]
#[endpoint(depositResources)]
fn deposit_resources(&self)
```

- Accepts any game resource tokens
- Tracks deposits per user
- Required for all game operations

```rust
#[endpoint(mintResources)]
fn mint_resources(&self)
```

- Mints base resources (WOOD, FOOD, STONE, GOLD) from their respective contracts
- Calls each resource contract's mint endpoint if configured
- Automatically triggers minting for all available resource types

```rust
#[endpoint(claimResources)]
fn claim_resources(&self)
```

- Claims all available base resources for the user
- Automatically claims from all configured resource contracts (WOOD, FOOD, STONE, GOLD)
- Resources are sent directly to the user's address
- Resources must be deposited using the `depositResources` endpoint

### Resource Transformation

```rust
#[endpoint(createOre)]
fn create_ore(&self, ore_units: u64)
```

- Creates ORE tokens from deposited STONE
- Requires 20 STONE tokens per ORE unit
- Parameters:
  - `ore_units`: Number of ORE units to create

- Process:
  1. Verifies sufficient STONE tokens in user's deposits
  2. Sends STONE tokens to the transform contract
  3. Receives ORE tokens back through callback (BackTransfer)
  4. Updates user deposits:
     - Decreases STONE balance
     - Adds received ORE tokens to deposits

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
- Burns the resources

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

```rust
#[endpoint(upgradeSoldier)]
fn upgrade_soldier(&self, soldier_nft_nonce: u64, tool_nft_nonce: u64)
```

- Upgrades a Soldier NFT with a Tool NFT through the [Character Contract](../character-contract/README.md)
- Parameters:
  - `soldier_nft_nonce`: Nonce of the Soldier NFT to upgrade
  - `tool_nft_nonce`: Nonce of the Tool NFT to use
- Requires deposited:
  - 1 Soldier NFT (specified by nonce)
  - 1 Tool NFT (specified by nonce)
- Returns the upgraded Soldier NFT to the owner

### Tools Operations

```rust
#[endpoint(mintShield)]
fn mint_shield(&self)
```

- Mints a Shield NFT using deposited resources through the [Tools Contract](../tools-contract/README.md)
- Automatically uses resources from user's deposits:
  - 2 ORE tokens
- Initiates the minting period
- Burns the resources

```rust
#[endpoint(claimShield)]
fn claim_shield(&self)
```

- Claims a Shield NFT after the minting period
- Must be called after mintShield and waiting period
- NFT is sent directly to the users's address

```rust
#[endpoint(mintSword)]
fn mint_sword(&self)
```

- Mints a Sword NFT using deposited resources through the [Tools Contract](../tools-contract/README.md)
- Automatically uses resources from user's deposits:
  - 3 ORE tokens
  - 1 GOLD tokens
- Initiates the minting period
- Burns the resources

```rust
#[endpoint(claimSword)]
fn claim_sword(&self)
```

- Claims a Sword NFT after the minting period
- Must be called after mintSword and waiting period
- NFT is sent directly to the user's address

## Admin Endpoints

### Contract Dependencies

```rust
#[only_owner]
#[endpoint(setCharacterContractAddress)]
fn set_character_contract_address(&self, address: ManagedAddress)

#[only_owner]
#[endpoint(setResourceTransformContractAddress)]
fn set_resource_transform_contract_address(&self, address: ManagedAddress)

#[only_owner]
#[endpoint(setToolsContractAddress)]
fn set_tools_contract_address(&self, address: ManagedAddress)
```

- Sets addresses for core contract dependencies
- Must be configured before the contract can be used

### Resource Mint Contracts

```rust
#[only_owner]
#[endpoint(setWoodMintContractAddress)]
fn set_wood_mint_contract_address(&self, address: ManagedAddress)

#[only_owner]
#[endpoint(setFoodMintContractAddress)]
fn set_food_mint_contract_address(&self, address: ManagedAddress)

#[only_owner]
#[endpoint(setStoneMintContractAddress)]
fn set_stone_mint_contract_address(&self, address: ManagedAddress)

#[only_owner]
#[endpoint(setGoldMintContractAddress)]
fn set_gold_mint_contract_address(&self, address: ManagedAddress)
```

- Sets addresses for resource mint contracts
- Each address should point to a deployed [Resource Mint Contract](../resource-mint-contract/README.md)
- Required for minting and claiming the respective resources:
  - WOOD tokens from wood mint contract
  - FOOD tokens from food mint contract
  - STONE tokens from stone mint contract
  - GOLD tokens from gold mint contract

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
   setToolsContractAddress(address: ManagedAddress)
   ```

    To mint and claim resources all at once, set the address of the deployed [Resource Mint Contracts](../resource-mint-contract/README.md):

   ```rust
   #[only_owner]
   setWoodMintContractAddress(address: ManagedAddress)
   setFoodMintContractAddress(address: ManagedAddress)
   setStoneMintContractAddress(address: ManagedAddress)
   setGoldMintContractAddress(address: ManagedAddress)
   ```

3. Deposit available resources:

   ```rust
   #[payable]
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

   Upgrade Soldier:

   ```rust
   upgradeSoldier(soldier_nft_nonce: u64, tool_nft_nonce: u64)
   ```

   Requires deposited:
   - 1 Soldier NFT (specified by nonce)
   - 1 Tool NFT (specified by nonce)

   Mint Shield:

   ```rust
   mintShield()
   ```

   Requires deposited:
   - 2 ORE tokens

   Claim Shield:

   ```rust
   claimShield()
   ```

   Mint Sword:

   ```rust
   mintSword()
   ```

   Requires deposited:
   - 3 ORE tokens
   - 1 GOLD tokens

   Claim Sword:

   ```rust
   claimSword()
   ```
