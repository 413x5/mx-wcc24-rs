use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule {

    /// Returns number of user unclaimed resources
    #[view(getUserUnclaimedResources)]
    fn user_unclaimed_resources(&self, address: &ManagedAddress<Self::Api>) -> BigUint {
        let user_minted = self.user_minted_resources(&address).get();
        let user_claimed = self.user_claimed_resources(&address).get();
        user_minted - user_claimed
    }
}

