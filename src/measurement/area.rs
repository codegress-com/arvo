use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of area.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AreaUnit {
    Mm2,
    Cm2,
    M2,
    Km2,
    In2,
    Ft2,
    Ha,
}

impl std::fmt::Display for AreaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AreaUnit::Mm2 => write!(f, "mm²"),
            AreaUnit::Cm2 => write!(f, "cm²"),
            AreaUnit::M2 => write!(f, "m²"),
            AreaUnit::Km2 => write!(f, "km²"),
            AreaUnit::In2 => write!(f, "in²"),
            AreaUnit::Ft2 => write!(f, "ft²"),
            AreaUnit::Ha => write!(f, "ha"),
        }
    }
}

/// Input for [`Area`].
#[derive(Debug, Clone, PartialEq)]
pub struct AreaInput {
    pub value: f64,
    pub unit: AreaUnit,
}

/// A validated area measurement.
///
/// **Validation:** value must be finite and non-negative.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Area, AreaInput, AreaUnit};
/// use arvo::traits::ValueObject;
///
/// let a = Area::new(AreaInput { value: 50.0, unit: AreaUnit::M2 })?;
/// assert_eq!(a.value(), "50 m²");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Area {
    value: f64,
    unit: AreaUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Area {
    type Input = AreaInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() || input.value < 0.0 {
            return Err(ValidationError::invalid("Area", &input.value.to_string()));
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
        AreaInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Area {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &AreaUnit {
        &self.unit
    }
}

impl TryFrom<AreaInput> for Area {
    type Error = ValidationError;

    fn try_from(value: AreaInput) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid() {
        let a = Area::new(AreaInput {
            value: 50.0,
            unit: AreaUnit::M2,
        })
        .unwrap();
        assert_eq!(a.value(), "50 m²");
    }

    #[test]
    fn accepts_zero() {
        assert!(
            Area::new(AreaInput {
                value: 0.0,
                unit: AreaUnit::M2
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_negative() {
        assert!(
            Area::new(AreaInput {
                value: -1.0,
                unit: AreaUnit::M2
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Area::new(AreaInput {
                value: f64::NAN,
                unit: AreaUnit::M2
            })
            .is_err()
        );
    }
}
