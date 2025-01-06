#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod constants;
pub mod admin;
pub mod storage;

use constants::*;

#[multiversx_sc::contract]
pub trait ToolsContract: 
    storage::StorageModule +
    admin::AdminModule {

    // Shield fuctionality

    /// Mints a Shield NFT
    #[payable("*")]
    #[endpoint(mintShield)]
    fn mint_shield(&self, receiver_address: OptionalValue<ManagedAddress>) {

        let (token_id, payment_amount) = self.call_value().single_fungible_esdt();

        let mut ore_amount = BigUint::zero();
        if self.is_required_token(&token_id, ORE_TICKER) { ore_amount = payment_amount.clone(); }

        require!(ore_amount == MINT_SHIELD_ORE_QUANTITY, "Ore amount sent must be {}.", MINT_SHIELD_ORE_QUANTITY);

        // Determine the receiver address if one is specified
        let user = match receiver_address {
                OptionalValue::Some(address) => address,
                OptionalValue::None => self.blockchain().get_caller(),
            };

        // Register the NFT mint to the receiver address
        let mut user_shields_to_mint = self.shields_to_mint().get(&user).unwrap_or_default();
        // Record the current mint start timestamp
        let mint_start_timestamp = self.blockchain().get_block_timestamp();
        // Add the mint start timestamp to the mint user's list
        user_shields_to_mint.push(mint_start_timestamp);
        self.shields_to_mint().insert(user.clone(), user_shields_to_mint);
        
        // Burn the ore sent
        self.send().esdt_local_burn(&token_id, 0, &payment_amount);

    }

    /// Claims a Shield NFT if the minting period is over
    #[endpoint(claimShield)]
    fn claim_shield(&self, receiver_address: OptionalValue<ManagedAddress>) {

        // Determine the receiver address if one is specified
        let user = match receiver_address {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };

        // Check if the user has any NFTs to mint
        let mut user_shields_to_mint = self.shields_to_mint().get(&user).unwrap_or_default();
        let user_shields_pending = user_shields_to_mint.len();

        // Exit with an error if the user has no NFTs to mint
        require!(user_shields_pending > 0, "No shields pending to be minted.");

        let mut shields_minted = 0;

        // Mint the available NFTs
        for i in 0..user_shields_to_mint.len() {
        
            // Check if the minting period is over
            let mint_start_timestamp = user_shields_to_mint.get(i);
            if self.blockchain().get_block_timestamp() - mint_start_timestamp >= MINT_SHIELD_SECONDS {
            
                // Mint the NFT
                let nft_nonce = self.create_shield_nft();

                shields_minted += 1;

                // Transfer the NFT to the user
                self.send().direct_esdt(
                    &user,
                    &self.tools_nft_collection().get_token_id(),
                    nft_nonce,
                    &BigUint::from(1u64),
                    );

                // Remove the mint start timestamp for the user
                user_shields_to_mint.remove(i);
            }

            // Update the user's mint list
            if user_shields_to_mint.is_empty() {
                self.shields_to_mint().remove(&user);
            } else {
                self.shields_to_mint().insert(user.clone(), user_shields_to_mint.clone());
            }
        }

        // Check if any NFTs were minted and exit with an error if still in the minting period
        if shields_minted == 0 { sc_panic!("{} shield(s) still in the minting period.", user_shields_pending); }

    }

    /// Creates a Shield NFT
    fn create_shield_nft(&self) -> u64 {
        self.require_tools_collection();

        // Get the last minted NFT nonce
        let last_minted_nft_nonce = self.get_last_minted_nft_nonce();

        // Get the collection ID
        let collection_id = self.tools_nft_collection().get_token_id();

        // Set the amount to mint to 1 NFT
        let amount = BigUint::from(1u64);

        // Set the NFT name
        let nft_name = sc_format!("{} {}",
            ManagedBuffer::from(SHIELD_NFT_NAME.as_bytes()),
            &(last_minted_nft_nonce + 1));

        // Set the royalties
        let royalties = BigUint::from(SHIELD_NFT_ROYALTIES);

        // Get the attributes
        let attributes = self.get_shiled_nft_attributes();

        // Get the URIs
        let uris = self.get_shield_nft_asset_uris();

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

    /// Gets the attributes for the Shield NFT
    fn get_shiled_nft_attributes(&self) -> ManagedBuffer {
        let nft_attributes = ManagedBuffer::from(
            sc_format!("metadata:{}/shield.json;tags:tool,shield",
            ManagedBuffer::from(IPFS_CID)
        ));
        nft_attributes
    }

    /// Get the URIs for the NFT assets (image, metadata)
    fn get_shield_nft_asset_uris(&self) -> ManagedVec<ManagedBuffer> {
        // Get the base filename
        let asset_base_filename = 
            sc_format!("https://{}.ipfs.w3s.link/shield", ManagedBuffer::from(IPFS_CID)
        );
        // Get the image and metadata URIs by adding the file extension
        let asset_image = sc_format!("{}.png", asset_base_filename);
        let asset_metadata = sc_format!("{}.json", asset_base_filename);
        
        // Return the URIs
        let mut assets = 
            ManagedVec::from_single_item(asset_image);
            assets.push(asset_metadata);

        assets
    }


    // Sword functionality

    /// Mints a Sword NFT
    #[payable("*")]
    #[endpoint(mintSword)]
    fn mint_sword(&self, receiver_address: OptionalValue<ManagedAddress>) {
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

        require!(gold_amount == MINT_SWORD_GOLD_QUANTITY, "Gold amount sent must be {}.", MINT_SWORD_GOLD_QUANTITY);
        require!(ore_amount == MINT_SWORD_ORE_QUANTITY, "Ore amount sent must be {}.", MINT_SWORD_ORE_QUANTITY);

        // Determine the receiver address if one is specified
        let user = match receiver_address {
                OptionalValue::Some(address) => address,
                OptionalValue::None => self.blockchain().get_caller(),
            };

        // Register the NFT mint to the receiver address
        let mut user_swords_to_mint = self.swords_to_mint().get(&user).unwrap_or_default();
        // Record the current mint start timestamp
        let mint_start_timestamp = self.blockchain().get_block_timestamp();
        // Add the mint start timestamp to the mint user's list
        user_swords_to_mint.push(mint_start_timestamp);
        self.swords_to_mint().insert(user.clone(), user_swords_to_mint);
        
        // Burn the gold and ore sent
        for payment in payments.iter() {
            self.send().esdt_local_burn(&payment.token_identifier, 0, &payment.amount);
        }
    }

    /// Claims a Sword NFT if the minting period is over
    #[endpoint(claimSword)]
    fn claim_sword(&self, receiver_address: OptionalValue<ManagedAddress>) {

        // Determine the receiver address if one is specified
        let user = match receiver_address {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };

        // Check if the user has any NFTs to mint
        let mut user_swords_to_mint = self.swords_to_mint().get(&user).unwrap_or_default();
        let user_swords_pending = user_swords_to_mint.len();

        // Exit with an error if the user has no NFTs to mint
        require!(user_swords_pending > 0, "No swords pending to be minted.");

        let mut swords_minted = 0;

        // Mint the available NFTs
        for i in 0..user_swords_to_mint.len() {
        
            // Check if the minting period is over
            let mint_start_timestamp = user_swords_to_mint.get(i);
            if self.blockchain().get_block_timestamp() - mint_start_timestamp >= MINT_SWORD_SECONDS {
            
                // Mint the NFT
                let nft_nonce = self.create_sword_nft();

                swords_minted += 1;

                // Transfer the NFT to the user
                self.send().direct_esdt(
                    &user,
                    &self.tools_nft_collection().get_token_id(),
                    nft_nonce,
                    &BigUint::from(1u64),
                    );

                // Remove the mint start timestamp for the user
                user_swords_to_mint.remove(i);
            }

            // Update the user's mint list
            if user_swords_to_mint.is_empty() {
                self.swords_to_mint().remove(&user);
            } else {
                self.swords_to_mint().insert(user.clone(), user_swords_to_mint.clone());
            }
        }

        // Check if any NFTs were minted and exit with an error if still in the minting period
        if swords_minted == 0 { sc_panic!("{} sword(s) still in the minting period.", user_swords_pending); }

    }

    /// Creates a Sword NFT
    fn create_sword_nft(&self) -> u64 {
        self.require_tools_collection();

        // Get the last minted NFT nonce
        let last_minted_nft_nonce = self.get_last_minted_nft_nonce();

        // Get the collection ID
        let collection_id = self.tools_nft_collection().get_token_id();

        // Set the amount to mint to 1 NFT
        let amount = BigUint::from(1u64);

        // Set the NFT name
        let nft_name = sc_format!("{} {}",
            ManagedBuffer::from(SWORD_NFT_NAME.as_bytes()),
            &(last_minted_nft_nonce + 1));

        // Set the royalties
        let royalties = BigUint::from(SWORD_NFT_ROYALTIES);

        // Get the attributes
        let attributes = self.get_sword_nft_attributes();

        // Get the URIs
        let uris = self.get_sword_nft_asset_uris();

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

    /// Gets the attributes for the Sword NFT
    fn get_sword_nft_attributes(&self) -> ManagedBuffer {
        let nft_attributes = ManagedBuffer::from(
            sc_format!("metadata:{}/sword.json;tags:tool,sword",
            ManagedBuffer::from(IPFS_CID)
        ));
        nft_attributes
    }

    /// Get the URIs for the NFT assets (image, metadata)
    fn get_sword_nft_asset_uris(&self) -> ManagedVec<ManagedBuffer> {
        // Get the base filename
        let asset_base_filename = 
            sc_format!("https://{}.ipfs.w3s.link/sword", ManagedBuffer::from(IPFS_CID)
        );
        // Get the image and metadata URIs by adding the file extension
        let asset_image = sc_format!("{}.png", asset_base_filename);
        let asset_metadata = sc_format!("{}.json", asset_base_filename);
        
        // Return the URIs
        let mut assets = 
            ManagedVec::from_single_item(asset_image);
            assets.push(asset_metadata);

        assets
    }


    // Common functions

    /// Check if a token is a required token
    fn is_required_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) -> bool {
        check_token_id.as_managed_buffer().copy_slice(0, required_token_ticker.len()).unwrap_or_default()
         == ManagedBuffer::from(required_token_ticker.as_bytes())
    }

    /// Check if a token is a required token and terminates if not
    fn require_expected_token(&self, check_token_id: &TokenIdentifier, required_token_ticker: &str) {
        let expected_token = ManagedBuffer::from(required_token_ticker.as_bytes());
        require!(self.is_required_token(check_token_id, required_token_ticker), "Invalid token {}. Expected {}.", check_token_id, expected_token);
    }

    /// Require that the tools collection is issued
    fn require_tools_collection(&self) {
        require!(!self.tools_nft_collection().is_empty(), "Tools collection not issued");
    }

    /// Get the last minted NFT nonce
    fn get_last_minted_nft_nonce(&self) -> u64 {
        if self.last_minted_nft_nonce().is_empty() { 0 } 
        else { self.last_minted_nft_nonce().get() }
    }
}
