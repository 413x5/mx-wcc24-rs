#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

/// Info structure for each token deposit
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub balance: BigUint<M>,
}