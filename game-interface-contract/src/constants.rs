/// Error messages
pub const ERR_NO_ESDT_TOKENS_RECEIVED: &str = "No tokens received.";
pub const ERR_CHARACTER_CONTRACT_ADDRESS_NOT_SET: &str = "Character contract address not set.";
pub const ERR_RESOURCE_TRANSFORM_CONTRACT_ADDRESS_NOT_SET: &str = "Resource transform contract address not set.";
pub const ERR_TOOLS_CONTRACT_ADDRESS_NOT_SET: &str = "Tools contract address not set.";
pub const ERR_RESOURCE_MINT_CONTRACT_ADDRESS_NOT_SET: &str = "Resource mint contract address not set.";
pub const ERR_CHARACTER_COLLECTION_NOT_SET: &str = "Character collection id not set.";
pub const ERR_TOOLS_COLLECTION_NOT_SET: &str = "Tools collection id not set.";
/// Token tickers
pub const WOOD_TOKEN_TICKER: &str = "WOOD-";
pub const FOOD_TOKEN_TICKER: &str = "FOOD-";
pub const STONE_TOKEN_TICKER: &str = "STONE-";
pub const GOLD_TOKEN_TICKER: &str = "GOLD-";
pub const ORE_TOKEN_TICKER: &str = "ORE-";
/// Game contracts endpoints
pub const RESOURCE_CONTRACT_MINT_RESOURCES_ENDPOINT_NAME: &str = "mintResources";
pub const RESOURCE_CONTRACT_CLAIM_RESOURCES_ENDPOINT_NAME: &str = "claimResources";
pub const RESOURCE_TRANSFORM_CONTRACT_CREATE_ORE_ENDPOINT_NAME: &str = "createOre";
pub const CHARACTER_CONTRACT_MINT_CITIZEN_ENDPOINT_NAME: &str = "mintCitizen";
pub const CHARACTER_CONTRACT_CLAIM_CITIZEN_ENDPOINT_NAME: &str = "claimCitizen";
pub const CHARACTER_CONTRACT_UPGRADE_CITIZEN_TO_SOLDIER_ENDPOINT_NAME: &str = "upgradeCitizenToSoldier";
pub const TOOLS_CONTRACT_MINT_SHIELD_ENDPOINT_NAME: &str = "mintShield";
pub const TOOLS_CONTRACT_MINT_SWORD_ENDPOINT_NAME: &str = "mintSword";
pub const TOOLS_CONTRACT_CLAIM_SHIELD_ENDPOINT_NAME: &str = "claimShield";
pub const TOOLS_CONTRACT_CLAIM_SWORD_ENDPOINT_NAME: &str = "claimSword";
pub const CHARACTER_CONTRACT_UPGRADE_SOLDIER_ENDPOINT_NAME: &str = "upgradeSoldier";
/// Game settings
pub const MINT_CITIZEN_WOOD_QUANTITY: u64 = 10;
pub const MINT_CITIZEN_FOOD_QUANTITY: u64 = 15;
pub const STONE_AMMOUNT_FOR_ORE: u64 = 20;
pub const UPGRADE_CITIZEN_TO_SOLDIER_GOLD_QUANTITY: u64 = 5;
pub const UPGRADE_CITIZEN_TO_SOLDIER_ORE_QUANTITY: u64 = 5;
pub const MINT_SHIELD_ORE_QUANTITY: u64 = 2;
pub const MINT_SWORD_ORE_QUANTITY: u64 = 3;
pub const MINT_SWORD_GOLD_QUANTITY: u64 = 1;