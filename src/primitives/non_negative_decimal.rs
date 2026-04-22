use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};
use rust_decimal::Decimal;

/// Input type for [`NonNegativeDecimal`].
pub type NonNegativeDecimalInput = Decimal;

/// A non-negative decimal number (`Decimal >= 0`).
///
/// Negative values are rejected on construction. Zero is allowed.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::NonNegativeDecimal;
/// use arvo::traits::ValueObject;
/// use rust_decimal_macros::dec;
///
/// let amount = NonNegativeDecimal::new(dec!(0)).unwrap();
/// assert_eq!(*amount.value(), dec!(0));
///
/// assert!(NonNegativeDecimal::new(dec!(-0.01)).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Decimal", into = "Decimal"))]
pub struct NonNegativeDecimal(Decimal);

impl ValueObject for NonNegativeDecimal {
    type Input = NonNegativeDecimalInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value < Decimal::ZERO {
            return Err(ValidationError::OutOfRange {
                type_name: "NonNegativeDecimal",
                min: "0".into(),
                max: "∞".into(),
                actual: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for NonNegativeDecimal {
    type Primitive = Decimal;
    fn value(&self) -> &Decimal {
        &self.0
    }
}

impl TryFrom<Decimal> for NonNegativeDecimal {
    type Error = ValidationError;
    fn try_from(v: Decimal) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<NonNegativeDecimal> for Decimal {
    fn from(v: NonNegativeDecimal) -> Decimal {
        v.0
    }
}
impl TryFrom<&str> for NonNegativeDecimal {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value
            .trim()
            .parse::<Decimal>()
            .map_err(|_| ValidationError::invalid("NonNegativeDecimal", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for NonNegativeDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromStr;

    #[test]
    fn accepts_zero() {
        let d = NonNegativeDecimal::new(Decimal::ZERO).unwrap();
        assert_eq!(d.value(), &Decimal::ZERO);
    }

    #[test]
    fn accepts_positive_value() {
        let d = NonNegativeDecimal::new(Decimal::from_str("1.5").unwrap()).unwrap();
        assert_eq!(d.value(), &Decimal::from_str("1.5").unwrap());
    }

    #[test]
    fn rejects_negative() {
        assert!(NonNegativeDecimal::new(Decimal::from_str("-0.01").unwrap()).is_err());
    }

    #[test]
    fn try_from_parses_valid() {
        let v = NonNegativeDecimal::try_from("0.00").unwrap();
        assert_eq!(v.value().to_string(), "0.00");
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(NonNegativeDecimal::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_negative() {
        assert!(NonNegativeDecimal::try_from("-1").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = NonNegativeDecimal::try_from("0.00").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: NonNegativeDecimal = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<NonNegativeDecimal, _> = serde_json::from_str("\"-1\"");
        assert!(result.is_err());
    }
}
