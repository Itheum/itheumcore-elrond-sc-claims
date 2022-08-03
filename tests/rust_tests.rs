use claims::storage::StorageModule;
use claims::*;
use elrond_wasm::{
    elrond_codec::multi_types::{MultiValue3, OptionalValue},
    types::{Address, MultiValueEncoded},
};

use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, testing_framework::*,
    DebugApi,
};
pub const WASM_PATH: &'static str = "../output/claims.wasm";
pub const TOKEN_ID: &[u8] = b"ITHEUM-df6f26";
pub const WRONG_TOKEN_ID: &[u8] = b"WRONG-123456";
pub const OWNER_EGLD_BALANCE: u64 = 100_000_000;

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> claims::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper: ContractObjWrapper<claims::ContractObj<DebugApi>, ContractObjBuilder>,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub third_user_address: Address,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> claims::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let first_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let second_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let third_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE));
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );
    blockchain_wrapper.set_esdt_balance(&owner_address, TOKEN_ID, &rust_biguint!(5_000_000));
    blockchain_wrapper.set_esdt_balance(&owner_address, WRONG_TOKEN_ID, &rust_biguint!(1_000_000));
    blockchain_wrapper.set_esdt_balance(&first_user_address, TOKEN_ID, &rust_biguint!(1_000));
    blockchain_wrapper.set_esdt_balance(&second_user_address, TOKEN_ID, &rust_biguint!(0));
    blockchain_wrapper.set_esdt_balance(&third_user_address, TOKEN_ID, &rust_biguint!(1_000));

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();
    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.set_claim_token(managed_token_id!(TOKEN_ID));
        })
        .assert_ok();

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.add_privileged_address(managed_address!(&first_user_address));
        })
        .assert_ok();

    blockchain_wrapper
        .execute_query(&cf_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true);
        })
        .assert_ok();

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.unpause();
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        third_user_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test] //Tests whether the contrat is deployed and initialized correctly after deployment
fn deploy_test() {
    let mut setup = setup_contract(claims::contract_obj);
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init();
            },
        )
        .assert_ok();
}

#[test] //Tests wether pausing and unpausing the contract works correctly
        //Tests wether trying to change the pause state to the already set state returns an error
        //Tests wether privileged addresses can pause harvesting, but normal addresses cannnot
fn pause_unpause_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let second_user_address = &setup.second_user_address;

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unpause();
            },
        )
        .assert_user_error("Contract is already unpaused");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.pause();
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.pause();
            },
        )
        .assert_user_error("Contract is already paused");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unpause();
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.pause();
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unpause();
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.pause();
            },
        )
        .assert_user_error("Address doesn't have the privilege to use this operation");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false);
        })
        .assert_ok();
}

#[test] //Tests wether adding and removing privileged addresses works as expected
        //Tests if trying give privileges to an address that already has them returns an error
        //Tests if trying to offer privileges to the owner of the smart contract returns an error
        //Tests if trying to remove the privileges that is not privileged returns an error
fn add_and_remove_privileged_addresses_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_addr = &setup.first_user_address;
    let second_user_addr = &setup.second_user_address;
    let third_user_addr = &setup.third_user_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.add_privileged_address(managed_address!(second_user_addr));
            },
        )
        .assert_ok();
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.add_privileged_address(managed_address!(third_user_addr));
            },
        )
        .assert_user_error("Maximum number of priviledged addresses reached");
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.remove_privileged_address(managed_address!(third_user_addr));
            },
        )
        .assert_user_error("Address is not privileged");
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.privileged_addresses()
                    .contains(&managed_address!(first_user_addr))
                    && sc
                        .privileged_addresses()
                        .contains(&managed_address!(second_user_addr)),
                true
            );
        })
        .assert_ok();
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.remove_privileged_address(managed_address!(second_user_addr));
            },
        )
        .assert_ok();
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.add_privileged_address(managed_address!(owner_address));
            },
        )
        .assert_user_error("Owner cannot be added to priviledged addresses");
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.privileged_addresses()
                    .contains(&managed_address!(first_user_addr))
                    && !sc
                        .privileged_addresses()
                        .contains(&managed_address!(second_user_addr)),
                true
            );
        })
        .assert_ok();
}

