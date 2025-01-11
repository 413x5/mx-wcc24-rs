use multiversx_sc::imports::*;

use crate::data::*;
use crate::constants::*;

#[multiversx_sc::module]
pub trait NftAttributesDecodeModule {

    /// Get the character from the NFT attributes data
    fn get_character(&self, owner_address: &ManagedAddress, character_collection_id: &TokenIdentifier, character_nonce: u64) -> Character {
        // Get the character NFT data
        let character_nft_data = self.blockchain().get_esdt_token_data(owner_address, character_collection_id, character_nonce);

        // Get the NFT attributes
        let nft_attributes = character_nft_data.attributes;
        require!(!nft_attributes.is_empty(), "Cannot get character nonce {} NFT attributes. Is the NFT owner address correct?", character_nonce);

        // Decode the NFT attributes
        let character = self.decode_character(nft_attributes);

        character
    }

    /// Get the tool from the NFT attributes data
    fn get_tool(&self, owner_address: &ManagedAddress, tool_collection_id: &TokenIdentifier, tool_nonce: u64) -> Tool {
        // Get the tool NFT data
        let tool_nft_data = self.blockchain().get_esdt_token_data(owner_address, tool_collection_id, tool_nonce);

        // Get the NFT attributes
        let nft_attributes = tool_nft_data.attributes;
        require!(!nft_attributes.is_empty(), "Cannot get tool nonce {} NFT attributes. Is the NFT owner address correct?", tool_nonce);

        // Decode the NFT attributes
        let tool = self.decode_tool(nft_attributes);

        tool
    }

    /// Decode the NFT attributes and return a Character object
    fn decode_character(&self, nft_attributes: ManagedBuffer) -> Character {

        // Character prefix
        let prefix_len = NFT_CHARACTER_ATTRIBUTES_PREFIX.len();
        let prefix_bytes = NFT_CHARACTER_ATTRIBUTES_PREFIX.as_bytes();
        
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
                    if i + prefix_len <= batch.len() && &batch[i..i+prefix_len] == prefix_bytes {
                        prefix_found = true;
                        i += prefix_len;
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


    /// Decode the NFT attributes and return a Tool object
    fn decode_tool(&self, nft_attributes: ManagedBuffer) -> Tool {

        // Tool prefix
        let prefix_len = NFT_TOOL_ATTRIBUTES_PREFIX.len();
        let prefix_bytes = NFT_TOOL_ATTRIBUTES_PREFIX.as_bytes();
        
        // Process attributes buffer
        const BATCH_SIZE: usize = 256; // should be enough for one pass
        let mut tool_type = 0u8;
        let mut attack = 0u8;
        let mut defence = 0u8;
        let mut prefix_found = false;
        let mut in_tool_type = false;
        let mut in_attack = false;
        let mut in_defence = false;
        
        nft_attributes.for_each_batch::<BATCH_SIZE, _>(|batch| {
            let mut i = 0;
            while i < batch.len() {
                if !prefix_found {
                    // Search for the tool prefix is present
                    if i + prefix_len <= batch.len() && &batch[i..i+prefix_len] == prefix_bytes {
                        prefix_found = true;
                        i += prefix_len;
                        in_tool_type = true;
                        continue;
                    }
                } else if in_tool_type {
                    if batch[i] == b':' {
                        in_tool_type = false;
                        in_attack = true;
                    } else {
                        require!(batch[i].is_ascii_digit(), "Invalid tool type format");
                        // Parse the tool type
                        tool_type = tool_type * 10 + (batch[i] - b'0');
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
        require!(prefix_found, "Tool attributes prefix not found");

        // Return the tool
        Tool { tool_type, attack, defence }
    }

}