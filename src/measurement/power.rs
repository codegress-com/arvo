use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of power.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerUnit {
    W,
    KW,
    MW,
    Hp,
}

#[cfg(feature = "serde")]
impl From<Power> for String {
    fn from(v: Power) -> String {
        v.canonical
    }
}

impl TryFrom<String> for Power {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for PowerUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerUnit::W => write!(f, "W"),
            PowerUnit::KW => write!(f, "kW"),
            PowerUnit::MW => write!(f, "MW"),
            PowerUnit::Hp => write!(f, "hp"),
        }
    }
}

/// Input for [`Power`].
#[derive(Debug, Clone, PartialEq)]
pub struct PowerInput {
    pub value: f64,
    pub unit: PowerUnit,
}

/// A validated power measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Power, PowerInput, PowerUnit};
/// use arvo::traits::ValueObject;
///
/// let p = Power::new(PowerInput { value: 3.7, unit: PowerUnit::KW })?;
/// assert_eq!(p.value(), "3.7 kW");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Power {
    value: f64,
    unit: PowerUnit,
    canonical: String,
}

impl ValueObject for Power {
    type Input = PowerInput;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Power", &input.value.to_string()));
        }
        let canonical = format!("{} {}", input.value, input.unit);
        Ok(Self {
            value: input.value,
            unit: input.unit,
            canonical,
        })
    }

    fn into_inner(self) -> Self::Input {
        PowerInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Power {
    pub fn value(&self) -> &str {
        &self.canonical
    }

    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &PowerUnit {
        &self.unit
    }
}

impl TryFrom<&str> for Power {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Power", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "W" => PowerUnit::W,
            "kW" => PowerUnit::KW,
            "MW" => PowerUnit::MW,
            "hp" => PowerUnit::Hp,
            _ => return Err(err()),
        };
        Self::new(PowerInput { value: val, unit })
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let p = Power::new(PowerInput {
            value: 3.7,
            unit: PowerUnit::KW,
        })
        .unwrap();
        assert_eq!(p.value(), "3.7 kW");
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Power::new(PowerInput {
                value: 0.0,
                unit: PowerUnit::W
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Power::new(PowerInput {
                value: -1.0,
                unit: PowerUnit::W
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Power::new(PowerInput {
                value: f64::NAN,
                unit: PowerUnit::W
            })
            .is_err()
        );
    }

    #[test]
    fn try_from_parses_valid() {
        let p = Power::try_from("3.7 kW").unwrap();
        assert_eq!(p.value(), "3.7 kW");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Power::try_from("3.7").is_err());
    }

    #[test]
    fn try_from_rejects_unknown_unit() {
        assert!(Power::try_from("3.7 CVs").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Power::try_from("3.7 kW").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Power = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = Power::try_from("3.7 kW").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("3.7 kW"));
    }
}
