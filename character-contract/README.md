# Character Contract

A MultiversX smart contract that handles character NFTs for the game. The contract manages the minting and upgrading of character NFTs (Citizens and Soldiers) and their interactions with tool NFTs.

## Overview

The contract implements a character NFT game where players can use the tokens from the [Resourse Mint Contract](../resource-mint-contract/README.md) and the [Resource Transform Contract](../resource-transform-contract/README.md) to:

- Mint Citizen NFTs using WOOD and FOOD tokens
- Upgrade Citizens to Soldiers using GOLD and ORE tokens
- Upgrade Soldiers with Tool NFTs (Shields and Swords)

## Contract Structure

The contract is organized into several modules:

- [`lib.rs`](src/lib.rs): Main contract implementation with public endpoints for minting and upgrading characters
- [`admin.rs`](src/admin.rs): Admin endpoints for contract configuration
- [`storage.rs`](src/storage.rs): Storage mappers for state management

The contract uses the [Game Common Module](../game-common-module/README.md) to provide common functionality.

## Configuration

Key parameters:

- **NFT Collection**:
  - [Registered](https://docs.multiversx.com/tokens/nft-tokens/#register-and-set-all-roles-to-dynamic) as dynamic NFT
  - Name: `Characters`
  - Ticker: `CHARACTER`
  - Royalties: 5%

- **Resource Requirements**:
  - Citizen Minting: 10 WOOD + 15 FOOD
  - Soldier Upgrade: 5 GOLD + 5 ORE

- **Character Types**:
  - Citizen (Rank 0)
  - Soldier (Rank 1)

## Public Endpoints

### Citizen creation

```rust
#[payable]
#[endpoint(mintCitizen)]
fn mint_citizen(&self, receiver_address: OptionalValue<ManagedAddress>)
```

- Mints a new Citizen NFT
- Requires 10 WOOD and 15 FOOD tokens
- Has a minting period of 3600 seconds (1 hour)
- Optional receiver address can be specified (used in the [Game Interface Contract](../game-interface-contract/README.md))

```rust
#[endpoint(claimCitizen)]
fn claim_citizen(&self, receiver_address: OptionalValue<ManagedAddress>)
```

- Claims a Citizen NFT after the minting period
- Can be claimed by the minter or a specified receiver
- Optional receiver address can be specified (used in the [Game Interface Contract](../game-interface-contract/README.md))

### Citizen upgrade to Soldier

```rust
#[payable]
#[endpoint(upgradeCitizenToSoldier)]
fn upgrade_citizen_to_soldier(
    &self,
    citizen_nft_nonce: u64,
    owner_address: ManagedAddress
)
```

- Upgrades a Citizen to a Soldier
- Requires 5 GOLD and 5 ORE tokens
- Takes the citizen NFT nonce and owner address as parameters

### Soldier upgrade with Tools

```rust
#[payable]
#[endpoint(upgradeSoldier)]
fn upgrade_soldier(&self, owner_address: ManagedAddress)
```

- Upgrades a Soldier NFT with a Tool NFT (Shield or Sword)
- Takes the owner address as parameter
- Requires:
  - 1 Soldier NFT
  - 1 Tool NFT
- Returns the upgraded Soldier NFT to the owner

## NFT Metadata

### Asset Structure

The NFT assets are stored on IPFS with the following structure:

- Base URL: `https://{CID}.ipfs.w3s.link/`
- IPFS CID: `bafybeih3vwnfq7qyvyb5s2ojjk4cs6gcwxzpatujtahpeiap5xu5k4r3pm`

#### Asset Files [(Included)](/character-contract/nft-assets/)

1. **Citizen**:
   - Image: `citizen.png`
   - Metadata: `citizen.json`

2. **Soldier**:
   - Base Images: `soldier{attack}{defence}.png` (for attack/defence 0-2)
   - Same Image: `soldierXX.png` (for higher stats)
   - Metadata: `soldier{attack}{defence}.json`
   - Same Metadata: `soldierXX.json` (for higher stats)

### Attribute Format

NFT attributes are encoded in the following format:

```md
metadata:{IPFS_CID}/{filename}.json;tags:{tag(s)}{PREFIX}{rank}:{attack}:{defence}
```

Examples:

```md
# Citizen
metadata:bafybeih.../citizen.json;tags:character,citizen;c:0:0:0

# Soldier with attack=2, defence=1
metadata:bafybeih.../soldier21.json;tags:character,soldier;c:1:2:1
```

### Character Attributes

Each NFT contains encoded attributes:

- **Rank**: 0 (Citizen) or 1 (Soldier)
- **Attack**: Combat attack value
- **Defence**: Combat defence value

### URIs Structure

Each NFT contains two URIs:

1. Image URI: `https://{CID}.ipfs.w3s.link/{filename}.png`
2. Metadata URI: `https://{CID}.ipfs.w3s.link/{filename}.json`

Where filename is:

- `citizen` for Citizens
- `soldier{attack}{defence}` for Soldiers with attack/defence 0-2
- `soldierXX` for Soldiers with higher attack/defence values

### Metadata Updates

When upgrading a Citizen to a Soldier:

1. Original NFT attributes are read and validated
2. New Soldier attributes are generated
3. NFT metadata is recreated with new attributes and URIs (Dynamic NFT feature)
4. Original NFT nonce remains the same

The metadata recreation is handled by the [ESDTMetaDataRecreate](https://docs.multiversx.com/tokens/nft-tokens/#metadata-recreate) system SC endpoint.

## Error Cases

The contract handles various error cases including:

1. Minting Citizen:
   - Incorrect number of payment tokens
   - Wrong token types
   - Incorrect token amounts

2. Claiming Citizen:
   - No citizens pending to be minted
   - Minting period not elapsed

3. Upgrading to Soldier:
   - Incorrect number of payment tokens
   - Wrong token types
   - Incorrect token amounts

4. Upgrading Soldier:
   - Incorrect number of NFTs
   - Wrong NFT types
   - NFT ownership verification

## How to Use

1. Upload character assets to [IPFS](https://web3.storage/):
   - [Citizen and Soldier images](/character-contract/nft-assets/) (.png)
   - [Metadata files](/character-contract/nft-assets/) (.json)
   - Set the IPFS CID in the [game common module](../game-common-module/src/constants.rs):

   ```rust
   pub const IPFS_CHARACTERS_CID: &str = "bafybeih3vwnfq7qyvyb5s2ojjk4cs6gcwxzpatujtahpeiap5xu5k4r3pm";
   ```

2. Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

    Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

3. Register the Characters NFT Collection by calling the registerCharactersCollection endpoint with 0.05 EGLD:

   ```rust
   #[only_owner]
   #[payable]
   registerCharactersCollection()
   ```

   This will:
   - Register a new NFT collection named "Characters" with ticker "CHARACTER"
   - Set all required roles for the collection
   - Configure NFT properties

4. Users can start minting Citizens by sending the required resources:

   ```rust
   #[payable]
   mintCitizen(
       receiver_address: OptionalValue<ManagedAddress>
   )
   ```

   Required tokens:
   - 10 WOOD
   - 15 FOOD

5. After the minting period (3600 seconds), Citizens can be claimed:

   ```rust
   claimCitizen(
       receiver_address: OptionalValue<ManagedAddress>
   )
   ```

6. Citizens can be upgraded to Soldiers by sending the required resources:

   ```rust
   #[payable]
   upgradeCitizenToSoldier(
       citizen_nft_nonce: u64,
       owner_address: ManagedAddress
   )
   ```

   Required tokens:
   - 5 GOLD
   - 5 ORE

7. Soldiers can be upgraded with Tools:

   ```rust
   #[payable]
   upgradeSoldier(
       owner_address: ManagedAddress
   )
   ```

   Required NFTs:
   - 1 Soldier NFT
   - 1 Tool NFT (Shield or Sword)
