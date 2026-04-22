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


#[cfg(feature = "sql")]
impl sqlx::Type<sqlx::Postgres> for Length {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "sql")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Length {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.canonical, buf)
    }
}

#[cfg(feature = "sql")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Length {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::try_from(s.as_str()).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(feature = "serde")]
impl From<Length> for String {
    fn from(v: Length) -> String {
        v.canonical
    }
}

impl TryFrom<String> for Length {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
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
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Length {
    value: f64,
    unit: LengthUnit,
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

    #[test]
    fn try_from_parses_valid() {
        let l = Length::try_from("1.5 km").unwrap();
        assert_eq!(l.value(), "1.5 km");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Length::try_from("1.5").is_err());
    }

    #[test]
    fn try_from_rejects_unknown_unit() {
        assert!(Length::try_from("1.5 parsec").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Length::try_from("1.5 km").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Length = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = Length::try_from("1.5 km").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("1.5 km"));
    }
}
