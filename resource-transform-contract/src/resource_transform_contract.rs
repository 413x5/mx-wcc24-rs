#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

const ISSUE_FEE: u64 = 50_000_000_000_000_000; // 0.05 EGLD (0.05 * 10^18 decimals)
pub const ORE_TOKEN_NAME: &str = "Ore";
pub const ORE_TOKEN_TICKER_PREFIX: &str = "ORE";
pub const ORE_TOKEN_DECIMALS: usize = 0;

pub const STONE_TOKEN_TICKER: &str = "STONE-";
pub const STONE_TOKEN_DECIMALS: usize = 0;
pub const STONE_AMMOUNT_FOR_ORE: u64 = 20;

#[multiversx_sc::contract]
pub trait ResourceTransformContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // Admin endpoints

    /// Issue and set all roles for the ore token
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueAndSetRolesOreToken)]
    fn issue_and_set_roles_ore_token(&self) {
        require!(self.ore_token_id().is_empty(), "Ore token already issued.");

        let payment = self.call_value().egld();
        require!(*payment == ISSUE_FEE, "Send 0.05 EGLD for the issue cost.");

        self.ore_token_id().issue_and_set_all_roles(
            payment.clone_value(), 
            ManagedBuffer::from(ORE_TOKEN_NAME.as_bytes()),
            ManagedBuffer::from(ORE_TOKEN_TICKER_PREFIX.as_bytes()),
            ORE_TOKEN_DECIMALS, 
            self.callbacks().issue_ore_token_callback().into(),
        );

    }

    /// Callback for the issue transaction to update the contract state with the issued token
    #[callback]
    fn issue_ore_token_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.ore_token_id().set_token_id(token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.tx().to(ToCaller).egld(returned.amount).transfer();
                }
                self.ore_token_id().clear();
            },
        }
    }

    // Public endpoints

    /// Create ore by burning the stone tokens
    #[payable("*")]
    #[endpoint(createOre)]
    fn create_ore(&self, receiver_address: OptionalValue<ManagedAddress>) {
        self.ore_token_id().require_issued_or_set();

        let (token_id, payment_amount) = self.call_value().single_fungible_esdt();

        // Check expected token
        self.require_expected_token(&token_id, STONE_TOKEN_TICKER);

        // Get the amount of stone received
        let stone_amount = payment_amount.clone() / 10u64.pow(STONE_TOKEN_DECIMALS as u32);

        // Check that the amount of stone is greater than or equal to the amount needed for one ore
        require!(stone_amount.clone() >= STONE_AMMOUNT_FOR_ORE, "Stone amount must be equal or greater than {}.", STONE_AMMOUNT_FOR_ORE);
        
        // Calculate the amount of ore units by dividing the stone amount by the amount needed for one ore unit
        let ore_units = (stone_amount / STONE_AMMOUNT_FOR_ORE).to_u64().unwrap_or_default();
        
        // Calculate the amount of ore to mint
        let ore_amount = BigUint::from(ore_units)*BigUint::from(10u64).pow(ORE_TOKEN_DECIMALS as u32);
        
        // Mint the amount of ore
        self.ore_token_id().mint(ore_amount.clone());
        
        // Burn the amount of stone
        self.send().esdt_local_burn(&token_id, 0, &payment_amount);
        
        // Determine the receiver address
        let receiver_address = match receiver_address.into_option() {
            Some(address) => address,
            None => self.blockchain().get_caller(),
        };

        // Send the amount of ore to the receiver address
        self.send().direct_esdt(&receiver_address, &self.ore_token_id().get_token_id(), 0, &ore_amount);
    }


    // Private functions

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

    // Storage
    
    /// Ore token identifier
    #[view(getOreTokenId)]
    #[storage_mapper("oreTokenId")]
    fn ore_token_id(&self) -> FungibleTokenMapper;
}
