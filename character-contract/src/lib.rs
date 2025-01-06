#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod data;
pub mod admin;
pub mod storage;
pub mod views;
pub mod constants;

use constants::*;
use data::*;


#[multiversx_sc::contract]
pub trait CharacterContract:
    admin::AdminModule +
    storage::StorageModule +
    views::ViewsModule
{

    /// Mints a Citizen NFT
    #[payable("*")]
    #[endpoint(mintCitizen)]
    fn mint_citizen(&self, receiver_address: OptionalValue<ManagedAddress>) {
        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 2, "Endpoint requires 2 payment tokens, Wood and Food.");

        let mut wood_amount = BigUint::zero();
        let mut food_amount = BigUint::zero();

        // Check the wood and food required
        for payment in payments.iter() {
            let token_id = payment.token_identifier;
            if self.is_required_token(&token_id, WOOD_TICKER) { wood_amount = payment.amount.clone(); }
            if self.is_required_token(&token_id, FOOD_TICKER) { food_amount = payment.amount.clone(); }         
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
        let mut user_citizens_to_mint = self.citizens_to_mint().get(&user).unwrap_or_default();
        let user_citizens_pending = user_citizens_to_mint.len();

        // Exit with an error if the user has no NFTs to mint
        require!(user_citizens_pending > 0, "No citizens pending to be minted.");

        let mut citizens_minted = 0;

        // Mint the available NFTs
        for i in 0..user_citizens_to_mint.len() {
        
            // Check if the minting period is over
            let mint_start_timestamp = user_citizens_to_mint.get(i);
            if self.blockchain().get_block_timestamp() - mint_start_timestamp >= MINT_CITIZEN_SECONDS {
            
                // Mint the NFT
                let nft_nonce = self.create_citizen_nft();

                citizens_minted += 1;

                // Transfer the NFT to the user
                self.send().direct_esdt(
                    &user,
                    &self.nft_token_id().get_token_id(),
                    nft_nonce,
                    &BigUint::from(1u64),
                    );

                // Remove the mint start timestamp for the user
                user_citizens_to_mint.remove(i);
            }

            // Update the user's mint list
            if user_citizens_to_mint.is_empty() {
                self.citizens_to_mint().remove(&user);
            } else {
                self.citizens_to_mint().insert(user.clone(), user_citizens_to_mint.clone());
            }
        }

        // Check if any NFTs were minted and exit with an error if still in the minting period
        if citizens_minted == 0 { sc_panic!("{} citizen(s) still in the minting period.", user_citizens_pending); }

    }


    #[payable("*")]
    #[endpoint(upgradeCitizenToSoldier)]
    fn upgrade_citizen_to_soldier(&self, citizen_nft_nonce: u64, owner_address: ManagedAddress) {
        self.require_character_collection();

        let payments = self.call_value().all_esdt_transfers();
        require!(payments.len() == 2, "Endpoint requires 2 payment tokens, Gold and Ore.");

        let mut gold_amount = BigUint::zero();
        let mut ore_amount = BigUint::zero();

        // Check the gold and ore required
        for payment in payments.iter() {
            let token_id = payment.token_identifier;

            if self.is_required_token(&token_id, GOLD_TICKER) { gold_amount = payment.amount.clone(); }
            if self.is_required_token(&token_id, ORE_TICKER) { ore_amount = payment.amount.clone(); }
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
        let collection_id = self.nft_token_id().get_token_id();

        // Set the amount to mint to 1 NFT
        let amount = BigUint::from(1u64);

        // Set the NFT name
        let nft_name = sc_format!("{} {}",
            ManagedBuffer::from(NFT_NAME_CITIZEN.as_bytes()),
            &(last_minted_nft_nonce + 1));

        // Set the royalties
        let royalties = BigUint::from(NFT_ROYALTIES);

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
    fn upgrade_citizen_to_soldier_nft(&self, nft_nonce: u64, owner_address: ManagedAddress) {
        self.require_character_collection();

        // Get the NFT data
        let nft_data = self.blockchain().get_esdt_token_data(&owner_address, &self.nft_token_id().get_token_id(), nft_nonce);

        // Get the NFT attributes
        let nft_attributes = nft_data.attributes;
        require!(!nft_attributes.is_empty(), "Cannot get NFT attributes. Is the NFT owner address correct?");

        // Decode the NFT attributes
        let character = self.decode_character(nft_attributes);

        // Check if the NFT is a citizen
        require!(character.is_citizen(), "Character is not a citizen");

        // Create new soldier character
        let soldier = Character::new_soldier();

        // Create new NFT name
        let new_nft_name = sc_format!("{} {}",
            ManagedBuffer::from(NFT_NAME_SOLDIER.as_bytes()),
            nft_nonce);

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
            .argument(&self.nft_token_id().get_token_id())
            .argument(&nft_nonce)
            .argument(&new_nft_name)
            .argument(&nft_data.royalties)
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
        require!(!self.nft_token_id().is_empty(), "Character collection not issued");
    }

    /// Check if a token is a required token
    fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
            == ManagedBuffer::from(required_token_ticker.as_bytes())
    }

    /// Encode nft attributes in the format: metadata:IPFS_CID/{filename}.json;tags:{tag(s)}{PREFIX}{rank}:{attack}:{defence}
    /// Ex: metadata:IPFS_CID/citizen.json;tags:character,citizen;c:0:0:0
    /// Ex: metadata:IPFS_CID/soldier21.json;tags:character,soldier;c:1:2:1
    fn get_nft_attributes(&self, character: &Character) -> ManagedBuffer {
        let nft_attributes = ManagedBuffer::from(
            sc_format!("metadata:{}/{}.{};tags:{}{}{}:{}:{}",
            ManagedBuffer::from(IPFS_CID),
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
            ManagedBuffer::from(IPFS_CID), // IPFS CID
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

    /// Decode the NFT attributes and return a Character object
    fn decode_character(&self, nft_attributes: ManagedBuffer) -> Character {
        
        // Process attributes buffer
        const BATCH_SIZE: usize = 256; // should be enough for one pass
        let mut rank = 0u8;
        let mut attack = 0u8;
        let mut defence = 0u8;
        let mut prefix_found = false;
        let mut in_rank = false;
        let mut in_attack = false;
        let mut in_defence = false;
        
        nft_attributes.for_each_batch::<BATCH_SIZE, _>(|batch| {
            let mut i = 0;
            while i < batch.len() {
                if !prefix_found {
                    // Search for the character prefix is present
                    if i + 2 < batch.len() && &batch[i..i+3] == NFT_CHARACTER_ATTRIBUTES_PREFIX.as_bytes() {
                        prefix_found = true;
                        i += 3;
                        in_rank = true;
                        continue;
                    }
                } else if in_rank {
                    if batch[i] == b':' {
                        in_rank = false;
                        in_attack = true;
                    } else {
                        require!(batch[i].is_ascii_digit(), "Invalid rank format");
                        // Parse the rank
                        rank = rank * 10 + (batch[i] - b'0');
                    }
                } else if in_attack {
                    if batch[i] == b':' {
                        in_attack = false;
                        in_defence = true;
                    } else {
                        require!(batch[i].is_ascii_digit(), "Invalid attack format");
                        // Parse the attack
                        attack = attack * 10 + (batch[i] - b'0');
                    }
                } else if in_defence {
                    require!(batch[i].is_ascii_digit(), "Invalid defence format");
                    // Parse the defence
                    defence = defence * 10 + (batch[i] - b'0');
                }
                i += 1;
            }
        });

        // Check if the prefix was found or return an error
        require!(prefix_found, "Character attributes prefix not found");

        // Return the character
        Character { rank, attack, defence }
    }

}