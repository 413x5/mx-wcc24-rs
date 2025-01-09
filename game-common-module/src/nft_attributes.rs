use multiversx_sc::imports::*;

use crate::data::*;
use crate::constants::*;

#[multiversx_sc::module]
pub trait NftAttributesDecodeModule {

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


    /// Decode the NFT attributes and return a Tool object
    fn decode_tool(&self, nft_attributes: ManagedBuffer) -> Tool {
        
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
                    if i + 2 < batch.len() && &batch[i..i+3] == NFT_TOOL_ATTRIBUTES_PREFIX.as_bytes() {
                        prefix_found = true;
                        i += 3;
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