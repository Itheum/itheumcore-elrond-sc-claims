elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait RequirementsModule: crate::storage::StorageModule {
    fn require_reward_token_is_set(&self) {
        require!(!self.reward_token().is_empty(), "Reward token is not set");
    }

    fn require_token_is_reward(&self, token: TokenIdentifier) {
        require!(
            token == self.reward_token().get(),
            "Can only add designated token"
        );
    }

    fn require_value_not_zero(&self, value: &BigUint) {
        require!(
            value > &BigUint::zero(),
            "Operations must have non-zero value"
        );
    }

    fn require_remove_claim_is_valid(&self, current_claim: &BigUint, amount: &BigUint) {
        require!(
            current_claim >= amount,
            "Cannot remove more than current claim"
        );
    }
}
