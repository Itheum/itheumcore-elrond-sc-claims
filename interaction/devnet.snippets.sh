PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallet.pem"

ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)

TOKEN="ITHEUM-a61317"
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

harvestAllClaims(){
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