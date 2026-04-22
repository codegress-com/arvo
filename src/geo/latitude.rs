use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Latitude`].
pub type LatitudeInput = f64;

/// Output type for [`Latitude`].
pub type LatitudeOutput = f64;

/// A validated geographic latitude in decimal degrees.
///
/// The value must be finite and in the inclusive range `−90.0..=90.0`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::geo::Latitude;
/// use arvo::traits::ValueObject;
///
/// let lat = Latitude::new(48.8588).unwrap();
/// assert_eq!(*lat.value(), 48.8588);
///
/// assert!(Latitude::new(91.0).is_err());
/// assert!(Latitude::new(f64::NAN).is_err());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Latitude(f64);

impl ValueObject for Latitude {
    type Input = LatitudeInput;
    type Output = LatitudeOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(ValidationError::invalid("Latitude", &value.to_string()));
        }
        if !(-90.0..=90.0).contains(&value) {
            return Err(ValidationError::invalid("Latitude", &value.to_string()));
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

impl TryFrom<&str> for Latitude {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<f64>().map_err(|_| ValidationError::invalid("Latitude", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.6}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_latitude() {
        let lat = Latitude::new(48.8588).unwrap();
        assert_eq!(*lat.value(), 48.8588);
    }

    #[test]
    fn accepts_boundaries() {
        assert!(Latitude::new(-90.0).is_ok());
        assert!(Latitude::new(90.0).is_ok());
    }

    #[test]
    fn rejects_out_of_range() {
        assert!(Latitude::new(90.001).is_err());
        assert!(Latitude::new(-90.001).is_err());
    }

    #[test]
    fn rejects_nan() {
        assert!(Latitude::new(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Latitude::new(f64::INFINITY).is_err());
        assert!(Latitude::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn display_six_decimal_places() {
        let lat = Latitude::new(48.858844).unwrap();
        assert_eq!(lat.to_string(), "48.858844");
    }

    #[test]
    fn into_inner_roundtrip() {
        let lat = Latitude::new(51.5074).unwrap();
        assert_eq!(lat.into_inner(), 51.5074);
    }

    #[test]
    fn try_from_parses_valid() {
        let lat = Latitude::try_from("48.8588").unwrap();
        assert_eq!(*lat.value(), 48.8588);
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(Latitude::try_from("not_a_number").is_err());
    }

    #[test]
    fn try_from_rejects_out_of_range() {
        assert!(Latitude::try_from("91.0").is_err());
    }
}
