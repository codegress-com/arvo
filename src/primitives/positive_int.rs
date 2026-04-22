use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`PositiveInt`].
pub type PositiveIntInput = i64;

/// Output type for [`PositiveInt`].
pub type PositiveIntOutput = i64;

/// A strictly positive integer (`i64 > 0`).
///
/// Zero and negative values are rejected on construction.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::PositiveInt;
/// use arvo::traits::ValueObject;
///
/// let n = PositiveInt::new(42).unwrap();
/// assert_eq!(*n.value(), 42);
///
/// assert!(PositiveInt::new(0).is_err());
/// assert!(PositiveInt::new(-1).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "i64", into = "i64"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct PositiveInt(i64);

impl ValueObject for PositiveInt {
    type Input = PositiveIntInput;
    type Output = PositiveIntOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value <= 0 {
            return Err(ValidationError::OutOfRange {
                type_name: "PositiveInt",
                min: "1".into(),
                max: i64::MAX.to_string(),
                actual: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}


impl TryFrom<i64> for PositiveInt {
    type Error = ValidationError;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<PositiveInt> for i64 {
    fn from(v: PositiveInt) -> i64 {
        v.0
    }
}
impl TryFrom<&str> for PositiveInt {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<i64>().map_err(|_| ValidationError::invalid("PositiveInt", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for PositiveInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_positive_value() {
        let n = PositiveInt::new(1).unwrap();
        assert_eq!(*n.value(), 1);
    }

    #[test]
    fn accepts_large_value() {
        let n = PositiveInt::new(i64::MAX).unwrap();
        assert_eq!(*n.value(), i64::MAX);
    }

    #[test]
    fn rejects_zero() {
        assert!(PositiveInt::new(0).is_err());
    }

    #[test]
    fn rejects_negative() {
        assert!(PositiveInt::new(-1).is_err());
    }

    #[test]
    fn try_from_parses_valid() {
        let v = PositiveInt::try_from("42").unwrap();
        assert_eq!(*v.value(), 42);
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(PositiveInt::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_zero() {
        assert!(PositiveInt::try_from("0").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = PositiveInt::new(42).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "42");
        let back: PositiveInt = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<PositiveInt, _> = serde_json::from_str("0");
        assert!(result.is_err());
    }
}
