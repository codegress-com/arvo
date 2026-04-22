use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};
use rust_decimal::Decimal;

/// Input type for [`PositiveDecimal`].
pub type PositiveDecimalInput = Decimal;

/// Output type for [`PositiveDecimal`].

/// A strictly positive decimal number (`Decimal > 0`).
///
/// Zero and negative values are rejected on construction.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::PositiveDecimal;
/// use arvo::traits::ValueObject;
/// use rust_decimal_macros::dec;
///
/// let price = PositiveDecimal::new(dec!(9.99)).unwrap();
/// assert_eq!(*price.value(), dec!(9.99));
///
/// assert!(PositiveDecimal::new(dec!(0)).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Decimal", into = "Decimal"))]
pub struct PositiveDecimal(Decimal);

impl ValueObject for PositiveDecimal {
    type Input = PositiveDecimalInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value <= Decimal::ZERO {
            return Err(ValidationError::OutOfRange {
                type_name: "PositiveDecimal",
                min: "0 (exclusive)".into(),
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
impl PrimitiveValue for PositiveDecimal {
    type Primitive = Decimal;
    fn value(&self) -> &Decimal {
        &self.0
    }
}

impl TryFrom<Decimal> for PositiveDecimal {
    type Error = ValidationError;
    fn try_from(v: Decimal) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<PositiveDecimal> for Decimal {
    fn from(v: PositiveDecimal) -> Decimal {
        v.0
    }
}
impl TryFrom<&str> for PositiveDecimal {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<Decimal>().map_err(|_| ValidationError::invalid("PositiveDecimal", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for PositiveDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromStr;

    #[test]
    fn accepts_positive_value() {
        let d = PositiveDecimal::new(Decimal::from_str("9.99").unwrap()).unwrap();
        assert_eq!(d.value(), &Decimal::from_str("9.99").unwrap());
    }

    #[test]
    fn rejects_zero() {
        assert!(PositiveDecimal::new(Decimal::ZERO).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(PositiveDecimal::new(Decimal::from_str("-1").unwrap()).is_err());
    }

    #[test]
    fn try_from_parses_valid() {
        let v = PositiveDecimal::try_from("3.14").unwrap();
        assert_eq!(v.value().to_string(), "3.14");
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(PositiveDecimal::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_zero() {
        assert!(PositiveDecimal::try_from("0").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = PositiveDecimal::try_from("3.14").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: PositiveDecimal = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<PositiveDecimal, _> = serde_json::from_str("\"0\"");
        assert!(result.is_err());
    }
}
