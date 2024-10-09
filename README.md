```
near deploy --accountId test22222222.testnet --wasmFile nearderthal.wasm
```

```
near call test22222222.testnet new_default_meta '{"owner_id": "test22222222.testnet", "total_supply": "1000000000"}' --accountId test22222222.testnet --gas 30 --deposit 0
```

```
near call test22222222.testnet storage_deposit '{"account_id": "test22222222.testnet"}' --accountId test22222222.testnet --gas 30 --deposit 1
```
