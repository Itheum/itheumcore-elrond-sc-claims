multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage::{self, ClaimType, Max};

// Structure that is used in order to return claims with their last modification timestamp
#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Claim<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub date: u64,
}

// Module that implements views, by which we understand read-only endpoints
#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {
    //View that returns the sum of all claims, from all claim types, for a given address
    #[view(viewClaims)]
    fn view_claims(&self, address: &ManagedAddress) -> BigUint {
        let mut claim = BigUint::zero();
        for claim_type in 0..ClaimType::max() {
            claim += self.claim(address, &ClaimType::from(claim_type)).get();
        }

        claim
    }

    // View that returns all claims with the last timestamp at which the claims have been modified by the owner for a given address
    #[view(viewClaimWithDate)]
    fn view_claims_with_date(&self, address: &ManagedAddress) -> ManagedVec<Claim<Self::Api>> {
        let mut claims = ManagedVec::new();
        for claim_type in 0..ClaimType::max() {
            claims.push(Claim {
                amount: self.claim(address, &ClaimType::from(claim_type)).get(),
                date: self
                    .claim_modify_date(address, &ClaimType::from(claim_type))
                    .get(),
            });
        }

        claims
    }
}
