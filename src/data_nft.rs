multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod itheum_data_nft_royalties_proxy {

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[multiversx_sc::proxy]
    pub trait ClaimsProxy {
        #[payable("*")]
        #[endpoint(addThirdPartyClaim)]
        fn send_data_nft_royalties(&self, token_id: &TokenIdentifier, nonce: u64);
    }
}

#[multiversx_sc::module]
pub trait ItheumDataNftModule {
    fn set_itheum_dependencies(&self, contract: &ManagedAddress, data_nft: &TokenIdentifier) {
        self.itheum_royalties_sc().set(contract);
        self.itheum_data_nft_id().set(data_nft);
    }

    fn send_itheum_data_nft_royalties_egld(&self, token_id: &TokenIdentifier, nonce: u64, amount: BigUint) {
        
        self.require_itheum_dependencies_init();

        let contract_address = self.itheum_royalties_sc().get();
        self.itheum_proxy(contract_address)
            .send_data_nft_royalties(token_id, nonce)
            .with_gas_limit(20000000u64)
            .with_egld_transfer(amount)
            .transfer_execute();
    }

    fn send_itheum_data_nft_royalties_esdt(&self, token_id: &TokenIdentifier, nonce: u64, payment: EsdtTokenPayment) {
        
        self.require_itheum_dependencies_init();

        let contract_address = self.itheum_royalties_sc().get();
        self.itheum_proxy(contract_address)
            .send_data_nft_royalties(token_id, nonce)
            .with_gas_limit(20000000u64)
            .with_esdt_transfer(payment)
            .transfer_execute();
    }

    fn send_itheum_data_nft_royalties_multiple_esdt(&self, token_id: &TokenIdentifier, nonce: u64, payments: ManagedVec<EsdtTokenPayment>) {
        
        self.require_itheum_dependencies_init();
        require!(payments.len()<=100usize,"Maximum 100 payments allowed");

        let contract_address = self.itheum_royalties_sc().get();
        self.itheum_proxy(contract_address)
            .send_data_nft_royalties(token_id, nonce)
            .with_gas_limit(20000000u64 + 2000000u64 * (payments.len() as u64))
            .with_multi_token_transfer(payments)
            .transfer_execute();
    }

    fn require_itheum_dependencies_init(&self){
        require!(self.itheum_royalties_sc().is_empty()==false,"Itheum royalties contract address not set");
        require!(self.itheum_data_nft_id().is_empty()==false,"Itheum Data NFT token identifier not set");
    }

    fn is_data_nft(&self, nft_id: &TokenIdentifier) -> bool {
        self.require_itheum_dependencies_init();
        &self.itheum_data_nft_id().get() == nft_id
    }

    #[view(getItheumDataNftRoyaltiesContractAddress)]
    #[storage_mapper("itheum_claims_contract")]
    fn itheum_royalties_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getItheumDataNftTokenId)]
    #[storage_mapper("itheum_data_nft_token_id")]
    fn itheum_data_nft_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[proxy]
    fn itheum_proxy(&self, sc_address: ManagedAddress)
        -> itheum_data_nft_royalties_proxy::Proxy<Self::Api>;
}