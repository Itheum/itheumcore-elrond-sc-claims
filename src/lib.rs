#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod data_nft;

#[multiversx_sc::contract]
pub trait SimpleClaimInteractionContract: data_nft::ItheumDataNftModule
{
    #[init]
    fn init(&self) {
    }

    // Setup endpoint that sets the Itheum dependencies
    // Should be called ONCE before using any other endpoint/function
    #[only_owner]
    #[endpoint(initializeItheumDependencies)]
    fn init_itheum(&self, royalties_contract: &ManagedAddress, data_nft: &TokenIdentifier) {
        self.set_itheum_dependencies(royalties_contract, data_nft);
    }

    // Simple endpoint that takes payments (eGLD, ESDT or multiple ESDTs) and sends them to the claims contract
    #[only_owner]
    #[payable("*")]
    #[endpoint(sendItheumDataNftRoyalties)]
    fn send_itheum_data_nft_royalties(&self, token_id: &TokenIdentifier, nonce: u64) {
        require!(self.is_data_nft(token_id), "Token is not a Data NFT");

        let egld_payment = self.call_value().egld_value().clone_value();
        let esdt_payments = self.call_value().all_esdt_transfers();
        require!(egld_payment > BigUint::from(0u64) || esdt_payments.len() > 0usize, "No payment received");

        if egld_payment > BigUint::from(0u64){

            self.send_itheum_data_nft_royalties_egld(token_id, nonce, egld_payment);

        }else{

            if esdt_payments.len() == 1usize{

                self.send_itheum_data_nft_royalties_esdt(token_id, nonce, esdt_payments.get(0usize));
            
            }else{

                self.send_itheum_data_nft_royalties_multiple_esdt(token_id, nonce, esdt_payments.clone_value());
            
            }
        }
    }

    // Endpoint that takes an ESDT payment
    // Checks if the received payment is a Data NFT 
    // Sends that payment back to the caller of the endpoint
    #[payable("*")]
    #[endpoint(isTokenDataNFT)]
    fn is_token_data_nft(&self) -> bool {
        let payment = self.call_value().single_esdt();
        let should_send_royalties_for_this_token = self.is_data_nft(&payment.token_identifier);

        let caller = self.blockchain().get_caller();
        self.send().direct_esdt(&caller, &payment.token_identifier, payment.token_nonce, &payment.amount);

        should_send_royalties_for_this_token
    }
}
