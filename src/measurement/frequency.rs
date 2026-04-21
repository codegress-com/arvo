use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of frequency.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FrequencyUnit {
    Hz,
    KHz,
    MHz,
    GHz,
}

impl std::fmt::Display for FrequencyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyUnit::Hz => write!(f, "Hz"),
            FrequencyUnit::KHz => write!(f, "kHz"),
            FrequencyUnit::MHz => write!(f, "MHz"),
            FrequencyUnit::GHz => write!(f, "GHz"),
        }
    }
}

/// Input for [`Frequency`].
#[derive(Debug, Clone, PartialEq)]
pub struct FrequencyInput {
    pub value: f64,
    pub unit: FrequencyUnit,
}

/// A validated frequency measurement.
///
/// **Validation:** value must be finite and strictly positive (> 0).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Frequency, FrequencyInput, FrequencyUnit};
/// use arvo::traits::ValueObject;
///
/// let f = Frequency::new(FrequencyInput { value: 2.4, unit: FrequencyUnit::GHz })?;
/// assert_eq!(f.value(), "2.4 GHz");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frequency {
    value: f64,
    unit: FrequencyUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Frequency {
    type Input = FrequencyInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value <= 0.0 {
            return Err(ValidationError::invalid(
                "Frequency",
                &input.value.to_string(),
            ));
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
        FrequencyInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Frequency {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &FrequencyUnit {
        &self.unit
    }
}

impl TryFrom<FrequencyInput> for Frequency {
    type Error = ValidationError;

    fn try_from(value: FrequencyInput) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let f = Frequency::new(FrequencyInput {
            value: 2.4,
            unit: FrequencyUnit::GHz,
        })
        .unwrap();
        assert_eq!(f.value(), "2.4 GHz");
    }

    #[test]
    fn rejects_zero() {
        assert!(
            Frequency::new(FrequencyInput {
                value: 0.0,
                unit: FrequencyUnit::Hz
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Frequency::new(FrequencyInput {
                value: -1.0,
                unit: FrequencyUnit::Hz
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Frequency::new(FrequencyInput {
                value: f64::NAN,
                unit: FrequencyUnit::Hz
            })
            .is_err()
        );
    }
}
