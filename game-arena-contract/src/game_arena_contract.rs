#![no_std]


#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

use game_common_module::data::Character;

pub const ERR_CHARACTER_COLLECTION_NOT_SET: &str = "Character NFT collection is not set.";

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Game<M: ManagedTypeApi> {
    pub initiator: ManagedAddress<M>,
    pub competitor: Option<ManagedAddress<M>>,
    pub fee_token: TokenIdentifier<M>,
    pub fee_amount: BigUint<M>,
    pub initiator_soldier_nonce: u64,
    pub competitor_soldier_nonce: u64,
    pub winner_soldier_nonce: u64,
}

impl<M: ManagedTypeApi> Game<M> {
    pub fn new(initiator: ManagedAddress<M>, fee_token: TokenIdentifier<M>, fee_amount: BigUint<M>, initiator_soldier: u64) -> Self {
        Self {
            initiator,
            competitor: None,
            fee_token,
            fee_amount,
            initiator_soldier_nonce: initiator_soldier,
            competitor_soldier_nonce: 0,
            winner_soldier_nonce: 0
        }
    }
}


#[multiversx_sc::contract]
pub trait GameArenaContract:
    game_common_module::GameCommonModule +
    game_common_module::nft_attributes::NftAttributesDecodeModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(setCharactersNftCollection)]
    fn set_characters_nft_collection(&self, collection_id: TokenIdentifier){
        self.characters_nft_collection().set_if_empty(collection_id);
    }

    /// Creates a new game
    #[payable]
    #[endpoint(createGame)]
    fn create_game(&self, initiator_address: OptionalValue<ManagedAddress>) {
        // Check if the characters NFT collection is set
        self.require_characters_nft_collection();

        // Get the transfers
        let transfers = self.call_value().all_esdt_transfers();
        require!(transfers.len() == 2, "Game requires 2 transfers, a Soldier NFT and the fee token amount.");

        // Get the initiator user address
        let initiator = match initiator_address {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };
        
        // Initialize variables
        let mut character_nonce = 0u64;
        let mut soldier: Option<Character> = None;
        let mut game_fee_token: Option<TokenIdentifier> = None;
        let mut game_fee_amount = BigUint::zero();

        // Process the transfers
        for transfer in transfers.iter() {
            let token_id = &transfer.token_identifier;
            let amount = &transfer.amount;

            // Check if the token is a character NFT
            if transfer.token_type() == EsdtTokenType::NonFungible && 
                    self.is_required_token(token_id, &self.characters_nft_collection().get_token_id().as_managed_buffer()) 
            {
                character_nonce = transfer.token_nonce;
                // Get the character object
                let character = self.get_character(&self.blockchain().get_sc_address(), &self.characters_nft_collection().get_token_id(), character_nonce);
                require!(character.is_soldier(), "Character NFT is not a soldier.");
                require!(character.attack > 0 || character.defence > 0, "Soldier NFT is not an upgraded soldier.");
                soldier = Some(character);
                continue;
            }
            // Check if the initiator is sending a fee token amount
            if transfer.token_type() == EsdtTokenType::Fungible {
                game_fee_amount = amount.clone();
                game_fee_token = Some(token_id.clone());
            }
        }

        // Check if a Soldier NFT was transferred
        require!(soldier.is_some(), "Game requires a Soldier NFT.");
        // Check if a game fee token was transferred
        require!(game_fee_token.is_some(), "Game requires a game fee token.");
        // Check if a game fee token amount was transferred
        require!(game_fee_amount > BigUint::zero(), "Game requires a game fee amount.");

        // Create the game
        let new_game = Game::new(initiator, game_fee_token.unwrap(), game_fee_amount, character_nonce);
        let game_id = self.last_game_id().get() + 1;

        // Store the game
        self.open_games().insert(game_id, new_game);
        self.last_game_id().set(game_id);

    }

    #[payable]
    #[endpoint(acceptGame)]
    fn accept_game(&self, game_id: u64, competitor_address: OptionalValue<ManagedAddress>) {

        // Find the game
        let find_open_game = self.open_games().get(&game_id);

        // Check if the game exists and is open
        let mut game = match find_open_game {
            Some(open_game) => open_game,
            None => {
                if self.completed_games().get(&game_id).is_some() {
                     sc_panic!("Game {} has already been completed.", game_id); }
                else { sc_panic!("Invalid game id {}.", game_id); }
            }
        };

        // Get the competitor
        let competitor = match competitor_address {
            OptionalValue::Some(address) => address,
            OptionalValue::None => self.blockchain().get_caller(),
        };

        // Check if the competitor is the initiator
        // require!(game.initiator != competitor, "Initiator cannot join their own game.");

        // Get the transfers
        let transfers = self.call_value().all_esdt_transfers();
        require!(transfers.len() == 2, "Game requires 2 transfers, a Soldier NFT and the fee token amount.");

        // Initialize variables
        let mut character_nonce = 0u64;
        let mut competitor_soldier: Option<Character> = None;
        let mut competitor_fee_token: Option<TokenIdentifier> = None;
        let mut competitor_fee_amount = BigUint::zero();
        let character_collection_id = self.characters_nft_collection().get_token_id();

        // Process transfers
        for transfer in transfers.iter() {
            let token_id = &transfer.token_identifier;
            let amount = &transfer.amount;
            // Check if the transfer is a character NFT
            if transfer.token_type() == EsdtTokenType::NonFungible && 
                self.is_required_token(token_id, &character_collection_id.as_managed_buffer()) {
                character_nonce = transfer.token_nonce;
                // Get the character object
                let character = self.get_character(&self.blockchain().get_sc_address(), &character_collection_id, character_nonce);
                require!(character.is_soldier(), "Character NFT is not a soldier.");
                require!(character.attack > 0 || character.defence > 0, "Soldier NFT is not an upgraded soldier.");
                competitor_soldier = Some(character);
                continue;
            }
            // Check if the transfer is a fee token
            if transfer.token_type() == EsdtTokenType::Fungible {
                competitor_fee_amount = amount.clone();
                competitor_fee_token = Some(token_id.clone());
            }
        }

        // Check if the competitor provided a Soldier NFT
        require!(competitor_soldier.is_some(), "Game requires a Soldier NFT.");

        // Check if the competitor provided the a fee token
        require!(competitor_fee_token.is_some(), "Game requires the fee payment.");

        // Check if the competitor provided the same fee token and amount
        let competitor_fee_token = competitor_fee_token.unwrap();
        require!(game.fee_token == competitor_fee_token && 
            game.fee_amount == competitor_fee_amount, "Game requires the same fee token and amount.");

        let game_fee_token_id = game.fee_token.clone(); 
        let game_fee_amount = game.fee_amount.clone();

        // Update the game
        game.competitor = Some(competitor.clone());
        game.competitor_soldier_nonce = character_nonce;

        // Get the competitor's Soldier character object
        let competitor_soldier = competitor_soldier.unwrap();  

        // Determine the winner
        let (winner, winner_soldier) = 
            // Check if the competitor wins
            if self.fight(&game, competitor_soldier) 
                {(competitor, game.competitor_soldier_nonce)} 
            // Otherwise, the initiator wins
            else {(game.initiator.clone(), game.initiator_soldier_nonce)};

        // Set the winner
        game.winner_soldier_nonce = winner_soldier;

        // Remove the game from the open games
        self.open_games().remove(&game_id);
        // Add the game to the completed games
        self.completed_games().insert(game_id, game);

        // Send the prize and the soldier back to the winner
        let prize = game_fee_amount * BigUint::from(2u64); //Fee amount from both players goes to the winner
        self.send().direct_esdt(&winner, &game_fee_token_id, 0u64, &prize);
        self.send().direct_esdt(&winner, &character_collection_id, winner_soldier, &BigUint::from(1u64));        
    }

    
    /// Returns true if the competitor wins, false if the initiator wins
    fn fight(&self, game: &Game<Self::Api>, competitor_soldier: Character) -> bool {

        // Get the initiator's soldier character
        let initiator_soldier = self.get_character(
            &self.blockchain().get_sc_address(), 
            &self.characters_nft_collection().get_token_id(), 
            game.initiator_soldier_nonce);

        // Calculate total competency for each soldier (attack + defence)
        let initiator_competency: i16 = initiator_soldier.attack as i16 + initiator_soldier.defence as i16;
        let competitor_competency: i16 = competitor_soldier.attack as i16 + competitor_soldier.defence as i16;

        // Calculate competency difference
        let competency_difference = (competitor_competency - initiator_competency).abs();

        // If difference is more than 100, stronger soldier wins automatically
        if competency_difference > 100 {
            return competitor_competency > initiator_competency;
        }

        // Generate random number 0-99 with equal probability
        let random_seed = self.blockchain().get_block_random_seed();
        let seed_bytes = random_seed.to_byte_array();
        // Largest multiple of 100 less than 256 (max byte value)
        let threshold = 200u64; // Ensures equal probability
        let mut random_number = 50u64; // Default to 50%

        for byte in seed_bytes.iter() {
            if (*byte as u64) <= threshold {
                random_number = (*byte as u64) % 100;
                // If equal competencies and the random number is 50 (50-50 chance), continue
                if competency_difference == 0 && random_number == 50 { continue; }
                // Otherwise, break to apply the random number
                break;
            }
        }

        // Start with 50-50 chance and adjust by competency difference / 2
        let base_chance = 50u64;
        let competitor_win_chance = if competitor_competency > initiator_competency {
            // If competitor stronger, add to base chance %
            base_chance + (competency_difference as u64 / 2)
        } else {
            // If initiator stronger, subtract from base chance %
            base_chance - (competency_difference as u64 / 2)
        };

        // Limit min and max to 1-99% chance for competency difference close to 100
        let competitor_win_chance = competitor_win_chance.clamp(1, 99);

        // Competitor wins if random number (0-99) is less than their win chance
        // The greater the win chance, the greater the chance the random number is less
        random_number < competitor_win_chance
    }


    /// Require the characters NFT collection is set
    fn require_characters_nft_collection(&self) {
        require!(!self.characters_nft_collection().is_empty(), ERR_CHARACTER_COLLECTION_NOT_SET);
    }

    /// Open games
    #[view(getOpenGames)]
    #[storage_mapper("open_games")]
    fn open_games(&self) -> MapMapper<u64, Game<Self::Api>>;

    /// Completed games
    #[view(getCompletedGames)]
    #[storage_mapper("completed_games")]
    fn completed_games(&self) -> MapMapper<u64, Game<Self::Api>>;

    /// Characters NFT collection
    #[view(getCharactersNftCollection)]
    #[storage_mapper("characters_nft_collection")]
    fn characters_nft_collection(&self) -> NonFungibleTokenMapper;

    /// Last game ID
    #[view(getLastGameId)]
    #[storage_mapper("last_game_id")]
    fn last_game_id(&self) -> SingleValueMapper<u64>;

}
