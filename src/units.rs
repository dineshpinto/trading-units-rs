use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Asset {
    BTC,
    ETH,
    SOL,
    USDT,
    USDC,
    None,
}

/// A basic struct to contain fundamental Units
/// Each unit has two parts -- an asset and quote
/// Basic operations of addition, subtraction and multiplication
/// is derived for these units
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Unit {
    pub(crate) asset: Asset,
    pub(crate) quote: Asset,
}

impl Default for Unit {
    fn default() -> Unit {
        Unit {
            asset: Asset::None,
            quote: Asset::None,
        }
    }
}

impl Add for Unit {
    type Output = Unit;

    fn add(self, other: Unit) -> Unit {
        if self.asset == other.asset && self.quote == other.quote {
            Unit {
                asset: self.asset,
                quote: self.quote,
            }
        } else {
            panic!("Cannot add units {:?} with {:?}", self, other)
        }
    }
}

impl Sub for Unit {
    type Output = Unit;

    fn sub(self, other: Unit) -> Unit {
        if self.asset == other.asset && self.quote == other.quote {
            Unit {
                asset: self.asset,
                quote: self.quote,
            }
        } else {
            panic!("Cannot subtract units {:?} with {:?}", self, other)
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, other: Unit) -> Unit {
        if self.asset == other.asset && (self.quote == Asset::None || other.quote == Asset::None) {
            Unit {
                asset: self.quote,
                quote: Asset::None,
            }
        } else {
            panic!("Cannot multiply units {:?} with {:?}", self, other)
        }
    }
}


/// A size struct is used to
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Size {
    pub(crate) value: f64,
    pub(crate) unit: Unit,
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        Size {
            value: self.value + other.value,
            unit: self.unit + other.unit,
        }
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Size {
        Size {
            value: self.value - other.value,
            unit: self.unit - other.unit,
        }
    }
}

impl Mul<Price> for Size {
    type Output = Price;

    fn mul(self, other: Price) -> Price {
        Price {
            value: self.value * other.value,
            unit: self.unit * other.unit,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Price {
    pub(crate) value: f64,
    pub(crate) unit: Unit,
}

impl Mul<Size> for Price {
    type Output = Price;

    fn mul(self, other: Size) -> Price {
        Price {
            value: self.value * other.value,
            unit: self.unit * other.unit,
        }
    }
}
