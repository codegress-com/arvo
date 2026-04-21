use crate::errors::ValidationError;
use crate::traits::ValueObject;

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
pub struct Coordinate {
    lat: Latitude,
    lng: Longitude,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Coordinate {
    type Input = CoordinateInput;
    type Output = str;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let canonical = format!("{:.6}, {:.6}", value.lat.value(), value.lng.value());
        Ok(Self {
            lat: value.lat,
            lng: value.lng,
            canonical,
        })
    }

    fn value(&self) -> &Self::Output {
        &self.canonical
    }

    fn into_inner(self) -> Self::Input {
        CoordinateInput {
            lat: self.lat,
            lng: self.lng,
        }
    }
}

impl Coordinate {
    /// Returns the latitude component.
    pub fn lat(&self) -> &Latitude {
        &self.lat
    }

    /// Returns the longitude component.
    pub fn lng(&self) -> &Longitude {
        &self.lng
    }
}

impl TryFrom<CoordinateInput> for Coordinate {
    type Error = ValidationError;

    fn try_from(value: CoordinateInput) -> Result<Self, Self::Error> {
        Self::new(value)
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
}
