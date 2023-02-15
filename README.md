# Itheum Core Elrond - Claims Smart Contract

## Abstract

The claims smart contract is the tool that stands at the heart of collaboration between Itheum and its community. Whether it's a reward for helping the project, an airdrop, some allocation of tokens or trading royalties, the claims smart contract is the tool that allows Itheum to give tokens to all community members that are using the Elrond blockchain.

## Introduction

This contract allows the owner of it to send tokens to the smart contract and reserve them for a specific address of their choice. There are 4 types of claims that are defined in the smart contract: rewards, airdrops, allocations and royalties. If a user has claims, they can harvest each type individually or can choose to harvest all of them in the same transaction. The contract is designed such that a user can only take their designated tokens from the contract.

## Prerequisites

This documentation assumes the user has previous programming experience. Moreover, the user should have a basic understanding of the Elrond blockchain. If you are new to the blockchain, please refer to the [Elrond documentation](https://docs.elrond.com/). In order to develop Elrond smart contract related solutions, one needs to have installed [erdpy](https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/).

Understanding this document is also easier if one knows how [ESDT token transactions](https://docs.elrond.com/developers/esdt-tokens/#transfers-to-a-smart-contract) are structured on the Elrond blockchain.

## Itheum deployed claims contract addresses

| Devnet                                                         | Mainnet          |
| -------------------------------------------------------------- | ---------------- |
| erd1qqqqqqqqqqqqqpgqtywnp7z0war94rpzk00p2n2wjwaws2xr7yqsejxy7f (V1) : (V2) | erd1qqqqqqqqqqqqqpgqnsmrn5q08eqth3fy8el87sgdj0mkhwdwl2jqnf59cg |

## Endpoints

### Setup endpoints

The setup workflow for the claims smart contract is as follows:

- The SC deployment
- Setting up the claims token.

#### init

```rust
    #[init]
    fn init(&self);
```

The init function is called when deploying or upgrading the smart contract. It receives no arguments and it the only thing it does for the claims smart contract is to pause it.

#### setClaimToken

```rust
    #[endpoint(setClaimToken)]
    fn set_claim_token(&self,
        token: TokenIdentifier
    );
```

Endpoint that sets the claims token. It can only be used once and it can only be called by the owner of the contract.
Call structure: "setClaimToken" + "@" + TokenIdentifier hex encoded
Example: "setClaimToken@49544845554d2d613631333137"

### Only owner endpoints

#### unpause

```rust
    #[endpoint(unpause)]
    fn unpause(&self);
```

Endpoint that unpauses the claims harvesting from the smart contract.
Call structure: "unpause"
Example: "unpause"

#### addPrivilegedAddress

```rust
    #[endpoint(addPrivilegedAddress)]
    fn add_privileged_address(&self,
        address: ManagedAddress
    );
```

Endpoint that gives an address privileges to add claims or pause the contract. The contract can only store up to two privileged addresses at a time.
Call structure: "addPrivilegedAddress" + "@" + Address hex encoded
Example: "addPrivilegedAddress@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### removePrivilegedAddress

```rust
    #[endpoint(removePrivilegedAddress)]
    fn remove_privileged_address(&self,
        address: ManagedAddress
    );
```

Endpoint that removes privileges of an already privileged address.
Call structure: "removePrivilegedAddress" + "@" + Address hex encoded
Example: "removePrivilegedAddress@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### addDepositorAddress

```rust
    #[endpoint(addDepositorAddress)]
    fn add_depositor_address(&self,
        address: ManagedAddress
    );
```

Endpoint that gives an address the right to add claims.
Call structure: "addDepositorAddress" + "@" + Address hex encoded
Example: "addDepositorAddress@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### removeDepositorAddress

```rust
    #[endpoint(removeDepositorAddress)]
    fn remove_depositor_address(&self,
        address: ManagedAddress
    );
```

Endpoint that removes an already added address to the depositor list.
Call structure: "removeDepositorAddress" + "@" + Address hex encoded
Example: "removeDepositorAddress@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### removeClaim

```rust
    #[endpoint(removeClaim)]
    fn remove_claim(&self,
        address: &ManagedAddress,
        claim_type: ClaimType,
        amount: BigUint
    );
```

Endpoint that allows the owner of the smart contract to remove a claim from the smart contract. Receives an address, the claim type and the amount of tokens to remove as arguments.
Call structure: "removeClaim" + "@" +address hex encoded + "@" + claim type hex encoded + "@" + amount to remove hex encoded
Example: "removeClaim@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101@01@8ac7230489e80000"

#### removeClaims

```rust
    #[endpoint(removeClaims)]
    fn remove_claims(&self,
        claims: MultiValueEncoded<MultiValue3<ManagedAddress, ClaimType, BigUint>>,
    );
```

Similar to the removeClaim endpoint, but it allows the owner to remove multiple claims from the smart contract through a single transaction. Receives a list of claims as arguments.
Call structure: "removeClaims" + "@" + address hex encoded + "@" + claim type hex encoded + "@" + amount to remove hex encoded (but can add as many pairs as needed)
Example: "removeClaims@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101@01@8ac7230489e80000"

### Priviledged address endpoints

These endpoints are endpoints that are callable by both the owner of the Smart Contract and up to two other addresses designated by the owner to have extra privileges.

#### pause

```rust
    #[endpoint(pause)]
    fn pause(&self);
```

Endpoint that pauses the claims harvesting from the smart contract.
Call structure: "pause"
Example: "pause"

#### addClaim

```rust
    #[payable("*")]
    #[endpoint(addClaim)]
    fn add_claim(&self,
        address: &ManagedAddress,
        claim_type: ClaimType
    );
```

Endpoint that allows the owner of the smart contract to add a claim to the smart contract. Receives an address and the claim type as arguments. The claim is set for the address and the claim type received as arguments.
Call structure:"ESDTTransfer"+ "@" + TokenIdentifier hex encoded + "@" + amount hex encoded + "@" + "addClaim" hex encoded + "@" + address hex encoded + "@" + claim type hex encoded
Example: "ESDTTransfer@49544845554d2d613631333137@8ac7230489e80000@616464436c61696d@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101@00"

#### addClaims

```rust
    #[payable("*")]
    #[endpoint(addClaims)]
    fn add_claims(&self,
        claims: MultiValueEncoded<MultiValue3<ManagedAddress, ClaimType, BigUint>>
    );
```

Similar to the addClaim endpoint, but it allows the owner to add multiple claims to the smart contract through a single transaction. Receives a list of claims as arguments.
Call structure: "ESDTTransfer" + "@" + TokenIdentifier hex encoded + "@" + total amounts of tokens added to claims hex encoded + "@" + "addClaims" hex encoded + "@" + address hex encoded + "@" + claim type hex encoded + "@" + amount for this address hex encoded (but can add as many address/claim type/amount pairs as needed)
Example: "ESDTTransfer@49544845554d2d613631333137@8ac7230489e80000@616464436c61696d73@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101@00@8ac7230489e80000"

### Public endpoints

#### claim

```rust
    #[endpoint(claim)]
    fn harvest_claim(&self,
        claim_type: OptionalValue<ClaimType>
    );
```

Endpoint that allows anyone to harvest their designated claims. Allows the user to input a claim type as argument, but that argument is optional. If no claim type is provided, the user will receive all claims attributed to themseles. If a claim type is provided as argument, the user will only receive that claim type.

Call structure without claim type: "harvestClaim"
Example without claim type: "harvestClaim"

Call structure wit claim type: "harvestClaim" + "@" + claim type hex encoded
Example with claim type: "harvestClaim@02"

## Development

This smart contract, albeit being a simple one, aims to set the standard when it comes to the quality of testing and documentation for which smart contract developers should aim. The above average level of documentation present aims specifically to take advantage of our open source codebase in order to learn, contribute and take good practices from the smart contract.

### Architecture

The Claims Smart Contract is structured in 5 files:

- events: This files has all the defined events of the smart contract. They are emitted whenever something relevant happens in the smart contract. Their role is to make debugging and logging easier and to allow data collecting based on the smart contract.
- storage: This file has all the storage/memory declaration of the smart contract. This is the main file that allows the smart contract to save data in the blockchain.
- views: This file contains all the read-only endpoints of the smart contract. These endpoints are used to retrieve relevant data from the smart contract.
- requirements: This file contains requirements for the endpoints of the smart contract. In order to avoid code duplication, encourage a healthy project structure and increase code readability we have decided to separate most of the requirements that would otherwise have been duplicated from the endpoints and put them here.
- lib: This is the main file of the smart contract, where all the logic of the smart contract is implemented. This connects all the other files (modules) and uses them to implement what is the claims contract itself.

### How to test

The tests are located in the tests folder, in the rust_tests file. In order to run the tests one can use the command:

```shell
    cargo test --package claims --test rust_tests -- --nocapture
```

Another way of running the tests is by using the rust-analyzer extension in Visual Studio Code, which is also very helpful for Elrond Smart Contract development. If one has the extension installed, they can go open and go to the top of the rust_tests file and click the Run Tests button.

Note: In order to run the tests, one has to use the rust nightly version. One can switch to the nightly version by using:

```shell
    rustup default nightly
```

### How to deploy

In order to deploy the smart contract on devnet one can use the interaction snippets present in the devnet.snippets file (which is located in the interactions folder). Before using the snippets, make sure to add your pem file in the root of the project under the name "wallet.pem" (or change the name to whichever one you wish to use in the interaction snippets). If you need info about how to derive a pem file you can find them [here](https://docs.elrond.com/sdk-and-tools/erdpy/deriving-the-wallet-pem-file/). To run the functions from the interaction file, one can use:

```shell
    source interaction/devnet.snippets.sh
```

After using that, to deploy one can simply use:

```shell
    deploy
```

### How to interact

After deployment, one can interact with the smart contract and test its functionality. To do so, one can use the interaction snippets already presented above. More explanations can be found about the snippets inside the devnet.snippets file.

## Contributing

Feel free the contact the development team if you wish to contribute or if you have any questions. If you find any issues, please report them in the Issues sections of the repository. You can also create your own pull requests which will be analyzed by the team.
