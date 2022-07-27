elrond_wasm::imports!();
elrond_wasm::derive_imports!();

//Module that handles generic (commonly used, which are not specific to one function) requirements which should stop execution and rollback if not met
#[elrond_wasm::module]
pub trait RequirementsModule: crate::storage::StorageModule {
    // Checks whether the owner of the smart contract designated a token to be used by the smart contract for all the claims
    fn require_claim_token_is_set(&self) {
        require!(!self.claim_token().is_empty(), "Claims token is not set");
    }

    //Checks whether a given token identifier is equal to the token identifier of the token used by the smart contract claims
    fn require_token_is_correct(&self, token: TokenIdentifier) {
        require!(
            token == self.claim_token().get(),
            "Can only add designated token"
        );
    }

    //Checks whether a value is not zero
    fn require_value_not_zero(&self, value: &BigUint) {
        require!(
            value > &BigUint::zero(),
            "Operations must have non-zero value"
        );
    }

    //Checks whether a claim that is intended to be removed is smaller than the amount reserved in the claim
    fn require_remove_claim_is_valid(&self, current_claim: &BigUint, amount: &BigUint) {
        require!(
            current_claim >= amount,
            "Cannot remove more than current claim"
        );
    }

    //Checks whether the number of claims added or removed is smaller than 200. Implemented in order to ensure no call will fail due to consuming more than the maxium gas allowed per transaciton on Elrond.
    fn require_number_of_claims_in_bulk_is_valid(&self, number_of_claims: &usize) {
        require!(
            number_of_claims <= &200usize,
            "Exceeded maximum number of claims per operation (200)"
        );
    }
}
