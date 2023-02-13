#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage::ClaimType;

pub mod events;
pub mod requirements;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait ClaimsContract:
    storage::StorageModule
    + events::EventsModule
    + views::ViewsModule
    + requirements::RequirementsModule
{
    //When the smart contract is deployed claim harvesting is paused
    #[init]
    fn init(&self) {
        self.is_paused().set(true);
    }

    //Endpoint available for the owner of the smart contract to set the token used by the smart contract for claims. Can only be called once successfully.
    #[only_owner]
    #[endpoint(setClaimToken)]
    fn set_claim_token(&self, token: TokenIdentifier) {
        require!(self.claim_token().is_empty(), "Claim token is already set");
        self.claim_token().set(&token);
    }

    //Endpoint available for privileged addresses of the smart contract to pause claim harvesting. Cannot be called while harvesting is already paused.
    #[endpoint(pause)]
    fn pause(&self) {
        require!(!self.is_paused().get(), "Contract is already paused");
        let caller = self.blockchain().get_caller();
        self.require_address_is_privileged(&caller);
        self.is_paused().set(true);
        self.harvest_paused_event(&caller);
    }

    //Endpoint avbailable for the owner of the smart contract to resume claim harvesting. Cannot be called while harvesting is already unpaused.
    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        require!(self.is_paused().get(), "Contract is already unpaused");
        self.is_paused().set(false);
        self.harvest_unpaused_event();
    }

    //Endpoint available for owner in order to add an address to the list of privileged addresses
    #[only_owner]
    #[endpoint(addPrivilegedAddress)]
    fn add_privileged_address(&self, address: ManagedAddress) {
        let privileged_addresses = self.privileged_addresses();
        require!(
            !privileged_addresses.contains(&address),
            "Address is already privileged"
        );
        require!(
            privileged_addresses.len() < 2usize,
            "Maximum number of priviledged addresses reached"
        );
        let owner = self.blockchain().get_owner_address();
        require!(
            owner != address,
            "Owner cannot be added to priviledged addresses"
        );
        self.privileged_address_added_event(&address);
        self.privileged_addresses().insert(address);
    }

    //Endpoint available for owner in order to remove an address from the list of privileged addresses
    #[only_owner]
    #[endpoint(removePrivilegedAddress)]
    fn remove_privileged_address(&self, address: ManagedAddress) {
        let privileged_addresses = self.privileged_addresses();
        require!(
            privileged_addresses.contains(&address),
            "Address is not privileged"
        );
        self.privileged_address_removed_event(&address);
        self.privileged_addresses().remove(&address);
    }

    //Endpoint available for privileged addresses of the smart contract to add a claim of a specific claim type for a specific address.
    #[payable("*")]
    #[endpoint(addClaim)]
    fn add_claim(&self, address: &ManagedAddress, claim_type: ClaimType) {
        self.require_claim_token_is_set();
        let (payment_token, payment_amount) = self.call_value().single_fungible_esdt();
        self.require_token_is_correct(payment_token);
        self.require_value_not_zero(&payment_amount);
        let caller = self.blockchain().get_caller();
        self.require_address_is_privileged(&caller);
        let current_claim = self.claim(address, &claim_type).get();
        let timestamp = self.blockchain().get_block_timestamp();
        //Add the amount of the tokens sent to the current claim reservation
        self.claim(address, &claim_type)
            .set(current_claim + &payment_amount);
        //Update the last modification date of the claim to the current timestamp
        self.claim_modify_date(address, &claim_type).set(timestamp);
        self.claim_added_event(&caller, &address, &claim_type, &payment_amount);
    }

    //Endpoint available for privileged addresses of the smart contract to add a bulk of claims of different claim types for different specific addresses.
    #[payable("*")]
    #[endpoint(addClaims)]
    fn add_claims(
        &self,
        claims: MultiValueEncoded<MultiValue3<ManagedAddress, ClaimType, BigUint>>,
    ) {
        self.require_claim_token_is_set();
        self.require_number_of_claims_in_bulk_is_valid(&claims.len());
        let (payment_token, payment_amount) = self.call_value().single_fungible_esdt();
        self.require_token_is_correct(payment_token);
        self.require_value_not_zero(&payment_amount);
        let caller = self.blockchain().get_caller();
        self.require_address_is_privileged(&caller);
        let timestamp = self.blockchain().get_block_timestamp();
        //Initialize the sum of claims to be added to zero
        let mut sum_of_claims = BigUint::zero();
        //Iterate over the claims provided as argument and proceeds similarly to the add_claim endpoint for each one
        for item in claims.into_iter() {
            let (address, claim_type, amount) = item.into_tuple();
            self.require_value_not_zero(&amount);
            let current_claim = self.claim(&address, &claim_type).get();
            self.claim(&address, &claim_type)
                .set(current_claim + &amount);
            self.claim_modify_date(&address, &claim_type).set(timestamp);
            sum_of_claims += &amount;
            self.claim_added_event(&caller, &address, &claim_type, &amount);
        }
        //Panic if the amount of tokens sent by the owner to the endpoint are not equal to the sum of the claims added to the contract
        require!(
            sum_of_claims == payment_amount,
            "Claims added must equal payment amount"
        );
    }

    //Endpoint available for the owner of the smart contract to remove a claim of a specific claim type for a specific address.
    #[only_owner]
    #[endpoint(removeClaim)]
    fn remove_claim(&self, address: &ManagedAddress, claim_type: ClaimType, amount: BigUint) {
        self.require_claim_token_is_set();
        self.require_value_not_zero(&amount);
        let current_claim = self.claim(address, &claim_type).get();
        self.require_remove_claim_is_valid(&current_claim, &amount);
        let owner = self.blockchain().get_owner_address();
        let claim_token = self.claim_token().get();
        let timestamp = self.blockchain().get_block_timestamp();
        //Remove the amount of tokens given as argument from the current claim reservation
        self.claim(address, &claim_type)
            .set(current_claim - &amount);
        //Update the modification date of the claim to the current timestamp
        self.claim_modify_date(address, &claim_type).set(timestamp);
        self.claim_removed_event(&address, &claim_type, &amount);
        //Send the removed tokens from the claim back to the owner of the smart contract
        self.send().direct_esdt(&owner, &claim_token, 0, &amount);
    }

    //Endpoint available for the owner of the smart contract to remove a bulk of claims of different claim types for different specific addresses.
    #[only_owner]
    #[endpoint(removeClaims)]
    fn remove_claims(
        &self,
        claims: MultiValueEncoded<MultiValue3<ManagedAddress, ClaimType, BigUint>>,
    ) {
        self.require_claim_token_is_set();
        //Panics if the user tries to add more than 200 claims per operation. Implemented in order to ensure
        self.require_number_of_claims_in_bulk_is_valid(&claims.len());
        //Initialize the sum of claims to be removed to zero
        let mut sum_of_claims = BigUint::zero();
        let timestamp = self.blockchain().get_block_timestamp();
        //Iterate over the claims provided as argument and proceeds similarly to the remove_claim endpoint for each one
        for item in claims.into_iter() {
            let (address, claim_type, amount) = item.into_tuple();
            self.require_value_not_zero(&amount);
            let current_claim = self.claim(&address, &claim_type).get();
            self.require_remove_claim_is_valid(&current_claim, &amount);
            self.claim_modify_date(&address, &claim_type).set(timestamp);
            sum_of_claims += &amount;
            self.claim(&address, &claim_type)
                .set(current_claim - &amount);
            self.claim_removed_event(&address, &claim_type, &amount);
        }
        let owner = self.blockchain().get_owner_address();
        let claim_token = self.claim_token().get();
        //Send the removed tokens from the claim back to the owner of the smart contract
        self.send()
            .direct_esdt(&owner, &claim_token, 0, &sum_of_claims);
    }

    //Endpoint available for the public to claim tokens reserved for the calling address. Cannot be called while contract is paused for the public/(harvesting is paused).
    //Can be given an argument as a claim type to harvest only specific claim type. If the claim_type argument is not provided, all claim types for the calling addresses will be harvested.
    #[endpoint(claim)]
    fn harvest_claim(&self, claim_type: OptionalValue<ClaimType>) {
        require!(!self.is_paused().get(), "Contract is paused");
        self.require_claim_token_is_set();
        let claim_token = self.claim_token().get();
        let caller = self.blockchain().get_caller();
        //Initializes the amount of tokens to be harvested to zero.
        let mut claim = BigUint::zero();
        //Checks whether the claim type argument is provided.
        if let OptionalValue::Some(what_type_to_claim) = claim_type {
            //Sets claim to the given amount of tokens reserved for the calling address and the given claim type.
            claim = self.claim(&caller, &what_type_to_claim).get();
            self.require_value_not_zero(&claim);
            //Resets the reserved tokens for the given claim type of the calling address to zero.
            self.claim(&caller, &what_type_to_claim)
                .set(BigUint::zero());
            self.claim_collected_event(&caller, &what_type_to_claim, &claim);
        } else {
            //Sets claim to the sum of all reserved tokens for the calling address.

            //Checks claims of the reward type and adds them to the sum if they are not zero.
            let reward_claim = self.claim(&caller, &ClaimType::Reward).get();
            if reward_claim > BigUint::zero() {
                claim += &reward_claim;
                self.claim_collected_event(&caller, &ClaimType::Reward, &reward_claim);
                self.claim(&caller, &ClaimType::Reward).set(BigUint::zero());
            }

            //Checks claims of the airdrop type and adds them to the sum if they are not zero.
            let airdrop_claim = self.claim(&caller, &ClaimType::Airdrop).get();
            if airdrop_claim > BigUint::zero() {
                claim += &airdrop_claim;
                self.claim_collected_event(&caller, &ClaimType::Airdrop, &airdrop_claim);
                self.claim(&caller, &ClaimType::Airdrop)
                    .set(BigUint::zero());
            }

            //Checks claims of the allocation type and adds them to the sum if they are not zero.
            let allocation_claim = self.claim(&caller, &ClaimType::Allocation).get();
            if allocation_claim > BigUint::zero() {
                claim += &allocation_claim;
                self.claim_collected_event(&caller, &ClaimType::Allocation, &allocation_claim);
                self.claim(&caller, &ClaimType::Allocation)
                    .set(BigUint::zero());
            }

            self.require_value_not_zero(&claim);
        }
        //Send the amount of tokens harvested (all tokens of a given claim type or the sum for all claim types) to the calling address.
        self.send().direct_esdt(&caller, &claim_token, 0, &claim);
    }
}
