use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule {

    /// Returns number of user unclaimed resources
    #[view(getUserUnclaimedResources)]
    fn user_unclaimed_resources(&self, address: &ManagedAddress<Self::Api>) -> BigUint {
        let user_minted = self.user_minted_resources().get(&address).unwrap_or_default();
        let user_claimed = self.user_claimed_resources().get(&address).unwrap_or_default();
        user_minted - user_claimed
    }
}
