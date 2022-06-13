elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Debug, TypeAbi)]
pub enum ClaimType {
    Reward,
    Airdrop,
    Allocation,
}

#[elrond_wasm::module]
pub trait StorageModule {
    #[view(viewTokenIdentifier)]
    #[storage_mapper("tokenIdentifier")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(viewClaim)]
    #[storage_mapper("claim")]
    fn claim(&self, address: &ManagedAddress, claim_type: &ClaimType)
        -> SingleValueMapper<BigUint>;

    #[view(viewClaimAddDate)]
    #[storage_mapper("claimDate")]
    fn claim_add_date(
        &self,
        address: &ManagedAddress,
        claim_type: &ClaimType,
    ) -> SingleValueMapper<u64>;

    #[view(isPaused)]
    #[storage_mapper("isPaused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;
}
