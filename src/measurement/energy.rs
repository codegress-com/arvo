use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of energy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnergyUnit {
    J,
    KJ,
    MJ,
    KWh,
    Cal,
    Kcal,
}

impl std::fmt::Display for EnergyUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnergyUnit::J => write!(f, "J"),
            EnergyUnit::KJ => write!(f, "kJ"),
            EnergyUnit::MJ => write!(f, "MJ"),
            EnergyUnit::KWh => write!(f, "kWh"),
            EnergyUnit::Cal => write!(f, "cal"),
            EnergyUnit::Kcal => write!(f, "kcal"),
        }
    }
}

/// Input for [`Energy`].
#[derive(Debug, Clone, PartialEq)]
pub struct EnergyInput {
    pub value: f64,
    pub unit: EnergyUnit,
}

/// A validated energy measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Energy, EnergyInput, EnergyUnit};
/// use arvo::traits::ValueObject;
///
/// let e = Energy::new(EnergyInput { value: 500.0, unit: EnergyUnit::Kcal })?;
/// assert_eq!(e.value(), "500 kcal");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Energy {
    value: f64,
    unit: EnergyUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Energy {
    type Input = EnergyInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Energy", &input.value.to_string()));
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
        EnergyInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Energy {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &EnergyUnit {
        &self.unit
    }
}

impl TryFrom<EnergyInput> for Energy {
    type Error = ValidationError;

    fn try_from(value: EnergyInput) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for Energy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let e = Energy::new(EnergyInput {
            value: 500.0,
            unit: EnergyUnit::Kcal,
        })
        .unwrap();
        assert_eq!(e.value(), "500 kcal");
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Energy::new(EnergyInput {
                value: 0.0,
                unit: EnergyUnit::J
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Energy::new(EnergyInput {
                value: -1.0,
                unit: EnergyUnit::J
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Energy::new(EnergyInput {
                value: f64::NAN,
                unit: EnergyUnit::J
            })
            .is_err()
        );
    }
}
