use multiversx_sc::imports::*;

use crate::constants::*;


#[multiversx_sc::module]
pub trait AdminModule: 
    crate::storage::StorageModule {

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Register character collection as dynamic NFT and set all roles
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerCharactersCollection)]
    fn register_characters_collection(&self) {
        require!(self.nft_token_id().is_empty(), "Character collection already registered.");

        let register_cost = self.call_value().egld_value();
        require!(*register_cost == REGISTER_FEE, "Send 0.05 EGLD for the register cost.");
        
        self.tx()
            .to(ESDTSystemSCAddress)
            .with_egld_transfer(register_cost.clone_value())
            .raw_call(REGISTER_AND_SET_ALL_ROLES_DYNAMIC_ENDPOINT_NAME)
            .argument(&ManagedBuffer::from(CHARACTER_COLLECTION_NAME))
            .argument(&ManagedBuffer::from(CHARACTER_COLLECTION_TICKER))
            .argument(&ManagedBuffer::from("NFT"))
            .callback(self.callbacks().register_characters_callback())
            .async_call_and_exit();
    }

    /// Callback to update the contract state with the registered NFT token
    #[callback]
    fn register_characters_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set_token_id(token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.tx().to(ToCaller).egld(returned.amount).transfer();
                }
                self.nft_token_id().clear();
            },
        }
    }
}
