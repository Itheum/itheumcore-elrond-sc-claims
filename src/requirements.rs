multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{constants::*, storage};

// Module that handles generic (commonly used, which are not specific to one function) requirements which should stop execution and rollback if not met
#[multiversx_sc::module]
pub trait RequirementsModule: storage::StorageModule {
    // Checks whether the owner of the smart contract designated a token to be used by the smart contract for all the claims
    fn require_claim_token_is_set(&self) {
        require!(!self.claim_token().is_empty(), ERR_TOKEN_NOT_SET);
    }

    // Checks whether a given token identifier is equal to the token identifier of the token used by the smart contract claims
    fn require_token_is_correct(&self, token: TokenIdentifier) {
        require!(token == self.claim_token().get(), ERR_TOKEN_INCORRECT);
    }

    // Checks whether a value is not zero
    fn require_value_not_zero(&self, value: &BigUint) {
        require!(value > &BigUint::zero(), ERR_NON_ZERO_VALUE);
    }

    // Checks whether a claim that is intended to be removed is smaller than the amount reserved in the claim
    fn require_remove_claim_is_valid(&self, current_claim: &BigUint, amount: &BigUint) {
        require!(current_claim >= amount, ERR_MORE_THAN_CLAIM);
    }

    // Checks whether the number of claims added or removed is smaller than 200. Implemented in order to ensure no call will fail due to consuming more than the maximum gas allowed per transaction on Elrond.
    fn require_number_of_claims_in_bulk_is_valid(&self, number_of_claims: &usize) {
        require!(
            number_of_claims <= &MAX_NUMBER_OF_CLAIMS_PER_OPERATION,
            ERR_MAX_NUMBER_OF_CLAIMS_PER_OPERATION
        );
    }

    // Checks whether the address has the special rights needed in case of some special operations
    fn require_address_is_privileged(&self, address: &ManagedAddress) {
        require!(
            self.privileged_addresses().contains(address)
                || &self.blockchain().get_owner_address() == address,
            ERR_ADDRESS_NOT_AUTHORIZED
        );
    }

    // Checks whether the address is an authorized third party
    fn require_address_is_authorized_third_party(&self, address: &ManagedAddress) {
        require!(
            self.authorized_third_parties().contains(address)
                || &self.blockchain().get_owner_address() == address,
            ERR_ADDRESS_NOT_AUTHORIZED
        );
    }

    // Checks whether the address has the Data NFT Marketplace Special Rights
    fn require_address_has_deposit_rights(&self, address: &ManagedAddress) {
        require!(
            self.privileged_addresses().contains(address)
                || &self.blockchain().get_owner_address() == address
                || self.depositor_addresses().contains(&address),
            ERR_ADDRESS_NOT_AUTHORIZED
        );
    }

    // Checks whether a token is of a fungible type
    fn require_token_is_fungible(&self, payment: &EsdtTokenPayment) {
        require!(
            payment.token_type()==EsdtTokenType::Fungible,
            ERR_TOKEN_IS_NOT_FUNGIBLE
        );
    }

    // Checks whether the factory address is set
    fn require_factory_address_is_set(&self) {
        require!(
            !self.factory_address().is_empty(),
            ERR_FACTORY_ADDRESS_NOT_SET
        );
    }

    // Checks whether the claims contract has a Data NFT creator set in memory
    fn require_data_nft_has_creator_set(&self, token_id: &TokenIdentifier, nonce: u64){
        require!(
            !self.data_nft_creator(token_id, nonce).is_empty(),
            ERR_DATA_NFT_CREATOR_NOT_SET
        );
    }
}
