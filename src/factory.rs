multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Factory of Data NFT Minter contract proxy
mod factory_contract_proxy {

    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[multiversx_sc::proxy]
    pub trait FactoryProxy {
        #[view(getTreasuryAddress)]
        fn get_factory_treasury_address(&self);

        #[view(getTax)]
        fn get_factory_tax(&self);

        #[view(getClaimsContractAddress)]
        fn get_factory_claims_contract_address(&self);

        #[view(getClaimsTokenIdentifier)]
        fn get_factory_claims_token_identifier(&self);
    }
}

#[multiversx_sc::module]
pub trait FactoryContractProxyMethods: crate::storage::StorageModule {
    #[proxy]
    fn factory_proxy(&self, sc_address: ManagedAddress)
        -> factory_contract_proxy::Proxy<Self::Api>;

    #[endpoint]
    fn factory_treasury_address(&self) -> ManagedAddress {
        let factory_proxy_address = self.factory_address().get();
        self.factory_proxy(factory_proxy_address)
            .get_factory_treasury_address()
            .execute_on_dest_context::<ManagedAddress>()
    }

    #[endpoint]
    fn factory_tax(&self) -> BigUint {
        let factory_proxy_address = self.factory_address().get();
        self.factory_proxy(factory_proxy_address)
            .get_factory_tax()
            .execute_on_dest_context::<BigUint>()
    }

    #[endpoint]
    fn factory_claims_contract_address(&self) -> ManagedAddress {
        let factory_proxy_address = self.factory_address().get();
        self.factory_proxy(factory_proxy_address)
            .get_factory_claims_contract_address()
            .execute_on_dest_context::<ManagedAddress>()
    }

    #[endpoint]
    fn factory_claims_token_identifier(&self) -> TokenIdentifier {
        let factory_proxy_address = self.factory_address().get();
        self.factory_proxy(factory_proxy_address)
            .get_factory_claims_token_identifier()
            .execute_on_dest_context::<TokenIdentifier>()
    }
}