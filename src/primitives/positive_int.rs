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
#[cfg_attr(feature = "serde", serde(transparent))]
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
}
