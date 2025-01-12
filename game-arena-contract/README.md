# Game Arena Contract

A smart contract that manages PvP (Player vs Player) battles in the MultiversX blockchain game. Players can create and accept game challenges using their Soldier NFTs, with entry fees and rewards for winners.

## Contract Structure

- [`game_arena_contract.rs`](src/game_arena_contract.rs): Core contract implementation containing game logic, storage, and endpoints

The contract uses the [Game Common Module](../game-common-module/README.md) to provide common functionality.

## Game Mechanics

### Game Creation

Players can create game challenges by:

1. Depositing an upgraded Soldier NFT (attack or defence > 0)
2. Depositing an entry fee amount in a token of their choice

### Game Acceptance

Other players can accept game challenges by:

1. Depositing their own upgraded Soldier NFT (attack or defence > 0)
2. Paying the same entry fee token and amount
3. Accepting a specific game by its ID

### Battle System

Battles are determined by the following factors:

- Total competency (attack + defence)
- Competency difference between Soldiers
- Random element

Battle resolution:

- If competency difference > 100: Stronger Soldier wins automatically
- Otherwise: Winner determined by weighted random chance:
  - 50-50 base chance
  - Adjusted by competency difference
  - Minimum 1% and maximum 99% win chance
  - Each competency value adds 1% to win chance
  - Uses block random seed for randomness source

## Public Endpoints

### Create Game

```rust
#[payable]
#[endpoint(createGame)]
fn create_game(&self, initiator_address: OptionalValue<ManagedAddress>)
```

Creates a new game challenge:

- Requires:
  - 1 Soldier NFT (must be an upgraded Soldier)
  - Entry fee tokens
- Optional: Specify the initiator's address (used in the [Game Interface](../game-interface-contract/README.md) contract)
- Stores new game in open games

### Accept Game

```rust
#[payable]
#[endpoint(acceptGame)]
fn accept_game(&self, game_id: u64, competitor_address: OptionalValue<ManagedAddress>)
```

Accepts an existing game challenge:

- Requires:
  - 1 Soldier NFT (must be an upgraded Soldier)
  - Matching entry fee token and amount
- Optional: Specify the competitor's address (used in the [Game Interface](../game-interface-contract/README.md) contract)
- Validates: Game exists and is open
- Triggers: Battle resolution and reward distribution
- Winner receives:
  - Both entry fees
  - Their Soldier NFT back

### View Functions

```rust
#[view(getOpenGames)]
fn open_games(&self) -> MapMapper<u64, Game<Self::Api>>

#[view(getCompletedGames)]
fn completed_games(&self) -> MapMapper<u64, Game<Self::Api>>

#[view(getLastGameId)]
fn last_game_id(&self) -> SingleValueMapper<u64>

#[view(getCharactersNftCollection)]
fn characters_nft_collection(&self) -> NonFungibleTokenMapper
```

View functions for:

- List of open games
- List of completed games
- Latest game ID
- Character NFT collection

## Admin Endpoints

### Set Characters NFT Collection

```rust
#[only_owner]
#[endpoint(setCharactersNftCollection)]
fn set_characters_nft_collection(&self, collection_id: TokenIdentifier)
```

Sets the Character NFT collection identifier:

- Required for game operations
- Can only be set once
- Must be set before games can be created

## How to Use

1. Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

    Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

2. Configure the contract:
   - Call `setCharactersNftCollection` as owner to set the Character NFT collection identifier

3. Create a game:
   - Ensure you have a Soldier NFT (must be an upgraded Soldier)
   - Transfer your Soldier NFT and entry fee to the contract with the call of `createGame` endpoint
   - Find your game ID using the `getOpenGames` view

4. Accept a game:
   - Use `getOpenGames` view to find an available game
   - Ensure you have a Soldier NFT
   - Prepare the same entry fee token and amount
   - Transfer your Soldier NFT and entry fee to the contract with the call of `acceptGame` endpoint
   - Battle is automatically resolved and rewards distributed to the winner

5. View game results:
   - Use `getCompletedGames` view to see:
     - Battle participants
     - Winner's Soldier nonce
     - Entry fee details
