# Game Common Module

A shared Rust module that provides common functionality for the game contracts. This module contains reusable components, data structures, and utilities used across multiple contracts in the game ecosystem.

## Module Structure

The module is organized into several components:

- [`lib.rs`](src/lib.rs): Main module implementation with token validation utilities
- [`data.rs`](src/data.rs): Common data structures for Characters and Tools
- [`constants.rs`](src/constants.rs): Shared constants used across contracts
- [`nft_attributes.rs`](src/nft_attributes.rs): NFT attribute decoding utilities

## Components

### Data Structures

#### Character

```rust
pub struct Character {
    pub rank: u8,
    pub attack: u8,
    pub defence: u8,
}
```

- Represents game characters with attributes:
  - `rank`: 0 (Citizen) or 1 (Soldier)
  - `attack`: Combat attack value
  - `defence`: Combat defence value

Helper functions:

- `new_citizen()`: Creates a new Citizen (rank 0)
- `new_soldier()`: Creates a new Soldier (rank 1)
- `upgrade(&mut self, tool: &Tool)`: Upgrades character with a tool

#### Tool

```rust
pub struct Tool {
    pub tool_type: u8,
    pub attack: u8,
    pub defence: u8,
}
```

- Represents game tools with attributes:
  - `tool_type`: 1 (Shield) or 2 (Sword)
  - `attack`: Attack bonus value
  - `defence`: Defence bonus value

Helper functions:

- `new_shield()`: Creates a new Shield (defence +1)
- `new_sword()`: Creates a new Sword (attack +1)

### Token Validation

```rust
fn is_required_token_str(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) -> bool
fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &ManagedBuffer) -> bool
fn is_required_nft(&self, check_token_id: &TokenIdentifier, check_token_nonce: u64, required_token_ticker: &ManagedBuffer, required_token_nonce: u64) -> bool
```

- Utilities for validating tokens and NFTs:
  - Token identifier validation
  - NFT collection and nonce validation
  - Error handling for invalid tokens

### NFT Attributes

```rust
fn decode_character(&self, nft_attributes: ManagedBuffer) -> Character
fn decode_tool(&self, nft_attributes: ManagedBuffer) -> Tool
```

- Functions for decoding NFT attributes:
  - Decodes character attributes from NFT metadata format
  - Decodes tool attributes from NFT metadata format
  - Validates attribute format and structure

## Usage

To use this module in a contract:

Add the module as a dependency in your contract's `Cargo.toml`:

```toml
[dependencies.game-common-module]
path = "../game-common-module"
```

Import and use the module in your contract:

```rust
use game_common_module::*;

#[multiversx_sc::contract]
pub trait YourContract:
    game_common_module::GameCommonModule
{
    // Your contract implementation
}
```

## Constants

The module provides various constants used across contracts:

### Token Settings

- Game Token Tickers:
  - `WOOD_TICKER`: "WOOD-"
  - `FOOD_TICKER`: "FOOD-"
  - `STONE_TICKER`: "STONE-"
  - `GOLD_TICKER`: "GOLD-"
  - `ORE_TICKER`: "ORE-"

### Resource Requirements

- Character Minting:
  - `MINT_CITIZEN_WOOD_QUANTITY`: 10 WOOD tokens
  - `MINT_CITIZEN_FOOD_QUANTITY`: 15 FOOD tokens
  - `CITIZEN_TO_SOLDIER_GOLD_QUANTITY`: 5 GOLD tokens
  - `CITIZEN_TO_SOLDIER_ORE_QUANTITY`: 5 ORE tokens

- Tool Minting:
  - `MINT_SHIELD_ORE_QUANTITY`: 2 ORE tokens
  - `MINT_SWORD_GOLD_QUANTITY`: 1 GOLD token
  - `MINT_SWORD_ORE_QUANTITY`: 3 ORE tokens

- Resource Transformation:
  - `STONE_AMMOUNT_FOR_ORE`: 20 STONE tokens required to create ORE

### NFT Collections

- Collection Names and Tickers:
  - Characters: `CHARACTER_COLLECTION_NAME` ("Characters"), `CHARACTER_COLLECTION_TICKER` ("CHARACTER")
  - Tools: `TOOLS_COLLECTION_NAME` ("Tools"), `TOOLS_COLLECTION_TICKER` ("TOOLS")

- NFT Names:
  - `NFT_NAME_CITIZEN`: "Citizen"
  - `NFT_NAME_SOLDIER`: "Soldier"
  - `SHIELD_NFT_NAME`: "Shield"
  - `SWORD_NFT_NAME`: "Sword"

### NFT Metadata

- Royalties (5% for all NFTs):
  - `CHARACTER_NFT_ROYALTIES`: 500
  - `SHIELD_NFT_ROYALTIES`: 500
  - `SWORD_NFT_ROYALTIES`: 500

- IPFS CIDs:
  - `IPFS_CHARACTERS_CID`: Base CID for character assets
  - `IPFS_TOOLS_CID`: Base CID for tool assets

- NFT Tags:
  - Citizens: "character,citizen"
  - Soldiers: "character,soldier"
  - Shields: "tool,shield"
  - Swords: "tool,sword"

### Time Intervals

- Default Minting Cooldowns (in seconds):
  - `MINT_CITIZEN_SECONDS_DEFAULT`: 3600 (1 hour)
  - `MINT_SHIELD_SECONDS_DEFAULT`: 3600 (1 hour)
  - `MINT_SWORD_SECONDS_DEFAULT`: 3600 (1 hour)

### Contract Endpoints

- Resource Contract:
  - `RESOURCE_CONTRACT_MINT_RESOURCES_ENDPOINT_NAME`: "mintResources"
  - `RESOURCE_CONTRACT_CLAIM_RESOURCES_ENDPOINT_NAME`: "claimResources"
  - `RESOURCE_TRANSFORM_CONTRACT_CREATE_ORE_ENDPOINT_NAME`: "createOre"

- Character Contract:
  - `CHARACTER_CONTRACT_MINT_CITIZEN_ENDPOINT_NAME`: "mintCitizen"
  - `CHARACTER_CONTRACT_CLAIM_CITIZEN_ENDPOINT_NAME`: "claimCitizen"
  - `CHARACTER_CONTRACT_UPGRADE_CITIZEN_TO_SOLDIER_ENDPOINT_NAME`: "upgradeCitizenToSoldier"
  - `CHARACTER_CONTRACT_UPGRADE_SOLDIER_ENDPOINT_NAME`: "upgradeSoldier"

- Tools Contract:
  - `TOOLS_CONTRACT_MINT_SHIELD_ENDPOINT_NAME`: "mintShield"
  - `TOOLS_CONTRACT_MINT_SWORD_ENDPOINT_NAME`: "mintSword"
  - `TOOLS_CONTRACT_CLAIM_SHIELD_ENDPOINT_NAME`: "claimShield"
  - `TOOLS_CONTRACT_CLAIM_SWORD_ENDPOINT_NAME`: "claimSword"
