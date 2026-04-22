use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`NonNegativeInt`].
pub type NonNegativeIntInput = i64;

/// A non-negative integer (`i64 >= 0`).
///
/// Negative values are rejected on construction. Zero is allowed.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::NonNegativeInt;
/// use arvo::traits::ValueObject;
///
/// let n = NonNegativeInt::new(0).unwrap();
/// assert_eq!(*n.value(), 0);
///
/// assert!(NonNegativeInt::new(-1).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "i64", into = "i64"))]
pub struct NonNegativeInt(i64);

impl ValueObject for NonNegativeInt {
    type Input = NonNegativeIntInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(ValidationError::OutOfRange {
                type_name: "NonNegativeInt",
                min: "0".into(),
                max: i64::MAX.to_string(),
                actual: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for NonNegativeInt {
    type Primitive = i64;
    fn value(&self) -> &i64 {
        &self.0
    }
}

impl TryFrom<i64> for NonNegativeInt {
    type Error = ValidationError;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<NonNegativeInt> for i64 {
    fn from(v: NonNegativeInt) -> i64 {
        v.0
    }
}
impl TryFrom<&str> for NonNegativeInt {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value
            .trim()
            .parse::<i64>()
            .map_err(|_| ValidationError::invalid("NonNegativeInt", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for NonNegativeInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_zero() {
        let n = NonNegativeInt::new(0).unwrap();
        assert_eq!(*n.value(), 0);
    }

    #[test]
    fn accepts_positive_value() {
        let n = NonNegativeInt::new(100).unwrap();
        assert_eq!(*n.value(), 100);
    }

    #[test]
    fn rejects_negative() {
        assert!(NonNegativeInt::new(-1).is_err());
    }

    #[test]
    fn try_from_parses_valid() {
        let v = NonNegativeInt::try_from("0").unwrap();
        assert_eq!(*v.value(), 0);
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(NonNegativeInt::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_negative() {
        assert!(NonNegativeInt::try_from("-1").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = NonNegativeInt::new(0).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "0");
        let back: NonNegativeInt = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<NonNegativeInt, _> = serde_json::from_str("-1");
        assert!(result.is_err());
    }
}
