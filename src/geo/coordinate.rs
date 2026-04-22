use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

use super::{Latitude, Longitude};

/// Input for [`Coordinate`].
#[derive(Debug, Clone, PartialEq)]
pub struct CoordinateInput {
    pub lat: Latitude,
    pub lng: Longitude,
}

/// A geographic coordinate (latitude + longitude pair).
///
/// Constructed from a validated [`Latitude`] and [`Longitude`]. The canonical
/// string representation is `"lat, lng"` with six decimal places each.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::geo::{Coordinate, CoordinateInput, Latitude, Longitude};
/// use arvo::traits::ValueObject;
///
/// let coord = Coordinate::new(CoordinateInput {
///     lat: Latitude::new(48.858844)?,
///     lng: Longitude::new(2.294351)?,
/// })?;
///
/// assert_eq!(coord.value(), "48.858844, 2.294351");
/// assert_eq!(*coord.lat().value(), 48.858844);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Coordinate {
    lat: Latitude,
    lng: Longitude,
    canonical: String,
}

impl ValueObject for Coordinate {
    type Input = CoordinateInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let canonical = format!("{:.6}, {:.6}", value.lat.value(), value.lng.value());
        Ok(Self {
            lat: value.lat,
            lng: value.lng,
            canonical,
        })
    }

    fn into_inner(self) -> Self::Input {
        CoordinateInput {
            lat: self.lat,
            lng: self.lng,
        }
    }
}

impl Coordinate {
    pub fn value(&self) -> &str {
        &self.canonical
    }

    /// Returns the latitude component.
    pub fn lat(&self) -> &Latitude {
        &self.lat
    }

    /// Returns the longitude component.
    pub fn lng(&self) -> &Longitude {
        &self.lng
    }
}

impl TryFrom<&str> for Coordinate {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Coordinate", value);
        let (lat_str, lng_str) = value.trim().split_once(", ").ok_or_else(err)?;
        let lat =
            Latitude::new(lat_str.trim().parse::<f64>().map_err(|_| err())?).map_err(|_| err())?;
        let lng =
            Longitude::new(lng_str.trim().parse::<f64>().map_err(|_| err())?).map_err(|_| err())?;
        Self::new(CoordinateInput { lat, lng })
    }
}

#[cfg(feature = "serde")]
impl From<Coordinate> for String {
    fn from(v: Coordinate) -> String {
        v.canonical
    }
}

impl TryFrom<String> for Coordinate {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(lat: f64, lng: f64) -> Coordinate {
        Coordinate::new(CoordinateInput {
            lat: Latitude::new(lat).unwrap(),
            lng: Longitude::new(lng).unwrap(),
        })
        .unwrap()
    }

    #[test]
    fn canonical_format() {
        let c = make(48.858844, 2.294351);
        assert_eq!(c.value(), "48.858844, 2.294351");
    }

    #[test]
    fn accessors() {
        let c = make(51.5074, -0.1278);
        assert_eq!(*c.lat().value(), 51.5074);
        assert_eq!(*c.lng().value(), -0.1278);
    }

    #[test]
    fn display_matches_value() {
        let c = make(0.0, 0.0);
        assert_eq!(c.to_string(), c.value());
    }

    #[test]
    fn into_inner_roundtrip() {
        let c = make(48.858844, 2.294351);
        let inner = c.clone().into_inner();
        assert_eq!(*inner.lat.value(), 48.858844);
        assert_eq!(*inner.lng.value(), 2.294351);
    }

    #[test]
    fn try_from_parses_valid() {
        let c = Coordinate::try_from("48.858844, 2.294351").unwrap();
        assert_eq!(c.value(), "48.858844, 2.294351");
    }

    #[test]
    fn try_from_rejects_no_comma_separator() {
        assert!(Coordinate::try_from("48.858844 2.294351").is_err());
    }

    #[test]
    fn try_from_rejects_invalid_lat() {
        assert!(Coordinate::try_from("91.0, 0.0").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Coordinate::try_from("48.858844, 2.294351").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Coordinate = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = Coordinate::try_from("48.858844, 2.294351").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("48.858844, 2.294351"));
    }
}
