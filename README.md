# Itheum Core Elrond - Claim Smart Contract
The core itheum elrond smart contract for `claims`

### How to Dev
## Build Environment
- You need `erdpy` on your system. Install as per `https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/`. (pay attention to min python version)
- On Mac, this method worked `https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/`. After installation, you need to restart your terminal session. If you are using ZSH shell, then this will work `source ~/.zshrc`. After this is done, to test if it works, you should be able to run `erdpy` anywhere and get a response. You `erdpy --version` to find version.

## Build App
- Clone the repo

## Build via IDE
- The framework is designed to be easiest to use with the Elrond IDE VSCode extension: https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide

## Manual build
- To build a smart contract without the IDE, run the following command in the project root:

```
./build-wasm.sh
```

### How to Test
- To run tests you need to swap to the nightly via `rustup default nightly`
- In VS-code, install the `rust-analyzer extension`. You also need `rustup` which installs rust on your system. See here for requirements `https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer` 
- In VS-code you can go on the tests folder and at the top of the first line of the empty_rust_test file you should have a `run tests` button. Click it
- Note: You can also run tests like so: `cargo test --package claims --test empty_rust_test --  --nocapture`

### Deployed Contract Addresses
Devnet | Mainnet
--- | --- 
0x | 0x


## Known Issues
