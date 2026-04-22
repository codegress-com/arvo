use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`NonNegativeInt`].
pub type NonNegativeIntInput = i64;

/// Output type for [`NonNegativeInt`].
pub type NonNegativeIntOutput = i64;

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
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct NonNegativeInt(i64);

impl ValueObject for NonNegativeInt {
    type Input = NonNegativeIntInput;
    type Output = NonNegativeIntOutput;
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

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl TryFrom<&str> for NonNegativeInt {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<i64>().map_err(|_| ValidationError::invalid("NonNegativeInt", value))?;
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
}
