use crate::errors::ValidationError;
use crate::traits::ValueObject;

use super::Coordinate;

/// Input for [`BoundingBox`].
#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBoxInput {
    /// South-west corner (minimum lat/lng).
    pub sw: Coordinate,
    /// North-east corner (maximum lat/lng).
    pub ne: Coordinate,
}

/// A geographic bounding box defined by a south-west and a north-east [`Coordinate`].
///
/// **Validation:** `sw.lat ≤ ne.lat` and `sw.lng ≤ ne.lng`.
///
/// The canonical string is `"SW: <sw> / NE: <ne>"`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::geo::{BoundingBox, BoundingBoxInput, Coordinate, CoordinateInput, Latitude, Longitude};
/// use arvo::traits::ValueObject;
///
/// let sw = Coordinate::new(CoordinateInput {
///     lat: Latitude::new(48.0)?,
///     lng: Longitude::new(14.0)?,
/// })?;
/// let ne = Coordinate::new(CoordinateInput {
///     lat: Latitude::new(51.0)?,
///     lng: Longitude::new(18.0)?,
/// })?;
///
/// let bbox = BoundingBox::new(BoundingBoxInput { sw, ne })?;
/// assert!(bbox.value().starts_with("SW:"));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingBox {
    sw: Coordinate,
    ne: Coordinate,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for BoundingBox {
    type Input = BoundingBoxInput;
    type Output = str;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let sw_lat = value.sw.lat().value();
        let sw_lng = value.sw.lng().value();
        let ne_lat = value.ne.lat().value();
        let ne_lng = value.ne.lng().value();

        if sw_lat > ne_lat || sw_lng > ne_lng {
            return Err(ValidationError::invalid(
                "BoundingBox",
                "sw must be south-west of ne (lat and lng must be ≤ ne)",
            ));
        }

        let canonical = format!("SW: {} / NE: {}", value.sw, value.ne);
        Ok(Self {
            sw: value.sw,
            ne: value.ne,
            canonical,
        })
    }

    fn value(&self) -> &Self::Output {
        &self.canonical
    }

    fn into_inner(self) -> Self::Input {
        BoundingBoxInput {
            sw: self.sw,
            ne: self.ne,
        }
    }
}

impl BoundingBox {
    /// Returns the south-west corner.
    pub fn sw(&self) -> &Coordinate {
        &self.sw
    }

    /// Returns the north-east corner.
    pub fn ne(&self) -> &Coordinate {
        &self.ne
    }

    /// Returns `true` if `coord` lies within this bounding box (inclusive on all edges).
    pub fn contains(&self, coord: &Coordinate) -> bool {
        let lat = coord.lat().value();
        let lng = coord.lng().value();
        lat >= self.sw.lat().value()
            && lat <= self.ne.lat().value()
            && lng >= self.sw.lng().value()
            && lng <= self.ne.lng().value()
    }
}

impl TryFrom<BoundingBoxInput> for BoundingBox {
    type Error = ValidationError;

    fn try_from(value: BoundingBoxInput) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::{CoordinateInput, Latitude, Longitude};

    fn coord(lat: f64, lng: f64) -> Coordinate {
        Coordinate::new(CoordinateInput {
            lat: Latitude::new(lat).unwrap(),
            lng: Longitude::new(lng).unwrap(),
        })
        .unwrap()
    }

    #[test]
    fn accepts_valid_bbox() {
        let bbox = BoundingBox::new(BoundingBoxInput {
            sw: coord(48.0, 14.0),
            ne: coord(51.0, 18.0),
        })
        .unwrap();
        assert!(bbox.value().starts_with("SW:"));
        assert!(bbox.value().contains("NE:"));
    }

    #[test]
    fn rejects_sw_north_of_ne() {
        assert!(
            BoundingBox::new(BoundingBoxInput {
                sw: coord(52.0, 14.0),
                ne: coord(51.0, 18.0),
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_sw_east_of_ne() {
        assert!(
            BoundingBox::new(BoundingBoxInput {
                sw: coord(48.0, 19.0),
                ne: coord(51.0, 18.0),
            })
            .is_err()
        );
    }

    #[test]
    fn accepts_equal_corners() {
        assert!(
            BoundingBox::new(BoundingBoxInput {
                sw: coord(50.0, 14.0),
                ne: coord(50.0, 14.0),
            })
            .is_ok()
        );
    }

    #[test]
    fn accessors() {
        let bbox = BoundingBox::new(BoundingBoxInput {
            sw: coord(48.0, 14.0),
            ne: coord(51.0, 18.0),
        })
        .unwrap();
        assert_eq!(*bbox.sw().lat().value(), 48.0);
        assert_eq!(*bbox.ne().lng().value(), 18.0);
    }

    #[test]
    fn contains_inside() {
        let bbox = BoundingBox::new(BoundingBoxInput {
            sw: coord(48.0, 14.0),
            ne: coord(51.0, 18.0),
        })
        .unwrap();
        assert!(bbox.contains(&coord(50.0, 16.0)));
    }

    #[test]
    fn contains_on_edge() {
        let bbox = BoundingBox::new(BoundingBoxInput {
            sw: coord(48.0, 14.0),
            ne: coord(51.0, 18.0),
        })
        .unwrap();
        assert!(bbox.contains(&coord(48.0, 14.0)));
        assert!(bbox.contains(&coord(51.0, 18.0)));
    }

    #[test]
    fn contains_outside() {
        let bbox = BoundingBox::new(BoundingBoxInput {
            sw: coord(48.0, 14.0),
            ne: coord(51.0, 18.0),
        })
        .unwrap();
        assert!(!bbox.contains(&coord(52.0, 16.0)));
    }

    #[test]
    fn display_matches_value() {
        let bbox = BoundingBox::new(BoundingBoxInput {
            sw: coord(48.0, 14.0),
            ne: coord(51.0, 18.0),
        })
        .unwrap();
        assert_eq!(bbox.to_string(), bbox.value());
    }
}
