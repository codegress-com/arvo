use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Percentage`].
pub type PercentageInput = f64;

/// Output type for [`Percentage`].
pub type PercentageOutput = f64;

/// A validated percentage value in the range `0.0..=100.0`.
///
/// The value must be finite (not NaN, not infinite) and within the inclusive
/// range from `0.0` to `100.0`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::Percentage;
/// use arvo::traits::ValueObject;
///
/// let p = Percentage::new(42.5).unwrap();
/// assert_eq!(*p.value(), 42.5);
///
/// assert!(Percentage::new(-1.0).is_err());
/// assert!(Percentage::new(101.0).is_err());
/// assert!(Percentage::new(f64::NAN).is_err());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "f64", into = "f64"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Percentage(f64);

impl ValueObject for Percentage {
    type Input = PercentageInput;
    type Output = PercentageOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(ValidationError::invalid("Percentage", &value.to_string()));
        }

        if !(0.0..=100.0).contains(&value) {
            return Err(ValidationError::invalid("Percentage", &value.to_string()));
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

impl Percentage {
    /// Returns the value as a fraction in `0.0..=1.0` (e.g. `42.5` → `0.425`).
    pub fn as_fraction(&self) -> f64 {
        self.0 / 100.0
    }
}


impl TryFrom<f64> for Percentage {
    type Error = ValidationError;
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<Percentage> for f64 {
    fn from(v: Percentage) -> f64 {
        v.0
    }
}
impl TryFrom<&str> for Percentage {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<f64>().map_err(|_| ValidationError::invalid("Percentage", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_zero() {
        let p = Percentage::new(0.0).unwrap();
        assert_eq!(*p.value(), 0.0);
    }

    #[test]
    fn accepts_hundred() {
        let p = Percentage::new(100.0).unwrap();
        assert_eq!(*p.value(), 100.0);
    }

    #[test]
    fn accepts_midpoint() {
        let p = Percentage::new(42.5).unwrap();
        assert_eq!(*p.value(), 42.5);
    }

    #[test]
    fn rejects_negative() {
        assert!(Percentage::new(-0.001).is_err());
    }

    #[test]
    fn rejects_above_hundred() {
        assert!(Percentage::new(100.001).is_err());
    }

    #[test]
    fn rejects_nan() {
        assert!(Percentage::new(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Percentage::new(f64::INFINITY).is_err());
        assert!(Percentage::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn display_appends_percent() {
        let p = Percentage::new(50.0).unwrap();
        assert_eq!(p.to_string(), "50%");
    }

    #[test]
    fn into_inner_roundtrip() {
        let p = Percentage::new(33.3).unwrap();
        assert_eq!(p.into_inner(), 33.3);
    }

    #[test]
    fn try_from_parses_valid() {
        let p = Percentage::try_from("42.5").unwrap();
        assert_eq!(*p.value(), 42.5);
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(Percentage::try_from("abc").is_err());
    }

    #[test]
    fn try_from_rejects_out_of_range() {
        assert!(Percentage::try_from("101.0").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Percentage::new(42.5).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "42.5");
        let back: Percentage = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Percentage, _> = serde_json::from_str("101.0");
        assert!(result.is_err());
    }
}
