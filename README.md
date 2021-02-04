# NEAR cross contract call promises

### Init project
```
cargo new near_promises_example --lib
```

Edit cargo.toml per [https://github.com/near/near-sdk-rs/blob/master/examples/status-message/Cargo.toml](https://github.com/near/near-sdk-rs/blob/master/examples/status-message/Cargo.toml)

### Compile contract
```
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```

### Deploy contract to a dev account on testnet
```
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/near_promises_example.wasm
```
This creates a new account for the contract named dev-... with a single access keys controlled by near.

### Call the contract at dev account
```
export pr_a=dev-1612432902723-8766979

near call $pr_a call_b '{"account_id": "bb"}' --accountId opa_code.testnet
```