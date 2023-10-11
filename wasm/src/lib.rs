// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           36
// Async Callback (empty):               1
// Total number of exported functions:  38

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    claims
    (
        init => init
        setClaimToken => set_claim_token
        pause => pause
        unpause => unpause
        addPrivilegedAddress => add_privileged_address
        removePrivilegedAddress => remove_privileged_address
        addDepositorAddress => add_depositor_address
        removeDepositorAddress => remove_depositor_address
        authorizeThirdParty => authorize_third_party_address
        unauthorizeThirdParty => unauthorize_third_party_address
        setFactoryAddress => set_factory_address
        addClaim => add_claim
        addClaims => add_claims
        removeClaim => remove_claim
        removeClaims => remove_claims
        claim => harvest_claim
        addThirdPartyClaim => add_third_party_claim
        claimThirdParty => harvest_third_party_claims
        viewTokenIdentifier => claim_token
        viewClaim => claim
        viewThirdPartyTokenClaims => third_party_token_claims
        viewThirdPartyEgldClaim => third_party_egld_claim
        viewClaimModifyDate => claim_modify_date
        viewThirdPartyClaimModifyDate => third_party_claim_modify_date
        isPaused => is_paused
        viewPrivilegedAddresses => privileged_addresses
        viewDepositorAddresses => depositor_addresses
        getFactoryAddress => factory_address
        getAuthorizedThirdParties => authorized_third_parties
        viewClaims => view_claims
        viewClaimWithDate => view_claims_with_date
        viewClaimsData => view_claims_data
        viewFactoryData => view_factory_data
        factory_treasury_address => factory_treasury_address
        factory_tax => factory_tax
        factory_claims_contract_address => factory_claims_contract_address
        factory_claims_token_identifier => factory_claims_token_identifier
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
