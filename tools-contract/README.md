# Tools Contract

A smart contract that manages tool NFTs for the MultiversX blockchain game. The contract handles minting and managing Shields and Swords, which can be used to upgrade Soldier NFTs.

## Contract Structure

- [`lib.rs`](src/lib.rs): Main contract implementation
- [`storage.rs`](src/storage.rs): Storage mappers for contract state
- [`admin.rs`](src/admin.rs): Admin endpoints for contract configuration

The contract uses the [Game Common Module](../game-common-module/README.md) to provide common functionality.

## NFT Collections

### Tools Collection

The contract manages a single NFT collection for tools (Shields and Swords).

#### Collection Settings

- Name: "Tools"
- Ticker: "TOOLS"
- Royalties: 5%

#### IPFS Storage

- Base URL: `https://{CID}.ipfs.w3s.link/`
- IPFS CID: `bafybeieysc7cv3cgwfdjdhujmmvscca4h67mbidbnbfzchyad4lib2ocpu`

#### Asset Files [(Included)](/tools-contract/nft-assets/)

1. **Shield**:
   - Image: `shield.png`
   - Metadata: `shield.json`
   - Tags: "tool,shield"

2. **Sword**:
   - Image: `sword.png`
   - Metadata: `sword.json`
   - Tags: "tool,sword"

## Public Endpoints

### Mint Shield

```rust
#[payable]
#[endpoint(mintShield)]
fn mint_shield(&self, receiver_address: OptionalValue<ManagedAddress>)
```

Starts the Shield NFT minting process:

- Requires: 2 ORE tokens
- Cooldown: 1 hour between mints
- Optional: Specify a receiver address (used in the [Game Interface](../game-interface-contract/README.md) contract)
- Burns the ORE tokens and registers the mint timestamp

### Claim Shield

```rust
#[endpoint(claimShield)]
fn claim_shield(&self, receiver_address: OptionalValue<ManagedAddress>)
```

Claims any ready Shield NFTs for the caller:

- Checks if minting period (1 hour) has passed
- Optional: Specify a receiver address (used in the [Game Interface](../game-interface-contract/README.md) contract)
- Mints and transfers the Shield NFT to the receiver
- Multiple shields can be claimed at once if ready

### Mint Sword

```rust
#[payable]
#[endpoint(mintSword)]
fn mint_sword(&self, receiver_address: OptionalValue<ManagedAddress>)
```

Starts the Sword NFT minting process:

- Requires: 1 GOLD token and 3 ORE tokens
- Cooldown: 1 hour between mints
- Optional: Specify a receiver address (used in the [Game Interface](../game-interface-contract/README.md) contract)
- Burns the tokens and registers the mint timestamp

### Claim Sword

```rust
#[endpoint(claimSword)]
fn claim_sword(&self, receiver_address: OptionalValue<ManagedAddress>)
```

Claims any ready Sword NFTs for the caller:

- Checks if minting period (1 hour) has passed
- Optional: Specify a receiver address
- Mints and transfers the Sword NFT to the receiver
- Multiple swords can be claimed at once if ready

## Admin Endpoints

### Register Tools Collection

```rust
#[only_owner]
#[payable]
#[endpoint(registerToolsCollection)]
fn register_tools_collection(&self)
```

Registers and configures the Tools NFT collection:

- Requires: 0.05 EGLD for registration
- Sets collection name, ticker, and type
- Configures all roles automatically
- Can only be called once

### Set Mint Shield Seconds

```rust
#[only_owner]
#[endpoint(setMintShieldSeconds)]
fn set_mint_shield_seconds(&self, mint_shield_seconds: u64)
```

Sets the time required to mint a Shield NFT:

- Default: 3600 seconds (1 hour)
- Can be adjusted by owner

### Set Mint Sword Seconds

```rust
#[only_owner]
#[endpoint(setMintSwordSeconds)]
fn set_mint_sword_seconds(&self, mint_sword_seconds: u64)
```

Sets the time required to mint a Sword NFT:

- Default: 3600 seconds (1 hour)
- Can be adjusted by owner

## How to Use

1. Upload tool assets to [IPFS](https://web3.storage/):
   - [Shield and Sword images](/tools-contract/nft-assets/) (.png)
   - [Metadata files](/tools-contract/nft-assets/) (.json)
   - Set the IPFS CID in the [game common module](../game-common-module/src/constants.rs):

   ```rust
   pub const IPFS_TOOLS_CID: &str = "bafybeieysc7cv3cgwfdjdhujmmvscca4h67mbidbnbfzchyad4lib2ocpu";
   ```

2. Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

    Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

3. Register the Tools NFT Collection:
   - Call `registerToolsCollection` with 0.05 EGLD to create the Tools collection
   - Optionally adjust minting times using `setMintShieldSeconds` and `setMintSwordSeconds`

4. Mint and claim tools:
   - Call `mintShield` with 2 ORE tokens to start Shield minting
   - Wait 1 hour, then call `claimShield` to receive the Shield NFT
   - Call `mintSword` with 1 GOLD and 3 ORE tokens to start Sword minting
   - Wait 1 hour, then call `claimSword` to receive the Sword NFT

5. Use tools to upgrade Soldiers:
   - You can transfer tool NFTs to the [Game Interface](../game-interface-contract/README.md) contract
   - Call the `upgrade_soldier` endpoint to enhance Soldier attributes:
     - Shield: +1 Defence
     - Sword: +1 Attack
