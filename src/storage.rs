elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Enumeration used to define claim types and increase readability of the code
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Debug, TypeAbi)]
pub enum ClaimType {
    Reward,
    Airdrop,
    Allocation,
}

// Module that handles the common storage of the smart contract
#[elrond_wasm::module]
pub trait StorageModule {
    // Stores the token identifier of the token that is used for claims in the smart contract
    #[view(viewTokenIdentifier)]
    #[storage_mapper("tokenIdentifier")]
    fn claim_token(&self) -> SingleValueMapper<TokenIdentifier>;

    // Stores the amount available to claim for each address and claim type
    #[view(viewClaim)]
    #[storage_mapper("claim")]
    fn claim(&self, address: &ManagedAddress, claim_type: &ClaimType)
        -> SingleValueMapper<BigUint>;

    // Stores the last timestamp at which the claim has been modified by the owner for each address and claim type
    #[view(viewClaimModifyDate)]
    #[storage_mapper("claimDate")]
    fn claim_modify_date(
        &self,
        address: &ManagedAddress,
        claim_type: &ClaimType,
    ) -> SingleValueMapper<u64>;

    // Stores whether claim harvesting is paused or not
    #[view(isPaused)]
    #[storage_mapper("isPaused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;
}
