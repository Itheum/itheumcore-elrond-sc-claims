PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"

WALLET="./wallet.pem"

ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)

TOKEN="ITHEUM-fce905"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

deploy(){
    mxpy --verbose contract deploy \
    --bytecode output/claims.wasm \
    --outfile deployOutput \
    --metadata-not-readable \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --send \
    --recall-nonce \
    --outfile="./interaction/deploy-devnet.interaction.json" || return

    TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

# if you interact without calling deploy(), then you need to 1st run this to restore the vars from data
restoreDeployData() {
  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

  # after we upgraded to mxpy 8.1.2, mxpy data parse seems to load the ADDRESS correctly but it breaks when used below with a weird "Bad address" error
  # so, we just hardcode the ADDRESS here. Just make sure you use the "data['contractAddress'] from the latest deploy-devnet.interaction.json file
  ADDRESS="erd1qqqqqqqqqqqqqpgqwu6qz3skzzdnmvnkknjngvrprpt4fwzffsxsr8ecca"
}

setClaimToken(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setClaimToken" \
    --arguments ${TOKEN_HEX} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

pause(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "pause" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

unpause(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "unpause" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addPrivilegedAddress(){
    # $1 = address to which to give privileges

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "addPrivilegedAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

removePrivilegedAddress(){
    # $1 = address to which to remove privileges

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "removePrivilegedAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addClaim(){
    # $1 = amount to add to claim
    # $2 = address to which to attribute the claim
    # $3 = claim type (0 = reward, 1 = aidrop, 2 = allocation, 3 = royalties)

    method="0x$(echo -n 'addClaim' | xxd -p -u | tr -d '\n')"
    address="0x$(mxpy wallet bech32 --decode ${2})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "ESDTTransfer" \
    --arguments ${TOKEN_HEX} $1 $method $address $3 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

removeClaim(){
    # $1 = address from which to remove the claim
    # $2 = claim type (0 = reward, 1 = aidrop, 2 = allocation, 3 = royalties)
    # $3 = amount to remove from claim

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "removeClaim" \
    --arguments $address $2 $3 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

harvestAllFirstPartyClaims(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "claim" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

harvestClaim(){
    # $1 = claim type (0 = reward, 1 = aidrop, 2 = allocation, 3 = royalties)

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "claim" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

# v2.0.0
addDepositorAddress(){
    # $1 = address to which to give privileges

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "addDepositorAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

removeDepositorAddress(){
    # $1 = address to which to remove privileges

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "removeDepositorAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

# v3.0.0
setFactoryAddress(){
    # $1 = address to set as factory

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "setFactoryAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addMinterAddress(){
    # $1 = address of the minter

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "addMinterAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

removeMinterAddress(){
    # $1 = address to remove from the minter list

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "removeMinterAddress" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addDataNftCreatorToMapping(){
    # $1 = token identifier to map
    # $2 = nonce to map
    # $3 = address to map

    token_hex="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    address="0x$(mxpy wallet bech32 --decode ${3})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "addDataNftCreators" \
    --arguments ${token_hex} $2 $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

authorizeThirdParty(){
    # $1 = address to authorize

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "authorizeThirdParty" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

unauthorizeThirdParty(){
    # $1 = address to unauthorize

    address="0x$(mxpy wallet bech32 --decode ${1})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "unauthorizeThirdParty" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addThirdPartyESDTClaim(){
    # $1 = token to add to claim
    # $2 = amount to add to claim
    # $3 = token to which to attribute the claim
    # $4 = nonce to which to attribute the claim

    token_to_claim_hex="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"
    token_hex="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    method="0x$(echo -n 'addThirdPartyClaim' | xxd -p -u | tr -d '\n')"
    address="0x$(mxpy wallet bech32 --decode ${3})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "ESDTTransfer" \
    --arguments ${token_hex} $2 $method $token_to_claim_hex $4 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addThirdPartyMultipleESDTClaim(){
    # $1 = address you are using to send the transaction (your wallet address)
    # $2 = token 1 to add to claim
    # $3 = amount 1 to add to claim
    # $4 = token 2 to add to claim
    # $5 = amount 2 to add to claim
    # $6 = address to which to attribute the claim
    # $7 = token to which to attribute the claim
    # $8 = nonce to which to attribute the claim

    token_to_claim_hex="0x$(echo -n ${7} | xxd -p -u | tr -d '\n')"

    token_hex_1="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    token_hex_2="0x$(echo -n ${4} | xxd -p -u | tr -d '\n')"
    method="0x$(echo -n 'addThirdPartyClaim' | xxd -p -u | tr -d '\n')"
    address="0x$(mxpy wallet bech32 --decode ${6})"
    sc_address="0x$(mxpy wallet bech32 --decode ${ADDRESS})"
    mxpy --verbose contract call ${1} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=20000000 \
    --function "MultiESDTNFTTransfer" \
    --arguments ${sc_address} 2 $token_hex_1 0 $3 $token_hex_2 0 $5 $method $token_to_claim_hex $8 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addThirdPartyEGLDClaim(){
    # $1 = amount to add to claim
    # $2 = address to which to attribute the claim
    # $3 = token to which to attribute the claim
    # $4 = nonce to which to attribute the claim

    token_to_claim_hex="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"

    address="0x$(mxpy wallet bech32 --decode ${2})"
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "addThirdPartyClaim" \
    --arguments $token_to_claim_hex $4 \
    --value $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

harvestThirdPartyClaims(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "claimThirdParty" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

harvestItheumThirdPartyClaims(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "selfClaimThirdParty" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

viewFactoryData(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=3000000 \
    --function "viewFactoryData" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}
