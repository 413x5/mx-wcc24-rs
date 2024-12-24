use multiversx_sc::imports::*;
use multiversx_sc::derive::*;
use multiversx_sc::proxy_imports::*;

/// Stake info structure for each stake
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct StakeInfo<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub round: u64,
}