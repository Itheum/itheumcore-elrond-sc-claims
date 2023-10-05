multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage::{self, ClaimType, Len};

// Structure that is used in order to return claims with their last modification timestamp
#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Claim<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub date: u64,
}


#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct ClaimsOut<M: ManagedTypeApi> {
    pub reward: BigUint<M>,
    pub airdrop: BigUint<M>,
    pub allocation: BigUint<M>,
    pub royalty: BigUint<M>,
    pub third_party_egld: BigUint<M>,
    pub third_party_esdt: ManagedVec<M, EsdtTokenPayment<M>>,
}

// Module that implements views, by which we understand read-only endpoints
#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {
    //View that returns the sum of all claims, from all claim types, for a given address
    #[view(viewClaims)]
    fn view_claims(&self, address: &ManagedAddress) -> BigUint {
        let mut claim = BigUint::zero();
        for claim_type in 0..ClaimType::len() + 1 {
            claim += self.claim(address, &ClaimType::from(claim_type)).get();
        }

        claim
    }

    // View that returns all claims with the last timestamp at which the claims have been modified by the owner for a given address
    #[view(viewClaimWithDate)]
    fn view_claims_with_date(&self, address: &ManagedAddress) -> ManagedVec<Claim<Self::Api>> {
        let mut claims = ManagedVec::new();
        for claim_type in 0..ClaimType::len() + 1 {
            claims.push(Claim {
                amount: self.claim(address, &ClaimType::from(claim_type)).get(),
                date: self
                    .claim_modify_date(address, &ClaimType::from(claim_type))
                    .get(),
            });
        }

        claims
    }

    // View that returns all claim amounts 
    #[view(viewAllClaims)]
    fn view_all_claims(&self, address: &ManagedAddress) -> ClaimsOut<Self::Api>{
        let claims = ClaimsOut {
            reward: self.claim(address, &ClaimType::Reward).get(),
            airdrop: self.claim(address, &ClaimType::Airdrop).get(),
            allocation: self.claim(address, &ClaimType::Allocation).get(),
            royalty: self.claim(address, &ClaimType::Royalty).get(),
            third_party_egld: self.third_party_egld_claim(address).get(),
            third_party_esdt: self.third_party_token_claims(address).iter().map(
                |(token, amount)|
                EsdtTokenPayment::new(token, 0u64, amount)).collect::<ManagedVec<EsdtTokenPayment>>(),
        };

        claims
    }
}
