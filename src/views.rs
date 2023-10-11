multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage::{self, ClaimType, Len}, factory};

// Structure that is used in order to return Itheum/eGLD claims with their last modification timestamp
#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Claim<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub date: u64,
}

// Structure that is used in order to return ESDT claims with their last modification timestamp
#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct EsdtClaim<M: ManagedTypeApi> {
    pub payment: EsdtTokenPayment<M>,
    pub date: u64,
}


#[derive(ManagedVecItem, Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct ClaimsData<M: ManagedTypeApi> {
    pub reward: Claim<M>,
    pub airdrop: Claim<M>,
    pub allocation: Claim<M>,
    pub royalty: Claim<M>,
    pub factory_address: ManagedAddress<M>,
    pub treasury_address: ManagedAddress<M>,
    pub tax_percentage: BigUint<M>,
    pub third_party_egld: Claim<M>,
    pub third_party_esdt: ManagedVec<M, EsdtClaim<M>>,
}

// Module that implements views, by which we understand read-only endpoints
#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule + factory::FactoryContractProxyMethods {
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
    #[view(viewClaimsData)]
    fn view_claims_data(&self, address: &ManagedAddress) -> ClaimsData<Self::Api>{
        let claims = ClaimsData {
            reward: Claim{
                amount: self.claim(address, &ClaimType::Reward).get(),
                date: self.claim_modify_date(address, &ClaimType::Reward).get()
            },
            airdrop: Claim{
                amount: self.claim(address, &ClaimType::Airdrop).get(),
                date: self.claim_modify_date(address, &ClaimType::Airdrop).get()
            },
            allocation: Claim{
                amount: self.claim(address, &ClaimType::Allocation).get(),
                date: self.claim_modify_date(address, &ClaimType::Allocation).get()
            },
            royalty: Claim{
                amount: self.claim(address, &ClaimType::Royalty).get(),
                date: self.claim_modify_date(address, &ClaimType::Royalty).get()
            },
            factory_address: self.factory_address().get(),
            treasury_address: self.factory_treasury_address(),
            tax_percentage: self.factory_tax(),
            third_party_egld: Claim{
                amount: self.third_party_egld_claim(address).get(),
                date: self.third_party_claim_modify_date(address, &EgldOrEsdtTokenIdentifier::egld()).get()
            },
            third_party_esdt: self.third_party_token_claims(address).iter().map(
                |(token, amount)|
                EsdtClaim{
                    payment: EsdtTokenPayment::new(token.clone(), 0u64, amount),
                    date: self.third_party_claim_modify_date(address, &EgldOrEsdtTokenIdentifier::esdt(token.clone())).get()
                }
            ).collect::<ManagedVec<EsdtClaim<Self::Api>>>(),
        };

        claims
    }
}
