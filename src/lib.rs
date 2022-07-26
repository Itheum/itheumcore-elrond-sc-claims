#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();

use crate::storage::ClaimType;

pub mod events;
pub mod requirements;
pub mod storage;
pub mod views;

#[elrond_wasm::contract]
pub trait ClaimsContract:
    storage::StorageModule
    + events::EventsModule
    + views::ViewsModule
    + requirements::RequirementsModule
{
    #[init]
    fn init(&self) {
        self.is_paused().set(true);
    }

    #[only_owner]
    #[endpoint(setRewardToken)]
    fn set_reward_token(&self, token: TokenIdentifier) {
        require!(
            self.reward_token().is_empty(),
            "Reward token is already set"
        );
        self.reward_token().set(&token);
    }

    #[only_owner]
    #[endpoint(pause)]
    fn pause(&self) {
        require!(!self.is_paused().get(), "Contract is already paused");
        self.is_paused().set(true);
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        require!(self.is_paused().get(), "Contract is already unpaused");
        self.is_paused().set(false);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(addClaim)]
    fn add_claim(&self, address: &ManagedAddress, claim_type: ClaimType) {
        self.require_reward_token_is_set();
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let current_claim = self.claim(address, &claim_type).get();
        let timestamp = self.blockchain().get_block_timestamp();
        self.require_token_is_reward(payment_token);
        self.require_value_not_zero(&payment_amount);
        self.claim(address, &claim_type)
            .set(current_claim + &payment_amount);
        self.claim_add_date(address, &claim_type).set(timestamp);
        self.claim_added_event(address, &claim_type, &payment_amount);
    }

    #[only_owner]
    #[payable("*")]
    #[endpoint(addClaims)]
    fn add_claims(
        &self,
        claims: MultiValueEncoded<MultiValue3<ManagedAddress, ClaimType, BigUint>>,
    ) {
        self.require_reward_token_is_set();
        require!(
            claims.len() <= 200,
            "Exceeded maximum number of claims per operation (200)"
        );
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let timestamp = self.blockchain().get_block_timestamp();
        self.require_token_is_reward(payment_token);
        self.require_value_not_zero(&payment_amount);
        let mut sum_of_claims = BigUint::zero();
        for item in claims.into_iter() {
            let (address, claim_type, amount) = item.into_tuple();
            let current_claim = self.claim(&address, &claim_type).get();
            self.claim(&address, &claim_type)
                .set(current_claim + &amount);
            self.claim_add_date(&address, &claim_type).set(timestamp);
            sum_of_claims += &amount;
            self.claim_added_event(&address, &claim_type, &amount);
        }
        require!(
            sum_of_claims == payment_amount,
            "Claims added must equal payment amount"
        );
    }

    #[only_owner]
    #[endpoint(removeClaim)]
    fn remove_claim(&self, address: &ManagedAddress, claim_type: ClaimType, amount: BigUint) {
        self.require_reward_token_is_set();
        let current_claim = self.claim(address, &claim_type).get();
        let owner = self.blockchain().get_owner_address();
        let reward_token = self.reward_token().get();
        let timestamp = self.blockchain().get_block_timestamp();
        self.require_value_not_zero(&amount);
        self.require_remove_claim_is_valid(&current_claim, &amount);
        self.claim(address, &claim_type)
            .set(current_claim - &amount);
        self.claim_add_date(address, &claim_type).set(timestamp);
        self.claim_removed_event(address, &claim_type, &amount);
        self.send().direct(&owner, &reward_token, 0, &amount, &[]);
    }

    #[only_owner]
    #[endpoint(removeClaims)]
    fn remove_claims(
        &self,
        claims: MultiValueEncoded<MultiValue3<ManagedAddress, ClaimType, BigUint>>,
    ) {
        self.require_reward_token_is_set();
        require!(
            claims.len() <= 200,
            "Exceeded maximum number of claims per operation (20)"
        );
        let mut sum_of_claims = BigUint::zero();
        let timestamp = self.blockchain().get_block_timestamp();
        for item in claims.into_iter() {
            let (address, claim_type, amount) = item.into_tuple();
            let current_claim = self.claim(&address, &claim_type).get();
            self.claim_add_date(&address, &claim_type).set(timestamp);
            self.require_value_not_zero(&amount);
            self.require_remove_claim_is_valid(&current_claim, &amount);
            sum_of_claims += &amount;
            self.claim(&address, &claim_type)
                .set(current_claim - &amount);
            self.claim_removed_event(&address, &claim_type, &amount);
        }
        let owner = self.blockchain().get_owner_address();
        let reward_token = self.reward_token().get();
        self.send()
            .direct(&owner, &reward_token, 0, &sum_of_claims, &[]);
    }

    #[endpoint(claim)]
    fn harvest_claim(&self, claim_type: OptionalValue<ClaimType>) {
        require!(!self.is_paused().get(), "Contract is paused");
        self.require_reward_token_is_set();
        let reward_token = self.reward_token().get();
        let caller = self.blockchain().get_caller();
        let mut claim = BigUint::zero();
        if let OptionalValue::Some(what_type_to_claim) = claim_type {
            claim = self.claim(&caller, &what_type_to_claim).get();
            self.require_value_not_zero(&claim);
            self.claim(&caller, &what_type_to_claim)
                .set(BigUint::zero());
            self.claim_collected_event(&caller, &what_type_to_claim, &claim);
        } else {
            let reward_claim = self.claim(&caller, &ClaimType::Reward).get();
            if reward_claim > BigUint::zero() {
                claim += &reward_claim;
                self.claim_collected_event(&caller, &ClaimType::Reward, &reward_claim);
                self.claim(&caller, &ClaimType::Reward).set(BigUint::zero());
            }

            let airdrop_claim = self.claim(&caller, &ClaimType::Airdrop).get();
            if airdrop_claim > BigUint::zero() {
                claim += &airdrop_claim;
                self.claim_collected_event(&caller, &ClaimType::Airdrop, &airdrop_claim);
                self.claim(&caller, &ClaimType::Airdrop)
                    .set(BigUint::zero());
            }

            let allocation_claim = self.claim(&caller, &ClaimType::Allocation).get();
            if allocation_claim > BigUint::zero() {
                claim += &allocation_claim;
                self.claim_collected_event(&caller, &ClaimType::Allocation, &allocation_claim);
                self.claim(&caller, &ClaimType::Allocation)
                    .set(BigUint::zero());
            }

            self.require_value_not_zero(&claim);
        }
        self.send().direct(&caller, &reward_token, 0, &claim, &[]);
    }
}
