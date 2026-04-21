use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of pressure.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PressureUnit {
    Pa,
    KPa,
    MPa,
    Bar,
    Psi,
    Atm,
}

impl std::fmt::Display for PressureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PressureUnit::Pa => write!(f, "Pa"),
            PressureUnit::KPa => write!(f, "kPa"),
            PressureUnit::MPa => write!(f, "MPa"),
            PressureUnit::Bar => write!(f, "bar"),
            PressureUnit::Psi => write!(f, "psi"),
            PressureUnit::Atm => write!(f, "atm"),
        }
    }
}

/// Input for [`Pressure`].
#[derive(Debug, Clone, PartialEq)]
pub struct PressureInput {
    pub value: f64,
    pub unit: PressureUnit,
}

/// A validated pressure measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Pressure, PressureInput, PressureUnit};
/// use arvo::traits::ValueObject;
///
/// let p = Pressure::new(PressureInput { value: 101.325, unit: PressureUnit::KPa })?;
/// assert_eq!(p.value(), "101.325 kPa");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pressure {
    value: f64,
    unit: PressureUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Pressure {
    type Input = PressureInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid(
                "Pressure",
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
        PressureInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Pressure {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &PressureUnit {
        &self.unit
    }
}

impl TryFrom<PressureInput> for Pressure {
    type Error = ValidationError;

    fn try_from(value: PressureInput) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for Pressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let p = Pressure::new(PressureInput {
            value: 101.325,
            unit: PressureUnit::KPa,
        })
        .unwrap();
        assert_eq!(p.value(), "101.325 kPa");
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Pressure::new(PressureInput {
                value: 0.0,
                unit: PressureUnit::Pa
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Pressure::new(PressureInput {
                value: -1.0,
                unit: PressureUnit::Pa
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Pressure::new(PressureInput {
                value: f64::NAN,
                unit: PressureUnit::Pa
            })
            .is_err()
        );
    }
}
