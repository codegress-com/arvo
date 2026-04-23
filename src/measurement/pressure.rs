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

#[cfg(feature = "serde")]
impl From<Pressure> for String {
    fn from(v: Pressure) -> String {
        v.canonical
    }
}

impl TryFrom<String> for Pressure {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
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
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Pressure {
    value: f64,
    unit: PressureUnit,
    canonical: String,
}

impl ValueObject for Pressure {
    type Input = PressureInput;
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

    fn into_inner(self) -> Self::Input {
        PressureInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Pressure {
    pub fn value(&self) -> &str {
        &self.canonical
    }

    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &PressureUnit {
        &self.unit
    }
}

impl TryFrom<&str> for Pressure {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Pressure", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "Pa" => PressureUnit::Pa,
            "kPa" => PressureUnit::KPa,
            "MPa" => PressureUnit::MPa,
            "bar" => PressureUnit::Bar,
            "psi" => PressureUnit::Psi,
            "atm" => PressureUnit::Atm,
            _ => return Err(err()),
        };
        Self::new(PressureInput { value: val, unit })
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

    #[test]
    fn try_from_parses_valid() {
        let p = Pressure::try_from("101.325 kPa").unwrap();
        assert_eq!(p.value(), "101.325 kPa");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Pressure::try_from("101").is_err());
    }

    #[test]
    fn try_from_rejects_unknown_unit() {
        assert!(Pressure::try_from("1.0 hg").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Pressure::try_from("101.325 kPa").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Pressure = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = Pressure::try_from("101.325 kPa").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("101.325 kPa"));
    }
}
