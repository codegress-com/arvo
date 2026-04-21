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
pub struct Speed {
    value: f64,
    unit: SpeedUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Speed {
    type Input = SpeedInput;
    type Output = str;
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

    fn value(&self) -> &Self::Output {
        &self.canonical
    }
    fn into_inner(self) -> Self::Input {
        SpeedInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Speed {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &SpeedUnit {
        &self.unit
    }
}

impl TryFrom<SpeedInput> for Speed {
    type Error = ValidationError;

    fn try_from(value: SpeedInput) -> Result<Self, Self::Error> {
        Self::new(value)
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
}