#[test] //Tests wether adding and removing singular claims works as expected
        //Tests if adding and removing a zero value claim returns an error
        //Tests if removing more than the amount reserved in claims returns an error
fn add_and_remove_claim_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let user_addr = &setup.first_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_user_error("Operations must have non-zero value");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.remove_claim(
                    &managed_address!(user_addr),
                    storage::ClaimType::Airdrop,
                    managed_biguint!(500_000),
                );
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                500_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.remove_claim(
                    &managed_address!(user_addr),
                    storage::ClaimType::Airdrop,
                    managed_biguint!(0),
                );
            },
        )
        .assert_user_error("Operations must have non-zero value");

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.remove_claim(
                    &managed_address!(user_addr),
                    storage::ClaimType::Airdrop,
                    managed_biguint!(700_000),
                );
            },
        )
        .assert_user_error("Cannot remove more than current claim");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                500_000
            );
        })
        .assert_ok();
}

#[test] //Same tests as the ones for singular claims, but for multiple claims
        //Tests if adding multiple claims, but not sending the right amount of tokens for it returns an error
        //Tests if adding or removing zero valued claims returns an error
        //Tests if removing more than the amount reserved in claims returns an error
fn add_and_remove_claims_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_addr = &setup.first_user_address;
    let second_user_addr = &setup.second_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(2_000_000),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(1_000_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(1_000_000),
                    )
                        .into(),
                ));
                sc.add_claims(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(first_user_addr),
                    &storage::ClaimType::Airdrop
                )
                .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(second_user_addr),
                    &storage::ClaimType::Allocation
                )
                .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_700_000),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(1_000_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(1_000_000),
                    )
                        .into(),
                ));
                sc.add_claims(args);
            },
        )
        .assert_user_error("Claims added must equal payment amount");

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(201_000),
            |sc| {
                let mut args = MultiValueEncoded::new();
                for _i in 0..201 {
                    args.push(MultiValue3(
                        (
                            managed_address!(first_user_addr),
                            storage::ClaimType::Airdrop,
                            managed_biguint!(1_000),
                        )
                            .into(),
                    ));
                }
                sc.add_claims(args);
            },
        )
        .assert_user_error("Exceeded maximum number of claims per operation (200)");

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_700_000),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(1_700_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(0),
                    )
                        .into(),
                ));
                sc.add_claims(args);
            },
        )
        .assert_user_error("Operations must have non-zero value");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(first_user_addr),
                    &storage::ClaimType::Airdrop
                )
                .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(second_user_addr),
                    &storage::ClaimType::Allocation
                )
                .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(700_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(500_000),
                    )
                        .into(),
                ));
                sc.remove_claims(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(first_user_addr),
                    &storage::ClaimType::Airdrop
                )
                .get(),
                300_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(second_user_addr),
                    &storage::ClaimType::Allocation
                )
                .get(),
                500_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(200_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(0),
                    )
                        .into(),
                ));
                sc.remove_claims(args);
            },
        )
        .assert_user_error("Operations must have non-zero value");

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(400_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(500_000),
                    )
                        .into(),
                ));
                sc.remove_claims(args);
            },
        )
        .assert_user_error("Cannot remove more than current claim");

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                for _i in 0..201 {
                    args.push(MultiValue3(
                        (
                            managed_address!(first_user_addr),
                            storage::ClaimType::Airdrop,
                            managed_biguint!(1_000),
                        )
                            .into(),
                    ));
                }
                sc.remove_claims(args);
            },
        )
        .assert_user_error("Exceeded maximum number of claims per operation (200)");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(first_user_addr),
                    &storage::ClaimType::Airdrop
                )
                .get(),
                300_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(second_user_addr),
                    &storage::ClaimType::Allocation
                )
                .get(),
                500_000
            );
        })
        .assert_ok();
}

#[test] //Tests wether privileged addresses can add a claim, but a non-priviledged address cannot
fn add_claim_privileged_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let user_addr = &setup.first_user_address;
    let user_addr_3 = &setup.third_user_address;
    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_ok();
    b_wrapper
        .execute_esdt_transfer(
            user_addr_3,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_user_error("Address doesn't have the privilege to use this operation");
}

