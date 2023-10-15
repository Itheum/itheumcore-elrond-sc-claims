use claims::{ProxyTrait as _, constants::{ERR_ADDRESS_NOT_AUTHORIZED, ERR_NO_THIRD_PARTY_CLAIMS}};
use core_mx_minter_factory_sc::ProxyTrait;
use multiversx_sc::{
    storage::mappers::SingleValue,
    types::{Address, BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier, MultiValueEncoded},
    codec::multi_types::MultiValue3,
};

use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_model::{
        Account, AddressValue, BytesValue, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep,
        TxExpect,
    },
    ContractInfo, ScenarioWorld,
};

const CLAIMS_TOKEN_ID_EXPR: &str = "str:ITHEUM-abc123";
const DATANFT_TOKEN_ID_EXPR: &str = "str:ITHEUM-abc123";
const CLAIMS_PATH_EXPR: &str = "file:output/claims.wasm";
const FACTORY_PATH_EXPR: &str = "file:/mnt/d/Programming/MultiversX/itheum/core-mx-minter-factory-sc/output/core-mx-minter-factory-sc.wasm";
const FIRST_USER_ADDRESS_EXPR: &str = "address:first-user";
const SECOND_USER_ADDRESS_EXPR: &str = "address:second-user";
const OWNER_ADDRESS_EXPR: &str = "address:owner";
const CLAIMS_CONTRACT_ADDRESS_EXPR: &str = "sc:claims-contract-address";
const TREASURY_ADDRESS_EXPR: &str = "address:treasury-address";
const FACTORY_CONTRACT_ADDRESS_EXPR: &str = "sc:factory-contract-address";
const THIRD_PARTY_CONTRACT_ADDRESS_EXPR: &str = "address:third-party-contract-address";

type ClaimsContract = ContractInfo<claims::Proxy<StaticApi>>;
type FactoryContract = ContractInfo<core_mx_minter_factory_sc::Proxy<StaticApi>>;

fn world() -> ScenarioWorld{
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("..");

    blockchain.register_contract(CLAIMS_PATH_EXPR, claims::ContractBuilder);
    blockchain.register_contract(FACTORY_PATH_EXPR, core_mx_minter_factory_sc::ContractBuilder);

    blockchain
}

struct ClaimsContractState{
    world: ScenarioWorld,
    claims_contract: ClaimsContract,
    factory_contract: FactoryContract,
    first_user_address: Address,
    treasury_address: Address,
    claims_token: TokenIdentifier<StaticApi>,
}

impl ClaimsContractState{
    fn new() -> Self{
        let mut world = world();

        world.set_state_step(
            SetStateStep::new()
            .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1).balance("1_000"))
            .new_address(OWNER_ADDRESS_EXPR, 1, CLAIMS_CONTRACT_ADDRESS_EXPR)
            .new_address(OWNER_ADDRESS_EXPR, 2, FACTORY_CONTRACT_ADDRESS_EXPR)
            .put_account(FIRST_USER_ADDRESS_EXPR, Account::new().nonce(1).balance("1_000"))
            .put_account(SECOND_USER_ADDRESS_EXPR, Account::new().nonce(1).balance("1_000"))
            .put_account(THIRD_PARTY_CONTRACT_ADDRESS_EXPR, Account::new().nonce(1).balance("1_000"))
            .put_account(TREASURY_ADDRESS_EXPR,Account::new().nonce(1).balance("1_000"))
        );

        let claims_contract = ClaimsContract::new(CLAIMS_CONTRACT_ADDRESS_EXPR);
        let first_user_address = AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address();
        let factory_contract = FactoryContract::new(FACTORY_CONTRACT_ADDRESS_EXPR);
        let treasury_address = AddressValue::from(TREASURY_ADDRESS_EXPR).to_address();
        let claims_token = TokenIdentifier::from(CLAIMS_TOKEN_ID_EXPR);

        Self{
            world,
            claims_contract,
            factory_contract,
            first_user_address,
            treasury_address,
            claims_token
        }
    }

    fn deploy(&mut self) -> &mut Self {
        let claims_code = self.world.code_expression(CLAIMS_PATH_EXPR);
        let factory_code = self.world.code_expression(FACTORY_PATH_EXPR);

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(claims_code)
                .call(self.claims_contract.init()),
        );

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(factory_code)
                .call(self.factory_contract.init()),
        );

        self
    }

    fn initialize(&mut self) -> &mut Self{
        self.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
            self.claims_contract.set_claim_token(self.claims_token.clone()),
        ));

        self.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
            self.claims_contract.set_factory_address(self.factory_contract.to_address()),
        ));

        self.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
            self.claims_contract.unpause(),
        ));

        let mut dnc = MultiValueEncoded::<StaticApi,MultiValue3<TokenIdentifier<StaticApi>,u64,ManagedAddress<StaticApi >>>::new();
        dnc.push((TokenIdentifier::from(DATANFT_TOKEN_ID_EXPR), 1, ManagedAddress::from(self.first_user_address.clone())).into());
        self.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
            self.claims_contract.add_data_nft_creators(dnc),
        ));

        self.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
            self.factory_contract.initialize_contract(
                false, 
                self.treasury_address.clone(), 
                1000u64, 
                self.claims_contract.to_address(), 
                self.claims_token.clone()
            ),
        ));

        self
    }
}

#[test]
fn deploy_initialize_test(){
    let mut state = ClaimsContractState::new();
    state.deploy();
    state.initialize();
}

#[test]
fn add_third_party_claim_test(){
    let mut state = ClaimsContractState::new();
    state.deploy();
    state.initialize();

    let third_party = AddressValue::from(THIRD_PARTY_CONTRACT_ADDRESS_EXPR).to_address();
    let first_user_address = AddressValue::from(FIRST_USER_ADDRESS_EXPR).to_address();
    let second_user_address = AddressValue::from(SECOND_USER_ADDRESS_EXPR).to_address();

    state.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).egld_value(1000u64).call(
        state.claims_contract.add_third_party_claim(TokenIdentifier::from(DATANFT_TOKEN_ID_EXPR), 1u64),
    ));

    state.world.sc_call(ScCallStep::new().from(THIRD_PARTY_CONTRACT_ADDRESS_EXPR).egld_value(1000u64).call(
        state.claims_contract.add_third_party_claim(TokenIdentifier::from(DATANFT_TOKEN_ID_EXPR), 1u64),
    ).expect(TxExpect::user_error("str:".to_string() + ERR_ADDRESS_NOT_AUTHORIZED)));

    let third_party = AddressValue::from(THIRD_PARTY_CONTRACT_ADDRESS_EXPR).to_address();
    state.world.sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
        state.claims_contract.authorize_third_party_address(third_party),
    ));

    state.world.sc_call(ScCallStep::new().from(THIRD_PARTY_CONTRACT_ADDRESS_EXPR).egld_value(1000u64).call(
        state.claims_contract.add_third_party_claim(TokenIdentifier::from(DATANFT_TOKEN_ID_EXPR), 1u64),
    ));

    state.world.sc_call(ScCallStep::new().from(THIRD_PARTY_CONTRACT_ADDRESS_EXPR).call(
        state.claims_contract.harvest_third_party_claims(),
    ).expect(TxExpect::user_error("str:".to_string() + ERR_NO_THIRD_PARTY_CLAIMS)));

    state.world.sc_call(ScCallStep::new().from(FIRST_USER_ADDRESS_EXPR).call(
        state.claims_contract.harvest_third_party_claims(),
    ));
}