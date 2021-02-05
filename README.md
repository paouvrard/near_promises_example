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

### Deploy another contract to a new dev account on testnet
```
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/near_promises_example.wasm
```

### Call the contract at dev account which will return the result of a callback
```
export pr_a=dev-1612502168408-7726225
export pr_b=dev-1612502422059-6077955

near call $pr_a call_b '{"account_id": "'$pr_b'"}' --accountId opa_code.testnet
```