#[test] //Tests wether privileged addresses can add claims, but a non-priviledged address cannot
fn add_claims_privileged_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let user_addr = &setup.first_user_address;
    let user_addr_3 = &setup.third_user_address;
    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(600),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(user_addr_3),
                        storage::ClaimType::Allocation,
                        managed_biguint!(400),
                    )
                        .into(),
                ));
                sc.add_claims(args);
            },
        )
        .assert_ok();
    b_wrapper
        .execute_esdt_transfer(
            user_addr_3,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(600),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(user_addr_3),
                        storage::ClaimType::Allocation,
                        managed_biguint!(400),
                    )
                        .into(),
                ));
                sc.add_claims(args);
            },
        )
        .assert_user_error("Address doesn't have the privilege to use this operation");
}
#[test] //Tests whether the transaction to add a token fails in the case in which a different token than the claim token is sent
fn add_claim_wrong_token_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let user_addr = &setup.first_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            WRONG_TOKEN_ID,
            0,
            &rust_biguint!(1_000_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_user_error("Can only add designated token");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                0
            );
        })
        .assert_ok();
}

#[test] //Tests whether the transaction to add tokens fails in the case in which a different token than the claim token is sent
fn add_claims_wrong_token_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_addr = &setup.first_user_address;
    let second_user_addr = &setup.second_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            WRONG_TOKEN_ID,
            0,
            &rust_biguint!(500_000),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue3(
                    (
                        managed_address!(first_user_addr),
                        storage::ClaimType::Airdrop,
                        managed_biguint!(200_000),
                    )
                        .into(),
                ));
                args.push(MultiValue3(
                    (
                        managed_address!(second_user_addr),
                        storage::ClaimType::Allocation,
                        managed_biguint!(300_000),
                    )
                        .into(),
                ));
                sc.add_claims(args);
            },
        )
        .assert_user_error("Can only add designated token");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(first_user_addr),
                    &storage::ClaimType::Airdrop
                )
                .get(),
                0
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(
                    &managed_address!(second_user_addr),
                    &storage::ClaimType::Allocation
                )
                .get(),
                0
            );
        })
        .assert_ok();
}

#[test] //Tests whether one can set the claim token only once
fn reset_claim_token_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_claim_token(managed_token_id!(TOKEN_ID));
            },
        )
        .assert_user_error("Claim token is already set");
}

#[test] //Tests whether claiming is impossible in pause state
fn harvest_claim_in_pause_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let user_addr = &setup.second_user_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.pause();
            },
        )
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.harvest_claim(OptionalValue::Some(storage::ClaimType::Airdrop));
            },
        )
        .assert_user_error("Contract is paused");
}

#[test] //Tests whether users can claim
fn harvest_claim_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let user_addr = &setup.second_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.harvest_claim(OptionalValue::Some(storage::ClaimType::Airdrop));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                0
            );
        })
        .assert_ok();

    b_wrapper.check_esdt_balance(user_addr, TOKEN_ID, &rust_biguint!(1_000_000));
}

#[test] //Test wether the transaction to claim returns an error if no claims are present for the user for the type he tries to claim
fn harvest_wrong_claim_type_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let user_addr = &setup.second_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.harvest_claim(OptionalValue::Some(storage::ClaimType::Reward));
            },
        )
        .assert_user_error("Operations must have non-zero value");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                1_000_000
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Reward)
                    .get(),
                0
            );
        })
        .assert_ok();

    b_wrapper.check_esdt_balance(user_addr, TOKEN_ID, &rust_biguint!(0));
}

#[test] //Tests whether claiming all claim types at once works
fn harvest_all_claims_test() {
    let mut setup = setup_contract(claims::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let user_addr = &setup.second_user_address;

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Airdrop);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            owner_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(1_000_000),
            |sc| {
                sc.add_claim(&managed_address!(user_addr), storage::ClaimType::Reward);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            user_addr,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(0),
            |sc| {
                sc.harvest_claim(OptionalValue::None);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Airdrop)
                    .get(),
                0
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.claim(&managed_address!(user_addr), &storage::ClaimType::Reward)
                    .get(),
                0
            );
        })
        .assert_ok();

    b_wrapper.check_esdt_balance(user_addr, TOKEN_ID, &rust_biguint!(2_000_000));
}
