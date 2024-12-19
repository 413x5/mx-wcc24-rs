# Token Manager Contract

A MultiversX smart contract that allows users to issue and manage SNOW tokens.

## Features

### Token Issuance

- Users can issue SNOW tokens by paying a 0.05 EGLD issue fee
- Each issued token will have:
  - Name: Custom name (defaults to "JohnSnow" if not specified)
  - Ticker: "SNOW-[xxxxxx]"
  - Decimals: 8
- Any excess EGLD payment is returned to the caller

### Token Management

- Token holders can burn any amount of their SNOW tokens
- Only token issuers can burn their issued tokens
- Token issuers can claim any available tokens of their issued tokens
- The total supply held by the contract is automatically updated after burning or claiming

### View Functions

- `getIssuedTokensInfo`: Get all tokens issued by a specific address with their balances

## Contract Endpoints

### Issue Token

```rust
#[payable("EGLD")]
#[endpoint(issue_token_snow)]
fn issue_token_snow(&self, token_amount: BigUint, token_name: OptionalValue<ManagedBuffer>)
```

- Requires minimum 0.05 EGLD payment
- `token_amount`: The amount of tokens to issue (will be multiplied by 10^8 for decimals)
- `token_name`: Optional token name (if not provided, defaults to "JohnSnow")

### Burn Token

```rust
#[endpoint(burn_tokens)]
fn burn_tokens(&self, token_id: TokenIdentifier, amount: BigUint)
```

- Only the token issuer can burn their issued tokens
- `token_id`: The identifier of the SNOW token to burn
- `amount`: The amount of tokens to burn

### Claim Tokens

```rust
#[endpoint(claim_tokens)]
fn claim_tokens(&self, token_id: TokenIdentifier, amount: BigUint)
```

- Only the token issuer can claim their issued tokens
- Claims the specified amount of tokens with the token ID
- `token_id`: The identifier of the SNOW token to claim
- `amount`: The amount of tokens to claim

## Storage

- `token_balances`: Maps token identifiers to their total supply
- `token_issuers`: Maps token identifiers to their issuer addresses

## How to Use

Build and Deploy the contract following the [instructions](../README.md#building-the-contracts)

Use the [MultiversX Utility App](https://utils.multiversx.com/) `Read endpoints` and `Write endpoints` tabs to interact with the contract.

1. To issue tokens:
   - Call `issue_token_snow` with desired amount and optional token name
   - Send at least 0.05 EGLD. Extra EGLD will be returned.
   - Specify 100000000 in the `Contract Transaction Gas Limit` field to have enouogh gas for the issue transaction

2. To burn tokens:
   - Call `burn_tokens` with the token ID and amount to burn
   - Only the token issuer can burn their tokens

3. To claim tokens:
   - Call `claim_tokens` with the token ID
   - Only the token issuer can claim their tokens

4. To query information:
   - Use `getIssuedTokensInfo` with an address to see all tokens issued by that address

## Implementation

See: [src/token_manager.rs](src/token_manager.rs)
