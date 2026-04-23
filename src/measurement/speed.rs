use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of speed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpeedUnit {
    Ms,
    Kmh,
    Mph,
    Kn,
}

#[cfg(feature = "serde")]
impl From<Speed> for String {
    fn from(v: Speed) -> String {
        v.canonical
    }
}

impl TryFrom<String> for Speed {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for SpeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpeedUnit::Ms => write!(f, "m/s"),
            SpeedUnit::Kmh => write!(f, "km/h"),
            SpeedUnit::Mph => write!(f, "mph"),
            SpeedUnit::Kn => write!(f, "kn"),
        }
    }
}

/// Input for [`Speed`].
#[derive(Debug, Clone, PartialEq)]
pub struct SpeedInput {
    pub value: f64,
    pub unit: SpeedUnit,
}

/// A validated speed measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Speed, SpeedInput, SpeedUnit};
/// use arvo::traits::ValueObject;
///
/// let s = Speed::new(SpeedInput { value: 120.0, unit: SpeedUnit::Kmh })?;
/// assert_eq!(s.value(), "120 km/h");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Speed {
    value: f64,
    unit: SpeedUnit,
    canonical: String,
}

impl ValueObject for Speed {
    type Input = SpeedInput;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Speed", &input.value.to_string()));
        }
        let canonical = format!("{} {}", input.value, input.unit);
        Ok(Self {
            value: input.value,
            unit: input.unit,
            canonical,
        })
    }

    fn into_inner(self) -> Self::Input {
        SpeedInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Speed {
    pub fn value(&self) -> &str {
        &self.canonical
    }

    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &SpeedUnit {
        &self.unit
    }
}

impl TryFrom<&str> for Speed {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Speed", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "m/s" => SpeedUnit::Ms,
            "km/h" => SpeedUnit::Kmh,
            "mph" => SpeedUnit::Mph,
            "kn" => SpeedUnit::Kn,
            _ => return Err(err()),
        };
        Self::new(SpeedInput { value: val, unit })
    }
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let s = Speed::new(SpeedInput {
            value: 120.0,
            unit: SpeedUnit::Kmh,
        })
        .unwrap();
        assert_eq!(s.value(), "120 km/h");
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Speed::new(SpeedInput {
                value: 0.0,
                unit: SpeedUnit::Ms
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Speed::new(SpeedInput {
                value: -1.0,
                unit: SpeedUnit::Ms
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Speed::new(SpeedInput {
                value: f64::NAN,
                unit: SpeedUnit::Ms
            })
            .is_err()
        );
    }

    #[test]
    fn try_from_parses_valid() {
        let s = Speed::try_from("120 km/h").unwrap();
        assert_eq!(s.value(), "120 km/h");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Speed::try_from("120").is_err());
    }

    #[test]
    fn try_from_rejects_unknown_unit() {
        assert!(Speed::try_from("120 warp").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Speed::try_from("120 km/h").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Speed = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = Speed::try_from("120 km/h").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("120 km/h"));
    }
}
