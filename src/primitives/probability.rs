use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Probability`].
pub type ProbabilityInput = f64;

/// Output type for [`Probability`].
pub type ProbabilityOutput = f64;

/// A probability value in the range `0.0..=1.0`.
///
/// NaN, infinite values, and values outside `[0.0, 1.0]` are rejected.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::Probability;
/// use arvo::traits::ValueObject;
///
/// let p = Probability::new(0.75).unwrap();
/// assert_eq!(*p.value(), 0.75);
///
/// assert!(Probability::new(1.5).is_err());
/// assert!(Probability::new(f64::NAN).is_err());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Probability(f64);

impl ValueObject for Probability {
    type Input = ProbabilityInput;
    type Output = ProbabilityOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ValidationError::OutOfRange {
                type_name: "Probability",
                min: "0.0".into(),
                max: "1.0".into(),
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

impl TryFrom<&str> for Probability {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<f64>().map_err(|_| ValidationError::invalid("Probability", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_zero() {
        let p = Probability::new(0.0).unwrap();
        assert_eq!(*p.value(), 0.0);
    }

    #[test]
    fn accepts_one() {
        let p = Probability::new(1.0).unwrap();
        assert_eq!(*p.value(), 1.0);
    }

    #[test]
    fn accepts_midpoint() {
        let p = Probability::new(0.5).unwrap();
        assert_eq!(*p.value(), 0.5);
    }

    #[test]
    fn rejects_above_one() {
        assert!(Probability::new(1.001).is_err());
    }

    #[test]
    fn rejects_below_zero() {
        assert!(Probability::new(-0.001).is_err());
    }

    #[test]
    fn rejects_nan() {
        assert!(Probability::new(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Probability::new(f64::INFINITY).is_err());
    }

    #[test]
    fn try_from_parses_valid() {
        let p = Probability::try_from("0.5").unwrap();
        assert_eq!(*p.value(), 0.5);
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(Probability::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_out_of_range() {
        assert!(Probability::try_from("1.1").is_err());
    }
}
