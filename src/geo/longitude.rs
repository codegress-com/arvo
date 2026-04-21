use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Longitude`].
pub type LongitudeInput = f64;

/// Output type for [`Longitude`].
pub type LongitudeOutput = f64;

/// A validated geographic longitude in decimal degrees.
///
/// The value must be finite and in the inclusive range `−180.0..=180.0`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::geo::Longitude;
/// use arvo::traits::ValueObject;
///
/// let lng = Longitude::new(14.4208).unwrap();
/// assert_eq!(*lng.value(), 14.4208);
///
/// assert!(Longitude::new(181.0).is_err());
/// assert!(Longitude::new(f64::NAN).is_err());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Longitude(f64);

impl ValueObject for Longitude {
    type Input = LongitudeInput;
    type Output = LongitudeOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(ValidationError::invalid("Longitude", &value.to_string()));
        }
        if !(-180.0..=180.0).contains(&value) {
            return Err(ValidationError::invalid("Longitude", &value.to_string()));
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

impl TryFrom<&str> for Longitude {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value.trim().parse::<f64>().map_err(|_| ValidationError::invalid("Longitude", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.6}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_longitude() {
        let lng = Longitude::new(14.4208).unwrap();
        assert_eq!(*lng.value(), 14.4208);
    }

    #[test]
    fn accepts_boundaries() {
        assert!(Longitude::new(-180.0).is_ok());
        assert!(Longitude::new(180.0).is_ok());
    }

    #[test]
    fn rejects_out_of_range() {
        assert!(Longitude::new(180.001).is_err());
        assert!(Longitude::new(-180.001).is_err());
    }

    #[test]
    fn rejects_nan() {
        assert!(Longitude::new(f64::NAN).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Longitude::new(f64::INFINITY).is_err());
        assert!(Longitude::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn display_six_decimal_places() {
        let lng = Longitude::new(14.420800).unwrap();
        assert_eq!(lng.to_string(), "14.420800");
    }

    #[test]
    fn into_inner_roundtrip() {
        let lng = Longitude::new(-0.1278).unwrap();
        assert_eq!(lng.into_inner(), -0.1278);
    }
}
