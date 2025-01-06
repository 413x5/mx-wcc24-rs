// MultiversX constants
pub const REGISTER_FEE: u64 = 50_000_000_000_000_000; // 0.05 EGLD
// MultiversX endpoints
pub const REGISTER_AND_SET_ALL_ROLES_DYNAMIC_ENDPOINT_NAME: &str = "registerAndSetAllRolesDynamic";
pub const ESDT_METADATA_RECREATE_ENDPOINT_NAME: &str = "ESDTMetaDataRecreate";
// Token tickers
pub const WOOD_TICKER: &str = "WOOD-";
pub const FOOD_TICKER: &str = "FOOD-";
pub const STONE_TICKER: &str = "STONE-";
pub const GOLD_TICKER: &str = "GOLD-";
pub const ORE_TICKER: &str = "ORE-";
// Game settings
pub const MINT_CITIZEN_WOOD_QUANTITY: u64 = 10;
pub const MINT_CITIZEN_FOOD_QUANTITY: u64 = 15;
pub const MINT_CITIZEN_SECONDS: u64 = 3600;
pub const CITIZEN_TO_SOLDIER_GOLD_QUANTITY: u64 = 5;
pub const CITIZEN_TO_SOLDIER_ORE_QUANTITY: u64 = 5;
// NFT Collection settings
pub const CHARACTER_COLLECTION_NAME: &str = "Characters";
pub const CHARACTER_COLLECTION_TICKER: &str = "CHARACTER";
pub const NFT_NAME_DEFAULT: &str = "Character";
pub const NFT_NAME_CITIZEN: &str = "Citizen";
pub const NFT_NAME_SOLDIER: &str = "Soldier";
pub const NFT_ROYALTIES: u64 = 500; // 5%
// NFT Assets settings
pub const IPFS_CID: &str = "bafybeih3vwnfq7qyvyb5s2ojjk4cs6gcwxzpatujtahpeiap5xu5k4r3pm";
pub const NFT_CHARACTER_ATTRIBUTES_PREFIX: &str = ";c:";
pub const CITIZEN_FILE_NAME : &str = "citizen";
pub const CITIZEN_NFT_TAGS : &str = "character,citizen";
pub const SOLDIER_FILE_NAME : &str = "soldier";
pub const SOLDIER_NFT_TAGS : &str = "character,soldier";
// No dot for file extensions
pub const NFT_IMAGE_FILE_EXTENSION: &str = "png";
pub const NFT_METADATA_FILE_EXTENSION: &str = "json";