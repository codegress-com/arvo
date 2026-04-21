use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of volume.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolumeUnit {
    Ml,
    L,
    M3,
    FlOz,
    Gal,
}

impl std::fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeUnit::Ml => write!(f, "ml"),
            VolumeUnit::L => write!(f, "l"),
            VolumeUnit::M3 => write!(f, "m³"),
            VolumeUnit::FlOz => write!(f, "fl oz"),
            VolumeUnit::Gal => write!(f, "gal"),
        }
    }
}

/// Input for [`Volume`].
#[derive(Debug, Clone, PartialEq)]
pub struct VolumeInput {
    pub value: f64,
    pub unit: VolumeUnit,
}

/// A validated volume measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Volume, VolumeInput, VolumeUnit};
/// use arvo::traits::ValueObject;
///
/// let v = Volume::new(VolumeInput { value: 1.5, unit: VolumeUnit::L })?;
/// assert_eq!(v.value(), "1.5 l");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Volume {
    value: f64,
    unit: VolumeUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Volume {
    type Input = VolumeInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Volume", &input.value.to_string()));
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
        VolumeInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Volume {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &VolumeUnit {
        &self.unit
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let v = Volume::new(VolumeInput {
            value: 1.5,
            unit: VolumeUnit::L,
        })
        .unwrap();
        assert_eq!(v.value(), "1.5 l");
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Volume::new(VolumeInput {
                value: 0.0,
                unit: VolumeUnit::Ml
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Volume::new(VolumeInput {
                value: -1.0,
                unit: VolumeUnit::L
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Volume::new(VolumeInput {
                value: f64::NAN,
                unit: VolumeUnit::L
            })
            .is_err()
        );
    }
}
