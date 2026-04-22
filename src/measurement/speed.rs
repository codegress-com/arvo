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


#[cfg(feature = "sql")]
impl sqlx::Type<sqlx::Postgres> for Speed {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "sql")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Speed {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.canonical, buf)
    }
}

#[cfg(feature = "sql")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Speed {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::try_from(s.as_str()).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
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

impl TryFrom<&str> for Speed {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Speed", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "m/s" => SpeedUnit::Ms,
            "km/h" => SpeedUnit::Kmh,
            "mph" => SpeedUnit::Mph,
            "kn" => SpeedUnit::Kn,
            _ => return Err(err()),
        };
        Self::new(SpeedInput { value: val, unit })
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

    #[test]
    fn try_from_parses_valid() {
        let s = Speed::try_from("120 km/h").unwrap();
        assert_eq!(s.value(), "120 km/h");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Speed::try_from("120").is_err());
    }

    #[test]
    fn try_from_rejects_unknown_unit() {
        assert!(Speed::try_from("120 warp").is_err());
    }
}
