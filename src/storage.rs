multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Enumeration used to define claim types and increase readability of the code
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Debug, TypeAbi)]
pub enum ClaimType {
    Reward,
    Airdrop,
    Allocation,
    Royalties,
}

// Trait used to define the maximum value of the ClaimType enumeration
pub trait Max {
    fn max() -> u8;
}

// Implementation of the Max trait for the ClaimType enumeration
impl Max for ClaimType {
    fn max() -> u8 {
        3
    }
}

// Implementation of the From trait for the ClaimType enumeration
impl From<u8> for ClaimType {
    fn from(claim_type: u8) -> Self {
        match claim_type {
            0 => ClaimType::Reward,
            1 => ClaimType::Airdrop,
            2 => ClaimType::Allocation,
            3 => ClaimType::Royalties,
            _ => ClaimType::Reward,
        }
    }
}

// Module that handles the common storage of the smart contract
#[multiversx_sc::module]
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

    #[view(viewPrivilegedAddresses)]
    #[storage_mapper("privilegedAddresses")]
    fn privileged_addresses(&self) -> SetMapper<ManagedAddress>;

    #[view(viewDepositorAddresses)]
    #[storage_mapper("depositorAddresses")]
    fn depositor_addresses(&self) -> SetMapper<ManagedAddress>;
}
