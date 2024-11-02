use std::{fmt::Display, str::FromStr};

use leptos::logging::log;
use num_bigint::BigUint;
use num_rational::BigRational;
use starknet_types_core::felt::Felt;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FeltFraction {
    pub numer: FeltParts,
    pub denom: FeltParts,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FeltParts {
    pub low: Felt,
    pub high: Felt,
}

impl Display for FeltFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}/{}) / ({}/{})",
            self.numer.low, self.numer.high, self.denom.low, self.denom.high
        )
    }
}

fn integer_decode(value: f64) -> (u64, i16, i8) {
    let bits: u64 = value.to_bits();
    let sign = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

impl FromStr for FeltFraction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('/') {
            log!("contains /");
            let parts = s.split('/').collect::<Vec<&str>>();
            let numer = FeltParts::from_str(parts[0])?;
            let denom = FeltParts::from_str(parts[1])?;
            Ok(FeltFraction { numer, denom })
        } else if s.contains('.') {
            log!("contains .");
            log!(
                "BigRational: {:?}",
                BigRational::from_float(f64::from_str(s).unwrap())
            );

            let value = f64::from_str(s).unwrap();

            if !value.is_finite() {
                return Err(());
            }

            let (mantissa, exponent, sign) = integer_decode(value);

            let mut numerator = mantissa as u128;
            if sign == -1 {
                numerator = !numerator + 1;
            }

            let (numerator, denominator) = if exponent < 0 {
                let denominator = 1u128 << (-exponent as usize);
                (numerator, denominator)
            } else {
                (numerator << exponent as usize, 1)
            };

            let numer = FeltParts::from_biguint(BigUint::from(numerator));
            let denom = FeltParts::from_biguint(BigUint::from(denominator));

            Ok(FeltFraction { numer, denom })
        } else {
            log!("does not contain / or .");
            let num = FeltParts::from_str(s)?;
            Ok(FeltFraction {
                numer: num,
                denom: FeltParts::one(),
            })
        }
    }
}

impl FromStr for FeltParts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FeltParts::from_felt(Felt::from_str(s).unwrap()))
    }
}

impl FeltFraction {
    pub fn new(numer: FeltParts, denom: FeltParts) -> Self {
        Self { numer, denom }
    }

    pub fn zero() -> Self {
        FeltFraction {
            numer: FeltParts::zero(),
            denom: FeltParts::one(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.numer.low == Felt::ZERO && self.numer.high == Felt::ZERO
    }

    pub fn one() -> Self {
        FeltFraction {
            numer: FeltParts::one(),
            denom: FeltParts::one(),
        }
    }

    pub fn reciprocal(&self) -> Self {
        FeltFraction {
            numer: self.denom.clone(),
            denom: self.numer.clone(),
        }
    }
}

impl FeltParts {
    pub fn zero() -> Self {
        FeltParts {
            low: Felt::ZERO,
            high: Felt::ZERO,
        }
    }

    pub fn one() -> Self {
        FeltParts {
            low: Felt::ONE,
            high: Felt::ZERO,
        }
    }

    pub fn from_biguint(big_uint: BigUint) -> Self {
        let mask = (BigUint::from(1u128) << 128u32) - BigUint::from(1u128);

        let low = Felt::from((big_uint.clone() >> 0) & mask.clone());
        let high = Felt::from((big_uint.clone() >> 128) & mask.clone());
        FeltParts { low, high }
    }

    pub fn from_felt(felt: Felt) -> Self {
        let big_uint = felt.to_biguint();
        FeltParts::from_biguint(big_uint)
    }
}
