# Resource Transform Contract

A MultiversX smart contract that allows users to transform resources (STONE tokens into ORE tokens), where a specific amount of STONE can be burned to mint ORE tokens.

## Overview

The contract implements a resource transformation system where players can:

- Transform 20 STONE tokens into 1 ORE token
- Send transformed ORE tokens to any specified address
- Burn STONE tokens in the process of creating ORE

## Contract Structure

The contract is organized into several modules:

- [`resource_transform_contract.rs`](src/resource_transform_contract.rs): Main contract implementation with endpoints for resource transformation

## Configuration

Key parameters:

- **Token Configuration**:
  - Name: "Ore"
  - Ticker: "ORE"
  - Decimals: 0

- **Transformation Rate**:
  - 20 STONE tokens = 1 ORE token

- **Required Tokens**:
  - STONE token with ticker "STONE-"
  - ORE token (issued by this contract)

## Public Endpoints

### Resource Transformation

```rust
#[payable("*")]
#[endpoint(createOre)]
fn create_ore(&self, receiver_address: OptionalValue<ManagedAddress>)
```

- Transforms STONE tokens into ORE tokens
- Requires at least 20 STONE tokens per transformation
- Burns the STONE tokens and mints ORE tokens
- Optional receiver address for the minted ORE tokens

## Admin Endpoints

```rust
#[only_owner]
#[payable("EGLD")]
#[endpoint(issueAndSetRolesOreToken)]
fn issue_and_set_roles_ore_token(&self)
```

- Issues the ORE token
- Sets all required roles for the contract
- Requires 0.05 EGLD payment for token issuance

## Error Cases

The contract handles various error cases including:

- Insufficient STONE tokens for transformation
- Invalid token types
- Token already issued errors
- Insufficient EGLD for token issuance

## How to Use

1. Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

    Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

2. Issue the ORE token by calling the issueAndSetRolesOreToken endpoint with 0.05 EGLD:

   ```rust
   #[only_owner]
   #[payable("EGLD")]
   issueAndSetRolesOreToken()
   ```

   This will:
   - Issue a new fungible ESDT token named "Ore" with ticker "ORE"
   - Set all required roles for the contract
   - Configure token properties (0 decimals)

3. Users can transform STONE tokens into ORE by calling the createOre endpoint:

   ```rust
   #[payable("*")]
   createOre(
       receiver_address: OptionalValue<ManagedAddress>
   )
   ```

   Required:
   - At least 20 STONE tokens per ORE token
   - STONE tokens must have the correct ticker ("STONE-")

   The transformation:
   - Burns the STONE tokens
   - Mints new ORE tokens
   - Sends ORE tokens to the specified receiver (or caller if not specified)
