use units::{Asset, Price, Size, Unit};

mod units;

/// A simple test of the unit system
/// The design is loosely based on the idea of fundamental and derived units in Physics
/// The fundamental units are the assets, and the derived units are the prices and sizes
fn main() {
    // Creating a Price
    let p = Price { value: 2000.0, unit: Unit { asset: Asset::ETH, quote: Asset::USDT } };
    // Two equivalent ways of creating a Size
    let s1 = Size { value: 2.0, unit: Unit { asset: Asset::ETH, quote: Asset::None } };
    let s2 = Size { value: 3.0, unit: Unit { asset: Asset::ETH, ..Unit::default() } };

    // Addition of sizes
    let size_sum = s1 + s2;
    println!("Total {:?}", size_sum); // Total Size { value: 5.0, unit: Unit { asset: ETH, quote: None } }

    // Multiplication of price and size
    let price_size_mul = p * size_sum;
    println!("Notional {:?}", price_size_mul); // Notional Price { value: 10000.0, unit: Unit { asset: USDT, quote: None } }
}
