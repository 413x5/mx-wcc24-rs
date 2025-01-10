#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod admin;
pub mod storage;
pub mod views;
pub mod constants;

use game_common_module::data::*;
use game_common_module::constants::*;


#[multiversx_sc::contract]
pub trait CharacterContract:
    admin::AdminModule +
    storage::StorageModule +
    views::ViewsModule +
    game_common_module::GameCommonModule +
    game_common_module::nft_attributes::NftAttributesDecodeModule
{

    /// Mints a Citizen NFT
    #[payable]
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self, receiver_address: OptionalValue<ManagedAddress>) {
        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 2, "Endpoint requires 2 payment tokens, Wood and Food.");

        let mut wood_amount = BigUint::zero();
        let mut food_amount = BigUint::zero();

        // Check the wood and food required
        for payment in payments.iter() {
            let token_id = &payment.token_identifier;
            if self.is_required_token_str(&token_id, WOOD_TICKER) { wood_amount = payment.amount.clone(); }
            if self.is_required_token_str(&token_id, FOOD_TICKER) { food_amount = payment.amount.clone(); }         
        }

        require!(wood_amount == MINT_CITIZEN_WOOD_QUANTITY, "Wood amount sent must be {}.", MINT_CITIZEN_WOOD_QUANTITY);
        require!(food_amount == MINT_CITIZEN_FOOD_QUANTITY, "Food amount sent must be {}.", MINT_CITIZEN_FOOD_QUANTITY);

        // Determine the receiver address if one is specified
        let user = match receiver_address {
                OptionalValue::Some(address) => address,
                OptionalValue::None => self.blockchain().get_caller(),
            };

        // Register the NFT mint to the receiver address
        let mut user_citizens_to_mint = self.citizens_to_mint().get(&user).unwrap_or_default();
        // Record the current mint start timestamp
        let mint_start_timestamp = self.blockchain().get_block_timestamp();
        // Add the mint start timestamp to the mint user's list
        user_citizens_to_mint.push(mint_start_timestamp);
        self.citizens_to_mint().insert(user.clone(), user_citizens_to_mint);
        
        // Burn the wood and food sent
        for payment in payments.iter() {
            self.send().esdt_local_burn(&payment.token_identifier, 0, &payment.amount);
        }
    }


    /// Claims a Citizen NFT if the minting period is over
    #[endpoint(claimCitizen)]
    fn claim_citizen(&self, receiver_address: OptionalValue<ManagedAddress>) {

        // Determine the receiver address if one is specified
        let user = match receiver_address {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };

        // Check if the user has any NFTs to mint
        let user_citizens = self.citizens_to_mint().get(&user).unwrap_or_default();
        let citizens_pending_count = user_citizens.len();

        // Exit with an error if the user has no NFTs to mint
        require!(citizens_pending_count > 0, "No citizens pending to be minted.");

        let mut citizens_minted = 0;

        let mut still_minting : ManagedVec<u64> = ManagedVec::new();

        let mint_citizen_seconds = if self.mint_citizen_seconds().is_empty() { MINT_CITIZEN_SECONDS_DEFAULT }
            else { self.mint_citizen_seconds().get() };

        // Find mintable citizens
        for timestamp in user_citizens.iter() {
        
            // Check if the minting period has elapsed
            if self.blockchain().get_block_timestamp() - timestamp < mint_citizen_seconds {
                still_minting.push(timestamp);
                continue;
            }

            // Mint the NFT
            let nft_nonce = self.create_citizen_nft();

            citizens_minted += 1;

         // Transfer the NFT to the user
            self.send().direct_esdt(
                &user,
                &self.characters_nft_collection().get_token_id(),
                nft_nonce,
                &BigUint::from(1u64),
            );
        }

        // Update the user's mint list
        if  still_minting.is_empty() {
            self.citizens_to_mint().remove(&user);
        } else {
            self.citizens_to_mint().insert(user, still_minting);
        }

        // Check if any NFTs were minted and exit with an error if still in the minting period
        if citizens_minted == 0 { sc_panic!("{} citizen(s) still in the minting period.", citizens_pending_count); }

    }


    #[payable]
    #[endpoint(upgradeCitizenToSoldier)]
    fn upgrade_citizen_to_soldier(&self, citizen_nft_nonce: u64, owner_address: ManagedAddress) {
        self.require_character_collection();

        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 2, "Endpoint requires 2 payment tokens, Gold and Ore.");

        let mut gold_amount = BigUint::zero();
        let mut ore_amount = BigUint::zero();

        // Check the gold and ore required
        for payment in payments.iter() {
            let token_id = &payment.token_identifier;

            if self.is_required_token_str(&token_id, GOLD_TICKER) { gold_amount = payment.amount.clone(); }
            if self.is_required_token_str(&token_id, ORE_TICKER) { ore_amount = payment.amount.clone(); }
        }

        require!(gold_amount == CITIZEN_TO_SOLDIER_GOLD_QUANTITY, "Gold amount must be {}.", CITIZEN_TO_SOLDIER_GOLD_QUANTITY);
        require!(ore_amount == CITIZEN_TO_SOLDIER_ORE_QUANTITY, "Ore amount must be {}.", CITIZEN_TO_SOLDIER_ORE_QUANTITY);
        
        // Upgrade the NFT
        self.upgrade_citizen_to_soldier_nft(citizen_nft_nonce, owner_address);

        // Burn the gold and ore sent
        for payment in payments.iter() {
            self.send().esdt_local_burn(&payment.token_identifier, 0, &payment.amount);
        }

    }

    /// Upgrades a Soldier NFT with a Tool NFT
    #[payable]
    #[endpoint(upgradeSoldier)]
    fn upgrade_soldier(&self, owner_address: ManagedAddress) {
        self.require_character_collection();
        self.require_tools_collection();

        let transfers = self.call_value().all_esdt_transfers();
        require!(transfers.len() == 2, "Endpoint requires 2 transfers, a Character NFT and a Tool NFT.");

        let mut character_nft_nonce = 0;
        let mut tool_nft_nonce = 0;
        let mut character_nft_count = BigUint::zero();
        let mut tool_nft_count = BigUint::zero();

        // Check the NFTs required
        for transfer in transfers.iter() {
            let token_id = &transfer.token_identifier;

            if self.is_required_token(&token_id, &self.characters_nft_collection().get_token_id().as_managed_buffer()) {
                character_nft_nonce = transfer.token_nonce;
                character_nft_count = transfer.amount.clone(); 
            }
            if self.is_required_token(&token_id, &self.tools_nft_collection().get().as_managed_buffer()) {
                tool_nft_nonce = transfer.token_nonce;
                tool_nft_count = transfer.amount.clone(); 
            }
        }

        require!(character_nft_count == 1, "No Soldier NFT received.");
        require!(tool_nft_count == 1, "No Tool NFT received.");

        // Upgrade the NFT
        self.upgrade_soldier_nft(character_nft_nonce, tool_nft_nonce);

        // Send the soldier NFT back to the owner
        self.send().direct_esdt(
            &owner_address,
            &self.characters_nft_collection().get_token_id(),
            character_nft_nonce,
            &BigUint::from(1u64),
        );


    }

    // Private functions

    /// Creates a Citizen NFT
    fn create_citizen_nft(&self) -> u64 {
        self.require_character_collection();

        // Create new citizen character
        let new_citizen = Character::new_citizen();

        // Get the last minted NFT nonce
        let last_minted_nft_nonce = 
            if self.last_minted_nft_nonce().is_empty() { 0 } 
            else { self.last_minted_nft_nonce().get() };

        // Get the collection ID
        let collection_id = self.characters_nft_collection().get_token_id();

        // Set the amount to mint to 1 NFT
        let amount = BigUint::from(1u64);

        // Set the NFT name
        let nft_name = sc_format!("{} {}",
            ManagedBuffer::from(CITIZEN_NFT_NAME.as_bytes()),
            &(last_minted_nft_nonce + 1));

        // Set the royalties
        let royalties = BigUint::from(CHARACTER_NFT_ROYALTIES);

        // Get the attributes
        let attributes = self.get_nft_attributes(&new_citizen);

        // Get the URIs
        let uris = self.get_nft_asset_uris(&new_citizen);

        // Get the attributes hash
        let attributes_sha256 = self.crypto().sha256(&attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();

        // Mint the NFT
        let nft_nonce = self.send()
            .esdt_nft_create(
                &collection_id, 
                &amount, 
                &nft_name, 
                &royalties, 
                &attributes_hash, 
                &attributes, 
                &uris);

        self.last_minted_nft_nonce().set(nft_nonce);

        nft_nonce
    }

    /// Upgrades a Citizen NFT to a Soldier NFT
    fn upgrade_citizen_to_soldier_nft(&self, citizen_nft_nonce: u64, owner_address: ManagedAddress) {
        self.require_character_collection();

        // Get the character NFT data
        let character_nft_data = self.blockchain().get_esdt_token_data(&owner_address, &self.characters_nft_collection().get_token_id(), citizen_nft_nonce);

        // Get the NFT attributes
        let nft_attributes = character_nft_data.attributes;
        require!(!nft_attributes.is_empty(), "Cannot get NFT attributes. Is the NFT owner address correct?");

        // Decode the NFT attributes
        let character = self.decode_character(nft_attributes);

        // Check if the NFT is a citizen
        require!(character.is_citizen(), "Character is not a citizen");

        // Create new soldier character
        let soldier = Character::new_soldier();

        // Create new NFT name
        let new_nft_name = sc_format!("{} {}",
            ManagedBuffer::from(SOLDIER_NFT_NAME.as_bytes()),
            citizen_nft_nonce);

        // Get new NFT attributes
        let new_attributes = self.get_nft_attributes(&soldier);
        // Get new NFT URIs
        let new_uris = self.get_nft_asset_uris(&soldier);

        // Calculate new NFT attributes hash
        let new_attributes_sha256 = self.crypto().sha256(&new_attributes);
        let new_attributes_hash = new_attributes_sha256.as_managed_buffer();

        // Recreate the NFT with the new attributes
        let mut tx = self.tx()
            .to(self.blockchain().get_sc_address())
            .raw_call(ESDT_METADATA_RECREATE_ENDPOINT_NAME)
            .argument(&self.characters_nft_collection().get_token_id())
            .argument(&citizen_nft_nonce)
            .argument(&new_nft_name)
            .argument(&character_nft_data.royalties)
            .argument(&new_attributes_hash)
            .argument(&new_attributes);
            // Add the new URIs
            for uri in new_uris.iter() {
                tx = tx.argument(&uri);
            }
        // Send the transaction and wait for completion (sync call)
        tx.sync_call();
    }

    /// Upgrades a Soldier NFT
    fn upgrade_soldier_nft(&self, character_nft_nonce: u64, tool_nft_nonce: u64) {
        self.require_character_collection();
        self.require_tools_collection();

        // The owner address is the SC address since the NFTs are sent to the SC
        let owner_address = self.blockchain().get_sc_address();

        // Get the character NFT data
        let character_nft_data = self.blockchain().get_esdt_token_data(&owner_address, &self.characters_nft_collection().get_token_id(), character_nft_nonce);

        // Get the NFT attributes
        let character_nft_attributes = character_nft_data.attributes;
        require!(!character_nft_attributes.is_empty(), "Cannot get character NFT attributes. Is the NFT owner address correct?");

        // Decode the NFT attributes
        let character = self.decode_character(character_nft_attributes);

        // Check if the character is a soldier
        require!(character.is_soldier(), "Character is not a soldier");

        // Character is an upgradable soldier
        let mut soldier = character;

        // Get the tool NFT data
        let tool_nft_data = self.blockchain().get_esdt_token_data(&owner_address, &self.tools_nft_collection().get(), tool_nft_nonce);

        // Get the NFT attributes
        let tool_nft_attributes = tool_nft_data.attributes;
        require!(!tool_nft_attributes.is_empty(), "Cannot get tool NFT attributes. Is the NFT owner address correct?");

        // Decode the NFT attributes
        let tool = self.decode_tool(tool_nft_attributes);

        // Upgrade the soldier
        soldier.upgrade(&tool);

        // NFT name
        let new_nft_name = sc_format!("{} {}",
            ManagedBuffer::from(SOLDIER_NFT_NAME.as_bytes()),
            character_nft_nonce);

        // Get new NFT attributes
        let new_attributes = self.get_nft_attributes(&soldier);
        // Get new NFT URIs
        let new_uris = self.get_nft_asset_uris(&soldier);

        // Calculate new NFT attributes hash
        let new_attributes_sha256 = self.crypto().sha256(&new_attributes);
        let new_attributes_hash = new_attributes_sha256.as_managed_buffer();

        // Recreate the NFT with the new attributes
        let mut tx = self.tx()
            .to(self.blockchain().get_sc_address())
            .raw_call(ESDT_METADATA_RECREATE_ENDPOINT_NAME)
            .argument(&self.characters_nft_collection().get_token_id())
            .argument(&character_nft_nonce)
            .argument(&new_nft_name)
            .argument(&character_nft_data.royalties)
            .argument(&new_attributes_hash)
            .argument(&new_attributes);
            // Add the new URIs
            for uri in new_uris.iter() {
                tx = tx.argument(&uri);
            }
        // Send the transaction and wait for completion (sync call)
        tx.sync_call();
    }

    /// Require that the character collection is issued
    fn require_character_collection(&self) {
        require!(!self.characters_nft_collection().is_empty(), "Character collection not issued");
    }

    /// Require that the tools collection is set
    fn require_tools_collection(&self) {
        require!(!self.tools_nft_collection().is_empty(), "Tools collection not set");
    }

    /// Encode nft attributes in the format: metadata:IPFS_CID/{filename}.json;tags:{tag(s)}{PREFIX}{rank}:{attack}:{defence}
    /// Ex: metadata:IPFS_CID/citizen.json;tags:character,citizen;c:0:0:0
    /// Ex: metadata:IPFS_CID/soldier21.json;tags:character,soldier;c:1:2:1
    fn get_nft_attributes(&self, character: &Character) -> ManagedBuffer {
        let nft_attributes = ManagedBuffer::from(
            sc_format!("metadata:{}/{}.{};tags:{}{}{}:{}:{}",
            ManagedBuffer::from(IPFS_CHARACTERS_CID),
            self.get_asset_filename(character),
            ManagedBuffer::from(NFT_METADATA_FILE_EXTENSION), 
            self.get_nft_tags(character),
            ManagedBuffer::from(NFT_CHARACTER_ATTRIBUTES_PREFIX), 
            character.rank, 
            character.attack, 
            character.defence));
        nft_attributes
    }

    /// Get the URIs for the NFT assets (image, metadata)
    fn get_nft_asset_uris(&self, character: &Character) -> ManagedVec<ManagedBuffer> {
        // Get the base filename
        let asset_base_filename = 
            //sc_format!("https://ipfs.io/ipfs/{}/{}", // This IPFS gateway timeouts
            sc_format!("https://{}.ipfs.w3s.link/{}",  // New IPFS gateway
            ManagedBuffer::from(IPFS_CHARACTERS_CID), // IPFS CID
            self.get_asset_filename(character)
        );
        // Get the image and metadata URIs by adding the file extension
        let asset_image = sc_format!("{}.{}", asset_base_filename, ManagedBuffer::from(NFT_IMAGE_FILE_EXTENSION));
        let asset_metadata = sc_format!("{}.{}", asset_base_filename, ManagedBuffer::from(NFT_METADATA_FILE_EXTENSION));
        
        // Return the URIs
        let mut assets = 
            ManagedVec::from_single_item(asset_image);
            assets.push(asset_metadata);

        assets
    }

    /// Get the NFT tags based on the character
    fn get_nft_tags(&self, character: &Character) -> ManagedBuffer {
        if character.is_citizen() { return ManagedBuffer::from(CITIZEN_NFT_TAGS) }
        if character.is_soldier() { return ManagedBuffer::from(SOLDIER_NFT_TAGS) }
        sc_panic!("Invalid character rank {}.", character.rank);
    }

    /// Get the asset filename based on the character
    fn get_asset_filename(&self, character: &Character) -> ManagedBuffer {
        // One image and metadata for citizen
        if character.is_citizen() { return ManagedBuffer::from(CITIZEN_FILE_NAME) }
        if character.is_soldier() {
            // Different soldier images and metadata available for attack and defence from 0 to 2
            if character.attack <= 2 && character.defence <= 2 {
                return sc_format!("{}{}{}", ManagedBuffer::from(SOLDIER_FILE_NAME), character.attack, character.defence)

            // If attack or defence is greater than 2, the assets remain the same, only the NFT attributes are different
            } else {
                return sc_format!("{}XX", ManagedBuffer::from(SOLDIER_FILE_NAME))
            }
        }
        sc_panic!("Invalid character rank {}.", character.rank);
    }


}