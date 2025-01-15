#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

/// Info structure for each token deposit (fungible and non-fungible)
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub balance: BigUint<M>,
}