Synnax is an attempt to provide a front for Cosmos-SDK using Rust. 


## Known limitations.
Synnax for the moment only support Cosmos queries using REST transport (LCD).


## Examples

### Balance querying

```rust
// Create a lcd transport using 
let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();

let address = "ki1....cafe".to_string();

// Load cosmos interface using the lcd as transport 
let cosmos = Cosmos::new(&lcd);

// query balance for address
let balances = cosmos
    .bank
    .balances(address)
    .unwrap();

// print the result
println!("balance for {} {:#?}", balances);
```

### Contract Helper

```rust

```