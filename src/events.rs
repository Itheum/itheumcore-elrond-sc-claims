elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::ClaimType;

//Module that handles event emitting for important smart contract event in order to facilitate logging, debugging and monitoring with ease
#[elrond_wasm::module]
pub trait EventsModule {
    //Emitted whenever the owner adds a new claim to the smart contract
    #[event("claimAdded")]
    fn claim_added_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] claim_type: &ClaimType,
        amount: &BigUint,
    );

    //Emitted whenever the owner removes a claim from the smart contract
    #[event("claimRemoved")]
    fn claim_removed_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] claim_type: &ClaimType,
        amount: &BigUint,
    );

    //Emitted whenever an address harvests a claim from the smart contract
    #[event("claimCollected")]
    fn claim_collected_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] claim_type: &ClaimType,
        amount: &BigUint,
    );
}
