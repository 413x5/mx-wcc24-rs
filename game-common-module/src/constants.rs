/// MultiversX constants
pub const REGISTER_FEE: u64 = 50_000_000_000_000_000; // 0.05 EGLD
/// MultiversX endpoints
pub const REGISTER_AND_SET_ALL_ROLES_DYNAMIC_ENDPOINT_NAME: &str = "registerAndSetAllRolesDynamic";
pub const ESDT_METADATA_RECREATE_ENDPOINT_NAME: &str = "ESDTMetaDataRecreate";
// Game Token tickers
pub const WOOD_TICKER: &str = "WOOD-";
pub const FOOD_TICKER: &str = "FOOD-";
pub const STONE_TICKER: &str = "STONE-";
pub const GOLD_TICKER: &str = "GOLD-";
pub const ORE_TICKER: &str = "ORE-";
/// Game settings
pub const MINT_CITIZEN_WOOD_QUANTITY: u64 = 10;
pub const MINT_CITIZEN_FOOD_QUANTITY: u64 = 15;
pub const STONE_AMMOUNT_FOR_ORE: u64 = 20;
pub const MINT_SHIELD_ORE_QUANTITY: u64 = 2;
pub const MINT_SWORD_GOLD_QUANTITY: u64 = 1;
pub const MINT_SWORD_ORE_QUANTITY: u64 = 3;
pub const CITIZEN_TO_SOLDIER_GOLD_QUANTITY: u64 = 5;
pub const CITIZEN_TO_SOLDIER_ORE_QUANTITY: u64 = 5;
pub const MINT_CITIZEN_SECONDS_DEFAULT: u64 = 3600;
pub const MINT_SHIELD_SECONDS_DEFAULT: u64 = 3600;
pub const MINT_SWORD_SECONDS_DEFAULT: u64 = 3600;
// NFT Collections settings
pub const CHARACTER_COLLECTION_NAME: &str = "Characters";
pub const CHARACTER_COLLECTION_TICKER: &str = "CHARACTER";
pub const TOOLS_COLLECTION_NAME: &str = "Tools";
pub const TOOLS_COLLECTION_TICKER: &str = "TOOLS";
/// NFT names
pub const CITIZEN_NFT_NAME: &str = "Citizen";
pub const SOLDIER_NFT_NAME: &str = "Soldier";
pub const SHIELD_NFT_NAME: &str = "Shield";
pub const SWORD_NFT_NAME: &str = "Sword";
/// NFT royalties
pub const CHARACTER_NFT_ROYALTIES: u64 = 500; // 5%
pub const SHIELD_NFT_ROYALTIES: u64 = 500; // 5%
pub const SWORD_NFT_ROYALTIES: u64 = 500; // 5%
/// NFT IPFS CIDs
pub const IPFS_CHARACTERS_CID: &str = "bafybeih3vwnfq7qyvyb5s2ojjk4cs6gcwxzpatujtahpeiap5xu5k4r3pm";
pub const IPFS_TOOLS_CID: &str = "bafybeieysc7cv3cgwfdjdhujmmvscca4h67mbidbnbfzchyad4lib2ocpu";
/// NFT tags
pub const CITIZEN_NFT_TAGS : &str = "character,citizen";
pub const SOLDIER_NFT_TAGS : &str = "character,soldier";
pub const SHIELD_NFT_TAGS : &str = "tool,shield";
pub const SWORD_NFT_TAGS : &str = "tool,sword";
/// NFT Assets files
pub const CITIZEN_FILE_NAME : &str = "citizen";
pub const SOLDIER_FILE_NAME : &str = "soldier";
pub const SHIELD_FILE_NAME : &str = "shield";
pub const SWORD_FILE_NAME : &str = "sword";
// NFT Assets files extensions
pub const NFT_IMAGE_FILE_EXTENSION: &str = "png";
pub const NFT_METADATA_FILE_EXTENSION: &str = "json";
/// NFT attributes prefixes
pub const NFT_CHARACTER_ATTRIBUTES_PREFIX: &str = ";c:";
pub const NFT_TOOL_ATTRIBUTES_PREFIX: &str = ";t:";
/// Game contracts endpoints
pub const RESOURCE_CONTRACT_MINT_RESOURCES_ENDPOINT_NAME: &str = "mintResources";
pub const RESOURCE_CONTRACT_CLAIM_RESOURCES_ENDPOINT_NAME: &str = "claimResources";
pub const RESOURCE_TRANSFORM_CONTRACT_CREATE_ORE_ENDPOINT_NAME: &str = "createOre";
pub const CHARACTER_CONTRACT_MINT_CITIZEN_ENDPOINT_NAME: &str = "mintCitizen";
pub const CHARACTER_CONTRACT_CLAIM_CITIZEN_ENDPOINT_NAME: &str = "claimCitizen";
pub const CHARACTER_CONTRACT_UPGRADE_CITIZEN_TO_SOLDIER_ENDPOINT_NAME: &str = "upgradeCitizenToSoldier";
pub const CHARACTER_CONTRACT_UPGRADE_SOLDIER_ENDPOINT_NAME: &str = "upgradeSoldier";
pub const TOOLS_CONTRACT_MINT_SHIELD_ENDPOINT_NAME: &str = "mintShield";
pub const TOOLS_CONTRACT_MINT_SWORD_ENDPOINT_NAME: &str = "mintSword";
pub const TOOLS_CONTRACT_CLAIM_SHIELD_ENDPOINT_NAME: &str = "claimShield";
pub const TOOLS_CONTRACT_CLAIM_SWORD_ENDPOINT_NAME: &str = "claimSword";
pub const GAME_ARENA_CONTRACT_CREATE_GAME_ENDPOINT_NAME: &str = "createGame";
pub const GAME_ARENA_CONTRACT_ACCEPT_GAME_ENDPOINT_NAME: &str = "acceptGame";
