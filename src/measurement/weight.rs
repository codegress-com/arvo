use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of weight/mass.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WeightUnit {
    Mg,
    G,
    Kg,
    T,
    Oz,
    Lb,
}

#[cfg(feature = "serde")]
impl From<Weight> for String {
    fn from(v: Weight) -> String {
        v.canonical
    }
}

impl TryFrom<String> for Weight {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeightUnit::Mg => write!(f, "mg"),
            WeightUnit::G => write!(f, "g"),
            WeightUnit::Kg => write!(f, "kg"),
            WeightUnit::T => write!(f, "t"),
            WeightUnit::Oz => write!(f, "oz"),
            WeightUnit::Lb => write!(f, "lb"),
        }
    }
}

/// Input for [`Weight`].
#[derive(Debug, Clone, PartialEq)]
pub struct WeightInput {
    pub value: f64,
    pub unit: WeightUnit,
}

/// A validated weight/mass measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Weight, WeightInput, WeightUnit};
/// use arvo::traits::ValueObject;
///
/// let w = Weight::new(WeightInput { value: 75.0, unit: WeightUnit::Kg })?;
/// assert_eq!(w.value(), "75 kg");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Weight {
    value: f64,
    unit: WeightUnit,
    canonical: String,
}

impl ValueObject for Weight {
    type Input = WeightInput;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Weight", &input.value.to_string()));
        }
        let canonical = format!("{} {}", input.value, input.unit);
        Ok(Self {
            value: input.value,
            unit: input.unit,
            canonical,
        })
    }

    fn into_inner(self) -> Self::Input {
        WeightInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Weight {
    pub fn value(&self) -> &str {
        &self.canonical
    }

    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &WeightUnit {
        &self.unit
    }
}

impl TryFrom<&str> for Weight {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Weight", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "mg" => WeightUnit::Mg,
            "g" => WeightUnit::G,
            "kg" => WeightUnit::Kg,
            "t" => WeightUnit::T,
            "oz" => WeightUnit::Oz,
            "lb" => WeightUnit::Lb,
            _ => return Err(err()),
        };
        Self::new(WeightInput { value: val, unit })
    }
}

impl std::fmt::Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let w = Weight::new(WeightInput {
            value: 75.0,
            unit: WeightUnit::Kg,
        })
        .unwrap();
        assert_eq!(w.value(), "75 kg");
        assert_eq!(w.amount(), 75.0);
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Weight::new(WeightInput {
                value: 0.0,
                unit: WeightUnit::G
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Weight::new(WeightInput {
                value: -1.0,
                unit: WeightUnit::Kg
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Weight::new(WeightInput {
                value: f64::NAN,
                unit: WeightUnit::Kg
            })
            .is_err()
        );
    }

    #[test]
    fn try_from_parses_valid() {
        let w = Weight::try_from("70 kg").unwrap();
        assert_eq!(w.value(), "70 kg");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Weight::try_from("70").is_err());
    }

    #[test]
    fn try_from_rejects_unknown_unit() {
        assert!(Weight::try_from("70 stone").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Weight::try_from("70 kg").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Weight = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = Weight::try_from("70 kg").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("70 kg"));
    }
}
