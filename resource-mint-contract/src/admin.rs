use multiversx_sc::imports::*;

use crate::constants::*;

/// Admin module to update contract parameters if needed
#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(setMintRoundsInterval)]
    fn set_mint_rounds_interval(&self, mint_rounds: u64) {
        self.mint_rounds_interval().set(mint_rounds);
    }

    #[only_owner]
    #[endpoint(setStakeThreshold)]
    fn set_stake_threshold(&self, stake_amount: BigUint) {
        self.mint_stake_threshold().set(stake_amount);
    }

    #[only_owner]
    #[endpoint(setOptionMintIfClaimed)]
    fn set_option_mint_if_claimed(&self, mint_if_claimed: bool) {
        self.option_mint_if_claimed().set(mint_if_claimed);
    }

    /// Issue token to be used for minting resources
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueResourceToken)]
    fn issue_resource_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer, initial_supply: OptionalValue<BigUint>) {
        require!(self.resource_token_id().is_empty(), ERR_RESOURCE_TOKEN_ALREADY_ISSUED);
        require!(!token_name.is_empty(), ERR_TOKEN_NAME_EMPTY);
        require!(!token_ticker.is_empty(), ERR_TOKEN_TICKER_EMPTY);

        let payment = self.call_value().egld_value();
        let issue_cost = BigUint::from(ISSUE_FEE);
        require!(
            *payment >= issue_cost,
            ERR_INSUFFICIENT_ISSUE_COST
        );

        // Initial supply
        let supply = match initial_supply {
            OptionalValue::Some(amount) => {
                require!(amount > BigUint::zero(), ERR_INITIAL_SUPPLY_ZERO);

                amount*BigUint::from(10u64).pow(RESOURCE_TOKEN_DECIMALS as u32)
            },
            // Reward token default initial supply of 1
            OptionalValue::None => BigUint::from(1u64)*BigUint::from(10u64).pow(RESOURCE_TOKEN_DECIMALS as u32), 
        };

        // Send the issue transaction with callback
        self.send()
            .esdt_system_sc_tx()
            .issue_fungible(
                issue_cost,
                &token_name,
                &token_ticker,
                &supply,
                FungibleTokenProperties {
                    num_decimals: RESOURCE_TOKEN_DECIMALS,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .with_callback(self.callbacks().issue_callback())
            .async_call_and_exit()
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                let (token_identifier, _returned_tokens) = 
                    self.call_value().single_fungible_esdt();
                // Set resource token id
                self.resource_token_id().set(token_identifier.clone());
            },
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_value();
                if *returned > 0 {
                    // Return issue payment in case of error
                    self.send().direct_egld(&caller, &returned);
                }
            },
        }
    }

    /// Sets the local mint role for the resource token
    #[only_owner]
    #[endpoint(setResourceTokenLocalMintRole)]
    fn set_resource_token_local_mint_role(&self) {
        require!(!self.resource_token_id().is_empty(), ERR_RESOURCE_TOKEN_NOT_ISSUED);

        // Send the set resource token local mint role transaction with callback
        self.send()
            .esdt_system_sc_tx()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.resource_token_id().get(),
                [EsdtLocalRole::Mint].iter().cloned(),
            )
            .with_callback(self.callbacks().resource_mint_role_callback())
            .async_call_and_exit();
    }

    #[callback]
    fn resource_mint_role_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // Resource token has local mint role
                self.resource_token_has_local_mint_role().set(true);
            },
            ManagedAsyncCallResult::Err(_) => {
                // Resource token has no local mint role
                self.resource_token_has_local_mint_role().set(false);
            },
        }
    }
}
