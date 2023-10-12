pub const MAX_NUMBER_OF_CLAIMS_PER_OPERATION: usize = 200;
pub const MAX_NUMBER_OF_PRIVILEGED_ADDRESSES: usize = 2;

pub const ERR_ADDRESS_NOT_AUTHORIZED: &str = "Address not authorized to use this operation";
pub const ERR_TOKEN_NOT_SET: &str = "Claims token is not set";
pub const ERR_TOKEN_SET: &str = "Claims token is already set";
pub const ERR_TOKEN_INCORRECT: &str = "Can only add designated token";
pub const ERR_NON_ZERO_VALUE: &str = "Operation must have non-zero value";
pub const ERR_MORE_THAN_CLAIM: &str = "Cannot remove more than current claim";
pub const ERR_MAX_NUMBER_OF_CLAIMS_PER_OPERATION: &str =
    "Exceeded maximum number of claims per operation";
pub const ERR_CONTRACT_PAUSED: &str = "Contract is paused";
pub const ERR_CONTRACT_ALREADY_PAUSED: &str = "Contract is already paused";
pub const ERR_CONTRACT_ALREADY_UNPAUSED: &str = "Contract is already unpaused";
pub const ERR_ADDRESS_PRIVILEGED: &str = "Address is already privileged";
pub const ERR_ADDRESS_DEPOSITOR: &str = "Address is already a depositor";
pub const ERR_ADDRESS_THIRD_PARTY: &str = "Address is already an authorized third party";
pub const ERR_ADDRESS_NOT_PRIVILEGED: &str = "Address is not privileged";
pub const ERR_ADDRESS_NOT_DEPOSITOR: &str = "Address is not a depositor";
pub const ERR_MAX_NUMBER_OF_PRIVILEGED_ADDRESSES: &str =
    "Exceeded maximum number of privileged addresses";

pub const ERR_OWNER_NOT_PRIVILEGED: &str = "Owner cannot be added to priviledged addresses";
pub const ERR_OWNER_NOT_DEPOSITOR: &str = "Owner cannot be added to depositor addresses";
pub const ERR_OWNER_NOT_THIRD_PARTY: &str = "Owner cannot be added to third parties";
pub const ERR_OWNER_NOT_MINTER: &str = "Owner cannot be added to minters";
pub const ERR_CLAIM_EQUAL_PAYMENT: &str = "Claims added must equal payment amount";
pub const ERR_TOKEN_IS_NOT_FUNGIBLE: &str = "Payment token is not fungible";
pub const ERR_FACTORY_ADDRESS_NOT_SET: &str = "Factory address is not set";
pub const ERR_NO_THIRD_PARTY_CLAIMS: &str = "No third party claims to harvest";
pub const ERR_DATA_NFT_CREATOR_NOT_SET: &str = "Data NFT creator is not set";
pub const ERR_ADDRESS_MINTER: &str = "Address is already a Data NFT Minter";
pub const ERR_ADDRESS_NOT_MINTER: &str = "Address is not a Data NFT Minter";
