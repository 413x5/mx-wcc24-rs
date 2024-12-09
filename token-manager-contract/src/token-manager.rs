#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

// Constants
const ISSUE_FEE: u64 = 50000000000000000; // 0.05 EGLD
const TOKEN_NAME: &str = "JohnSnow";
const TOKEN_TICKER: &str = "SNOW";
const TOKEN_DECIMALS: usize = 8;

#[multiversx_sc::contract]
pub trait TokenManager {
    #[init]
    fn init(&self) {}


    #[payable("EGLD")]
    #[endpoint(issue_token_snow)]
    fn issue_token_snow(&self, token_amount: BigUint) {
        // Check the EGLD payment is enough for issuing the token and token amount is greater than 0.
        // If more funds are sent, the difference will be returned to the caller
        let payment = self.call_value().egld_value();
        let issue_cost = BigUint::from(ISSUE_FEE);
        require!(
            *payment >= issue_cost,
            "Must send at least 0.05 EGLD for the issue cost. Any extra funds will be returned.");
        require!(
            token_amount > 0, "Token amount for issue must be greater than 0.");

        let caller = self.blockchain().get_caller();
        // Calculate initial supply. 1 SNOW = token_amount * 10^TOKEN_DECIMALS
        let initial_supply = BigUint::from(token_amount)*BigUint::from(10u64).pow(TOKEN_DECIMALS as u32);
        // Set token properties
        let mut properties = FungibleTokenProperties::default();
        properties.num_decimals = TOKEN_DECIMALS;

        // Send the issue transaction with callback
        self.send()
            .esdt_system_sc_tx()
            .issue_fungible(
                issue_cost,
                TOKEN_NAME,
                TOKEN_TICKER,
                initial_supply.clone(),
                properties
            )
            .with_callback(self.callbacks().issue_callback(&caller, &payment, &initial_supply))
            .async_call_and_exit()
    }

    #[callback]
    fn issue_callback(
        &self,
        caller: &ManagedAddress,
        payment: &BigUint,
        initial_supply: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        // get the returned tokens
        let (token_identifier, returned_tokens) = 
            self.call_value().egld_or_single_fungible_esdt();
        
        match result {
            // If the issue was successful
            ManagedAsyncCallResult::Ok(()) => {

                // Store token balance and issuer
                if let Some(token_id) = token_identifier.into_esdt_option() {
                    self.token_balances().insert(token_id.clone(), initial_supply.clone());
                    self.token_issuers().insert(token_id.clone(), caller.clone());
                    
                    // Send the newly issued tokens to the caller
                    self.send().direct_esdt(caller, &token_id, 0, initial_supply);
                }

                // return any extra EGLD amount to the caller
                let issue_fee = BigUint::from(ISSUE_FEE);
                if payment > &issue_fee {
                    let difference = payment - &issue_fee;
                    self.tx().to(caller).egld(difference).transfer();
                }
            },
            // If the issue failed
            ManagedAsyncCallResult::Err(_) => {
                // return all EGLD payment to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.tx().to(caller).egld(payment).transfer();
                }
            },
        }
    }

    #[payable("*")]
    #[endpoint(burn_tokens)]
    fn burn_tokens(&self) {
        // Get the tokens sent to be burned
        let (token_id, amount) = self.call_value().single_fungible_esdt();
        require!(amount > 0, "Must send more than 0 tokens.");
        
        // Check if sent token was issued by this contract
        require!(
            self.token_issuers().contains_key(&token_id),
            "Sent token was not issued by this contract."
        );
        
        // Update total supply
        let mut current_supply = self.get_token_balance(token_id.clone());
        current_supply -= &amount;
        self.token_balances().insert(token_id.clone(), current_supply);

        // Burn the tokens
        self.send().esdt_local_burn(&token_id, 0, &amount);
    }

    // Get all tokens issued by a specific address and their balances
    #[view(getIssuedTokensInfo)]
    fn get_issued_tokens(&self, address: ManagedAddress) -> MultiValueEncoded<MultiValue2<TokenIdentifier, BigUint>> {
        let mut result = MultiValueEncoded::new();
        
        // Iterate through all tokens
        for token_id in self.token_issuers().keys() {
            // Check if the token was issued by the specified address
            if let Some(issuer) = self.token_issuers().get(&token_id) {
                if issuer.eq(&address) {
                    let balance = self.get_token_balance(token_id.clone());
                    result.push((token_id, balance).into());
                }
            }
        }
        
        result
    }

    // Get all issued tokens, their balances and their issuers 
    #[view(getAllIssuedTokensInfo)]
    fn get_all_issued_tokens(&self) -> MultiValueEncoded<MultiValue3<TokenIdentifier, BigUint, ManagedAddress>> {
        let mut result = MultiValueEncoded::new();
        
        for token_id in self.token_issuers().keys() {
            if let Some(issuer) = self.token_issuers().get(&token_id) {
                let balance = self.get_token_balance(token_id.clone());
                result.push((token_id, balance, issuer).into());
            }
        }
        
        result
    }

    
    // Get the balance of a specific token
    fn get_token_balance(&self, token_id: TokenIdentifier) -> BigUint {
        self.token_balances().get(&token_id).unwrap_or_default()
    }

    // Store the balance of a specific token
    #[storage_mapper("token_balances")]
    fn token_balances(&self) -> MapMapper<TokenIdentifier, BigUint>;

    // Store the address that issued a specific token
    #[storage_mapper("token_issuers")]
    fn token_issuers(&self) -> MapMapper<TokenIdentifier, ManagedAddress>;

    #[upgrade]
    fn upgrade(&self) {}

}
