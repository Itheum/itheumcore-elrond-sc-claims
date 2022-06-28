elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::ClaimType;

#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Claim<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub date: u64,
}

#[elrond_wasm::module]
pub trait ViewsModule: crate::storage::StorageModule {
    #[view(viewClaims)]
    fn view_claims(&self, address: &ManagedAddress) -> BigUint {
        let mut claim = BigUint::zero();
        claim += self.claim(address, &ClaimType::Reward).get();
        claim += self.claim(address, &ClaimType::Airdrop).get();
        claim += self.claim(address, &ClaimType::Allocation).get();
        claim
    }

    #[view(viewClaimWithDate)]
    fn view_claims_with_date(&self, address: &ManagedAddress) -> ManagedVec<Claim<Self::Api>> {
        let mut claims = ManagedVec::new();
        claims.push(Claim {
            amount: self.claim(address, &ClaimType::Reward).get(),
            date: self.claim_add_date(address, &ClaimType::Reward).get(),
        });
        claims.push(Claim {
            amount: self.claim(address, &ClaimType::Airdrop).get(),
            date: self.claim_add_date(address, &ClaimType::Airdrop).get(),
        });
        claims.push(Claim {
            amount: self.claim(address, &ClaimType::Allocation).get(),
            date: self.claim_add_date(address, &ClaimType::Allocation).get(),
        });
        claims
    }
}
