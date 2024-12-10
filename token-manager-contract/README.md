# Token Manager Contract

A MultiversX smart contract that allows users to issue and manage SNOW tokens.

## Features

### Token Issuance
- Users can issue SNOW tokens by paying 0.05 EGLD
- Each issued token will have:
  - Name: "JohnSnow"
  - Ticker: "SNOW-xxxxxx"
  - Decimals: 8
- The newly issued tokens are automatically sent to the issuer
- Any excess EGLD payment is returned to the caller

### Token Burning
- Token holders can burn any amount of their SNOW tokens
- Only tokens issued through this contract can be burned
- The total supply is automatically updated after burning

### View Functions
- `getAllIssuedTokensInfo`: Get all issued tokens with their balances and issuers
- `getIssuedTokensInfo`: Get all tokens issued by a specific address with their balances

## Contract Endpoints

### Issue Token
```rust
#[payable("EGLD")]
#[endpoint(issue_token_snow)]
fn issue_token_snow(&self, token_amount: BigUint)
```
- Requires minimum 0.05 EGLD payment
- `token_amount`: The amount of tokens to issue (will be multiplied by 10^8 for decimals)

### Burn Token
```rust
#[payable("*")]
#[endpoint(burn_tokens)]
fn burn_tokens(&self)
```
- Accepts any SNOW token issued through this contract
- Burns the entire amount sent to the contract

## Storage
- `token_balances`: Tracks the total supply of each issued token
- `token_issuers`: Maps token identifiers to their issuer addresses

## How to Use

Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

1. To issue tokens:
   - Call `issue_token_snow` with desired amount
   - Send at least 0.05 EGLD
   - Receive newly issued tokens automatically in your wallet

2. To burn tokens:
   - Call `burn_tokens`
   - Send an amount of the SNOW tokens you want to burn

3. To query information:
   - Use `getAllIssuedTokensInfo` to see all issued tokens
   - Use `getIssuedTokensInfo` with your address to see your issued tokens

## Implementation
See: [src/token-manager.rs](src/token-manager.rs)