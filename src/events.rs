multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage::ClaimType;

// Module that handles event emitting for important smart contract events in order to facilitate logging, debugging and monitoring with ease
#[multiversx_sc::module]
pub trait EventsModule {
    // Emitted whenever a privileged address pauses claim harvesting
    #[event("harvestPaused")]
    fn harvest_paused_event(&self, #[indexed] operator: &ManagedAddress);

    // Emitted whenever the owner unpauses claim harvesting
    #[event("harvestUnpaused")]
    fn harvest_unpaused_event(&self);

    // Emitted whenever the owner adds a privileged address
    #[event("privilegedAddressAdded")]
    fn privileged_address_added_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever the owner removes a privileged address
    #[event("privilegedAddressRemoved")]
    fn privileged_address_removed_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever the owner sets the address (e.g. Data NFT marketplace contract)
    #[event("depositorAddressAdded")]
    fn depositor_address_added_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever the owner clears the address (e.g. Data NFT marketplace contract)
    #[event("depositorAddressRemoved")]
    fn depositor_address_removed_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever the owner authorizes a third party  (e.g. marketplace contract)
    #[event("thirdPartyAddressAuthorized")]
    fn third_party_address_authorized_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever the owner unauthorizes a third party (e.g. marketplace contract)
    #[event("thirdPartyAddressUnauthorized")]
    fn third_party_address_unauthorized_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever a new claim is added to the smart contract
    #[event("claimAdded")]
    fn claim_added_event(
        &self,
        #[indexed] operator: &ManagedAddress,
        #[indexed] address: &ManagedAddress,
        #[indexed] claim_type: &ClaimType,
        #[indexed] amount: &BigUint,
    );

    // Emitted whenever a claim is removed from the smart contract
    #[event("claimRemoved")]
    fn claim_removed_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] claim_type: &ClaimType,
        #[indexed] amount: &BigUint,
    );

    // Emitted whenever an address harvests a claim from the smart contract
    #[event("claimCollected")]
    fn claim_collected_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] claim_type: &ClaimType,
        #[indexed] amount: &BigUint,
    );

    // Emitted whenever a third party claim is added to the smart contract
    #[event("thirdPartyClaimAdded")]
    fn third_party_claim_added_event(
        &self,
        #[indexed] adder: &ManagedAddress,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    // Emitted whenever a third party claim is collected
    #[event("thirdPartyClaimCollected")]
    fn third_party_claim_collected_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );
}
