use multiversx_sc::imports::*;

use crate::constants::*;


#[multiversx_sc::module]
pub trait AdminModule:
    crate::storage::StorageModule {

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Register collection as dynamic NFT and set all roles
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerToolsCollection)]
    fn register_tools_collection(&self) {
        require!(self.tools_nft_collection().is_empty(), "Tools collection already registered.");

        let register_cost = self.call_value().egld();
        require!(*register_cost == REGISTER_FEE, "Send 0.05 EGLD for the register cost.");
        
        self.tx()
            .to(ESDTSystemSCAddress)
            .with_egld_transfer(register_cost.clone_value())
            .raw_call(REGISTER_AND_SET_ALL_ROLES_DYNAMIC_ENDPOINT_NAME)
            .argument(&ManagedBuffer::from(TOOLS_COLLECTION_NAME))
            .argument(&ManagedBuffer::from(TOOLS_COLLECTION_TICKER))
            .argument(&ManagedBuffer::from("NFT"))
            .callback(self.callbacks().register_tools_callback())
            .async_call_and_exit();
    }

    /// Callback to update the contract state with the registered NFT token
    #[callback]
    fn register_tools_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.tools_nft_collection().set_token_id(token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.tx().to(ToCaller).egld(returned.amount).transfer();
                }
                self.tools_nft_collection().clear();
            },
        }
    }

    /// Set the time in seconds to mint a shield
    #[only_owner]
    #[endpoint(setMintShieldSeconds)]
    fn set_mint_shield_seconds(&self, mint_shield_seconds: u64){
        self.mint_shield_seconds().set(mint_shield_seconds);
    }

    /// Set the time in seconds to mint a sword
    #[only_owner]
    #[endpoint(setMintSwordSeconds)]
    fn set_mint_sword_seconds(&self, mint_sword_seconds: u64){
        self.mint_sword_seconds().set(mint_sword_seconds);
    }

}