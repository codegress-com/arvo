use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`Probability`].
pub type ProbabilityInput = f64;

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
#[cfg_attr(feature = "serde", serde(try_from = "f64", into = "f64"))]
pub struct Probability(f64);

impl ValueObject for Probability {
    type Input = ProbabilityInput;
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

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for Probability {
    type Primitive = f64;
    fn value(&self) -> &f64 {
        &self.0
    }
}

impl TryFrom<f64> for Probability {
    type Error = ValidationError;
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<Probability> for f64 {
    fn from(v: Probability) -> f64 {
        v.0
    }
}
impl TryFrom<&str> for Probability {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(|_| ValidationError::invalid("Probability", value))?;
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

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Probability::new(0.5).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "0.5");
        let back: Probability = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Probability, _> = serde_json::from_str("1.1");
        assert!(result.is_err());
    }
}
