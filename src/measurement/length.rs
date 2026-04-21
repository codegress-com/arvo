use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of length.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LengthUnit {
    Mm,
    Cm,
    M,
    Km,
    In,
    Ft,
}

impl std::fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Mm => write!(f, "mm"),
            LengthUnit::Cm => write!(f, "cm"),
            LengthUnit::M => write!(f, "m"),
            LengthUnit::Km => write!(f, "km"),
            LengthUnit::In => write!(f, "in"),
            LengthUnit::Ft => write!(f, "ft"),
        }
    }
}

/// Input for [`Length`].
#[derive(Debug, Clone, PartialEq)]
pub struct LengthInput {
    pub value: f64,
    pub unit: LengthUnit,
}

/// A validated length measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Length, LengthInput, LengthUnit};
/// use arvo::traits::ValueObject;
///
/// let len = Length::new(LengthInput { value: 1.80, unit: LengthUnit::M })?;
/// assert_eq!(len.value(), "1.80 m");
/// assert_eq!(len.amount(), 1.80);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Length {
    value: f64,
    unit: LengthUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Length {
    type Input = LengthInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Length", &input.value.to_string()));
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
        LengthInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Length {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &LengthUnit {
        &self.unit
    }
}

impl TryFrom<&str> for Length {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Length", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "mm" => LengthUnit::Mm,
            "cm" => LengthUnit::Cm,
            "m" => LengthUnit::M,
            "km" => LengthUnit::Km,
            "in" => LengthUnit::In,
            "ft" => LengthUnit::Ft,
            _ => return Err(err()),
        };
        Self::new(LengthInput { value: val, unit })
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let l = Length::new(LengthInput {
            value: 1.80,
            unit: LengthUnit::M,
        })
        .unwrap();
        assert_eq!(l.value(), "1.8 m");
        assert_eq!(l.amount(), 1.80);
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Length::new(LengthInput {
                value: 0.0,
                unit: LengthUnit::Cm
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Length::new(LengthInput {
                value: -1.0,
                unit: LengthUnit::M
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Length::new(LengthInput {
                value: f64::NAN,
                unit: LengthUnit::M
            })
            .is_err()
        );
    }

    #[test]
    fn all_units_display() {
        for unit in [
            LengthUnit::Mm,
            LengthUnit::Cm,
            LengthUnit::M,
            LengthUnit::Km,
            LengthUnit::In,
            LengthUnit::Ft,
        ] {
            assert!(Length::new(LengthInput { value: 1.0, unit }).is_ok());
        }
    }
}
