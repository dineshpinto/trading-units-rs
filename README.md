# trading-units-rs

A (very) basic unit system for trading in Rust 🦀.

## Usage

```rust
use units::{Asset, Price, Size, Unit};
```

Creating `Price` and `Size`
```rust
// Creating a Price
let p = Price { value: 2000.0, unit: Unit { asset: Asset::ETH, quote: Asset::USDT } };
// Two equivalent ways of creating a Size
let s1 = Size { value: 2.0, unit: Unit { asset: Asset::ETH, quote: Asset::None } };
let s2 = Size { value: 3.0, unit: Unit { asset: Asset::ETH, ..Unit::default() } };
```

Performing basic operations
```rust
// Addition of sizes
let size_sum = s1 + s2;
println!("Total {:?}", size_sum); 
// Total Size { value: 5.0, unit: Unit { asset: ETH, quote: None } }

// Multiplication of price and size
let price_size_mul = p * size_sum;
println!("Notional {:?}", price_size_mul); 
// Notional Price { value: 10000.0, unit: Unit { asset: USDT, quote: None } }
```