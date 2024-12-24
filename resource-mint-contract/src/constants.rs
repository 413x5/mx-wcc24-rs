// Constants
pub const ISSUE_FEE: u64 = 50_000_000_000_000_000; // 0.05 EGLD
pub const RESOURCE_TOKEN_DECIMALS: usize = 0; // In this case resources don't have decimals

// Error messages
pub const ERR_RESOURCE_TOKEN_ALREADY_ISSUED: &str = "Resource token already issued.";
pub const ERR_TOKEN_NAME_EMPTY: &str = "Token name must not be empty.";
pub const ERR_TOKEN_TICKER_EMPTY: &str = "Token ticker must not be empty.";
pub const ERR_INSUFFICIENT_ISSUE_COST: &str = "Must send 0.05 EGLD for issue cost.";
pub const ERR_INITIAL_SUPPLY_ZERO: &str = "Initial supply to issue must be greater than 0.";
pub const ERR_RESOURCE_TOKEN_NOT_ISSUED: &str = "Resource token not issued.";
pub const ERR_CONTRACT_NO_MINT_ROLE: &str = "Contract does not have mint role.";
pub const ERR_MINT_ROUNDS_INTERVAL_ZERO: &str = "Mint rounds interval must be greater than 0.";
pub const ERR_MINT_STAKE_THRESHOLD_ZERO: &str = "Mint stake threshold must be greater than 0.";
pub const ERR_STAKE_TOKEN_NOT_SET: &str = "Stake token not set.";
pub const ERR_NO_ESDT_TOKENS_RECEIVED: &str = "No ESDT tokens received.";
pub const ERR_INVALID_STAKE_TOKEN: &str = "Sent tokens are not valid for staking.";
