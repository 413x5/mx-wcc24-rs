# Game Interface Contract

A MultiversX smart contract that serves as a unified interface for interacting with the other game contracts. It manages user resource token deposits (WOOD, FOOD, STONE, GOLD, ORE) and provides simplified access to resource, character operations and upgrades, and game battles between upgraded soldiers. The purpose of this contract is to provide a single interface to interact with all the game contracts.

It handles [MultiESDTPayment](https://docs.multiversx.com/developers/transactions/tx-payment/#multi-esdt-payment) token transfers and tokens transfered back (BackTransfers) from the game contracts.

## Overview

The contract implements a game interface where players can:

- Deposit game resources (WOOD, FOOD, STONE, GOLD, ORE)
- Use deposited resources to mint Citizens
- Transform deposited STONE into ORE tokens
- Manage character upgrades and resources
- Craft game tools (Shields and Swords) using deposited resources
- Participate in PvP battles using their upgraded Soldiers in the Game Arena contract

## Contract Structure

The contract is organized into several modules:

- [`lib.rs`](src/lib.rs): Main contract implementation with deposit management
- [`game_characters.rs`](src/game_characters.rs): Character-related operations (citizen minting, citizen claim, soldier upgrading)
- [`game_resources.rs`](src/game_resources.rs): Resource claiming and transformation operations (ORE creation)
- [`game_tools.rs`](src/game_tools.rs): Tools-related operations (shield minting, sword minting)
- [`game_arena.rs`](src/game_arena.rs): Game Arena module for PvP battles
- [`admin.rs`](src/admin.rs): Admin endpoints for contract configuration (set game contract addresses)
- [`storage.rs`](src/storage.rs): Storage mappers for state management
- [`common.rs`](src/common.rs): Common functionality shared between modules
- [`constants.rs`](src/constants.rs): Contract constants and game settings
- [`data.rs`](src/data.rs): Data structures for contract states

The contract uses the [Game Common Module](../game-common-module/README.md) to provide common functionality.

## Configuration

Key parameters:

- **Resource Management**:
  - Tracks deposits for each user
  - Supports all game tokens (WOOD, FOOD, STONE, GOLD, ORE)

- **Contract Dependencies**:

  - [Resource Mint Contracts](../resource-mint-contract/README.md)
  - [Character Contract](../character-contract/README.md)
  - [Resource Transform Contract](../resource-transform-contract/README.md)
  - [Tools Contract](../tools-contract/README.md)
  - [Game Arena Contract](../game-arena-contract/README.md)

## Public Endpoints

### Deposit Endpoint

```rust
#[payable]
#[endpoint(deposit)]
fn deposit(&self)
```

- Accepts any funglible token transfers for deposit
- Accepts Character or Tool NFT transfers for deposit
- Adds tokens to user's deposits

### Resource Management

```rust
#[endpoint(mintResources)]
fn mint_resources(&self)
```

- Mints base resources (WOOD, FOOD, STONE, GOLD) from their respective [Resource Mint Contracts](../resource-mint-contract/README.md)
- Calls each resource contract's mint endpoint if configured
- Automatically triggers minting for all available resource types

```rust
#[endpoint(claimResources)]
fn claim_resources(&self)
```

- Claims all available base resources for the user trough their [Resource Mint Contracts](../resource-mint-contract/README.md)
- Automatically claims from all configured resource contracts (WOOD, FOOD, STONE, GOLD)
- Resources are sent directly to the user's address
- Resources must be deposited using the `depositResources` endpoint

### Resource Transformation

```rust
#[endpoint(createOre)]
fn create_ore(&self, ore_units: u64)
```

- Creates ORE tokens from deposited STONE trough the [Resource Transform Contract](../resource-transform-contract/README.md)
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

- Initiates the mint of a new Citizen using deposited resources, by calling the [Character Contract](../character-contract/README.md)
- Automatically uses resources from user's deposits
- Requires:
  - 10 WOOD tokens
  - 15 FOOD tokens
- The resources are burned by the character contract

```rust
#[endpoint(claimCitizen)]
fn claim_citizen(&self)
```

- Claims a Citizen NFT after the minting period by calling the [Character Contract](../character-contract/README.md)
- Should be called after calling mintCitizen and after the mint waiting period
- The citizen NFT is sent directly to the user's address

```rust
#[endpoint(upgradeCitizenToSoldier)]
fn upgrade_citizen_to_soldier(
    &self,
    citizen_nft_nonce: u64,
    nft_owner_address: OptionalValue<ManagedAddress>
)
```

- Upgrades a Citizen to a Soldier using deposited resources by calling the [Character Contract](../character-contract/README.md)
- Parameters:
  - `citizen_nft_nonce`: The nonce of the Citizen NFT to upgrade
  - `nft_owner_address`: Optional address if the NFT owner is different from the caller
- Requires deposited:
  - 5 GOLD tokens
  - 5 ORE tokens
- The resources are burned by the character contract

```rust
#[endpoint(upgradeSoldier)]
fn upgrade_soldier(&self, soldier_nft_nonce: u64, tool_nft_nonce: u64)
```

- Upgrades a Soldier NFT with a Tool NFT by calling the [Character Contract](../character-contract/README.md)
- Parameters:
  - `soldier_nft_nonce`: Nonce of the Soldier NFT to upgrade
  - `tool_nft_nonce`: Nonce of the Tool NFT to use
- Requires deposited:
  - 1 Soldier NFT (specified by nonce)
  - 1 Tool NFT (specified by nonce)
- Returns the upgraded Soldier NFT to the user

### Tools Operations

```rust
#[endpoint(mintShield)]
fn mint_shield(&self)
```

- Initiates the mint of a Shield NFT using deposited resources by calling the [Tools Contract](../tools-contract/README.md)
- Automatically uses resources from user's deposits:
  - 2 ORE tokens
- The resources are burned by the tools contract

```rust
#[endpoint(claimShield)]
fn claim_shield(&self)
```

- Claims a Shield NFT after the minting period by calling the [Tools Contract](../tools-contract/README.md)
- Should be called after calling mintShield and after the waiting period
- The NFT is sent directly to the user's address

```rust
#[endpoint(mintSword)]
fn mint_sword(&self)
```

- Initiates the mint of a Sword NFT using deposited resources by calling the [Tools Contract](../tools-contract/README.md)
- Automatically uses resources from user's deposits:
  - 3 ORE tokens
  - 1 GOLD tokens
- The resources are burned by the tools contract

```rust
#[endpoint(claimSword)]
fn claim_sword(&self)
```

- Claims a Sword NFT after the minting period by calling the [Tools Contract](../tools-contract/README.md)
- Should be called after calling mintSword and after the waiting period
- The NFT is sent directly to the user's address

### Game Arena Operations

```rust
#[endpoint(createGame)]
fn create_game(&self, soldier_nft_nonce: u64, fee_token_id: TokenIdentifier, fee_amount: BigUint)
```

- Creates a new game challenge that other players can accept trough the [Game Arena Contract](../game-arena-contract/README.md)
- Requires deposited:
  - 1 upgraded Soldier NFT (attack or defence > 0)
  - Fee amount in specified token
- Creates a new game with fee requirements and makes it available for other players

```rust
#[endpoint(acceptGame)]
fn accept_game(&self, game_id: u64, soldier_nft_nonce: u64, fee_token_id: TokenIdentifier, fee_amount: BigUint)
```

- Accepts an existing game challenge and triggers battle resolution trough the [Game Arena Contract](../game-arena-contract/README.md)
- Requires deposited:
  - 1 upgraded Soldier NFT (attack or defence > 0)
  - Matching fee token and amount as specified in the game
- Resolves battle based on:
  - Total competency (attack + defence)
  - Competency difference
  - Weighted random chance
- Winner receives:
  - Both entry fees
  - Their Soldier NFT back

## Admin Endpoints

### Contract Dependencies Endpoints

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

#[only_owner]
#[endpoint(setGameArenaContractAddress)]
fn set_game_arena_contract_address(&self, address: ManagedAddress)
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

1. ### Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

    Use the [MultiversX Utility App](https://utils.multiversx.com/) **Read endpoints** and **Write endpoints** tabs to interact with the contract.

2. ### Configure contract dependencies

   ```rust
   #[only_owner]
   setCharacterContractAddress(address: ManagedAddress)
   setResourceTransformContractAddress(address: ManagedAddress)
   setToolsContractAddress(address: ManagedAddress)
   setGameArenaContractAddress(address: ManagedAddress)
   ```

   To mint and claim resources all at once, set the address of the deployed [Resource Mint Contracts](../resource-mint-contract/README.md):

   ```rust
   #[only_owner]
   setWoodMintContractAddress(address: ManagedAddress)
   setFoodMintContractAddress(address: ManagedAddress)
   setStoneMintContractAddress(address: ManagedAddress)
   setGoldMintContractAddress(address: ManagedAddress)
   ```

3. ### Deposit available resources

   ```rust
   #[payable]
   deposit()
   ```

   Deposit any fungible tokens or game NFTs to the contract:
   - WOOD tokens
   - FOOD tokens
   - STONE tokens
   - GOLD tokens
   - ORE tokens
   - Any game fee tokens
   - Any Character or Tool NFTs

     ### NOTE

      To deposit NFTs, you cannot use the MultiversX Utility App's SC Interaction page, as it does not (yet) support NFT transfers.
      To deposit NFTs, you can send them directly from you wallet, by using the [Web Wallet](https://devnet-wallet.multiversx.com/) or the [Chrome Extension Wallet](https:/chromewebstore.google.com/detail/multiversx-wallet/dngmlblcodfobpdpecaadgfbcggfjfnm)

     - Select the NFT you want to send and click **Send**
     - On the **Transaction Details** page, in the **Receiver** field, enter the contract address
     - Expand the **Fee** section and enter `10,000,000` in the **Gas Limit** field, to have enough gas for the transfer transaction
     - The **Data** section should display the transaction data in this form (your actual data will be different):

        *`ESDTNFTTransfer@4348415241435445522d646366353235@23@01@000000000000000005005da6fd06e116c6cf6951fe964a61a9c70b415f6d9044`*

      This represents the [Standard transfer data](https://docs.multiversx.com/tokens/nft-tokens/#transfers) for sending the NFT to any other address.
  
      Sending the NFT directly to the contract will not call the contract's `deposit` endpoint (and will also fail as the contract is not directly payable).

   - To send the NFT to the contract and call the `deposit` endpoint, we need to use the [Transfer to smart contract](https://docs.multiversx.com/tokens/nft-tokens/#transfers-to-a-smart-contract) transaction format, by adding into the transaction data the name of the endpoint to call (and any additional arguments if necessary)

      To call this endpoint in the NFT transfer transaction, we'll have to add its name at the end of the transaction data.

      The endpoint name has to be encoded in hexadecimal format. You can use the MultiversX Utility App [Converters](https://utils.multiversx.com/converters#string-converters-string-to-hexadecimal) section to do this. Enter `deposit` in the **Convert a string to a hexadecimal encoded string** field, and click **Convert**. The **Result** field shows the hexadecimal encoded endpoint name, which is `6465706f736974`.

   - To edit the transaction data, double-click the **Advanced** label near the **Data** field in the **Transaction Details** page, to make the field editable
   - Add the encoded endpoint name `6465706f736974` at the end of the transaction data and prefix it with the `@` character (that delimits the transaction arguments). The data should now look like:

      *ESDTNFTTransfer@4348415241435445522d646366353235@23@01@000000000000000005005da6fd06e116c6cf6951fe964a61a9c70b415f6d9044`@6465706f736974`*

   - You can now send the NFT to the contract and call the contract's `deposit` endpoint using the **Send NFT** button

4. ### Use deposited resources

   #### - Mint Citizens

   ```rust
   mintCitizen()
   ```

   Requires deposited:
   - 10 WOOD tokens
   - 15 FOOD tokens

   #### - Claim Citizen

   ```rust
   claimCitizen()
   ```

   #### - Create ORE

   ```rust
   createOre(ore_units: u64)
   ```

   Requires deposited:
   - 20 STONE tokens per ORE unit

   #### - Upgrade Citizen to Soldier

   ```rust
   upgradeCitizenToSoldier(citizen_nft_nonce: u64, nft_owner_address: OptionalValue<ManagedAddress>)
   ```

   Requires deposited:
   - 5 GOLD tokens
   - 5 ORE tokens

   #### - Mint Shield

   ```rust
   mintShield()
   ```

   Requires deposited:
   - 2 ORE tokens

   #### - Claim Shield

   ```rust
   claimShield()
   ```

   #### - Mint Sword

   ```rust
   mintSword()
   ```

   Requires deposited:
   - 3 ORE tokens
   - 1 GOLD tokens

   #### - Claim Sword

   ```rust
   claimSword()
   ```

   #### - Upgrade Soldier

   ```rust
   upgradeSoldier(soldier_nft_nonce: u64, tool_nft_nonce: u64)
   ```

   Requires deposited:
   - 1 Soldier NFT (specified by nonce)
   - 1 Tool NFT (specified by nonce)

   #### - Create Game

   ```rust
   createGame(soldier_nft_nonce: u64, fee_token_id: TokenIdentifier, fee_amount: BigUint)
   ```

   Requires deposited:
   - 1 upgraded Soldier NFT (attack or defence > 0)
   - Fee amount in specified token

   #### - Accept Game

   ```rust
   acceptGame(game_id: u64, soldier_nft_nonce: u64, fee_token_id: TokenIdentifier, fee_amount: BigUint)
   ```

   Requires deposited:
   - 1 upgraded Soldier NFT (attack or defence > 0)
   - Matching fee token and amount as specified in the game

## Contract Dependencies

1. Resource Mint Contracts
   - Addresses must be set using
   `setWoodMintContractAddress`,
   `setFoodMintContractAddress`,
   `setStoneMintContractAddress`,
   `setGoldMintContractAddress`

   - Required for minting and claiming the respective resources:
     - WOOD tokens from wood mint contract
     - FOOD tokens from food mint contract
     - STONE tokens from stone mint contract
     - GOLD tokens from gold mint contract

2. Character Contract
   - Address must be set using `setCharacterContractAddress`
   - Required for:
     - Minting Citizens
     - Upgrading Citizens to Soldiers
     - Upgrading Soldiers with tools

3. Resource Transform Contract
   - Address must be set using `setResourceTransformContractAddress`
   - Required for:
     - Creating ORE tokens from STONE

4. Tools Contract
   - Address must be set using `setToolsContractAddress`
   - Required for:
     - Minting Shields
     - Minting Swords
     - Upgrading Soldiers with tools

5. Game Arena Contract
   - Address must be set using `setGameArenaContractAddress`
   - Required for:
     - Creating game challenges
     - Accepting game challenges
     - PvP battles between Soldiers
