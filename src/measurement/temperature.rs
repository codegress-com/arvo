use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Unit of temperature.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}


#[cfg(feature = "sql")]
impl sqlx::Type<sqlx::Postgres> for Temperature {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "sql")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Temperature {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.canonical, buf)
    }
}

#[cfg(feature = "sql")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Temperature {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Self::try_from(s.as_str()).map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureUnit::Celsius => write!(f, "°C"),
            TemperatureUnit::Fahrenheit => write!(f, "°F"),
            TemperatureUnit::Kelvin => write!(f, "K"),
        }
    }
}

/// Input for [`Temperature`].
#[derive(Debug, Clone, PartialEq)]
pub struct TemperatureInput {
    pub value: f64,
    pub unit: TemperatureUnit,
}

/// A validated temperature measurement.
///
/// **Validation:** value must be finite and above absolute zero for the given unit:
/// - Kelvin: `>= 0.0`
/// - Celsius: `>= -273.15`
/// - Fahrenheit: `>= -459.67`
///
/// # Example
///
/// ```rust,ignore
/// use arvo::measurement::{Temperature, TemperatureInput, TemperatureUnit};
/// use arvo::traits::ValueObject;
///
/// let t = Temperature::new(TemperatureInput { value: 100.0, unit: TemperatureUnit::Celsius })?;
/// assert_eq!(t.value(), "100 °C");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Temperature {
    value: f64,
    unit: TemperatureUnit,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Temperature {
    type Input = TemperatureInput;
    type Output = str;
    type Error = ValidationError;

    fn new(input: Self::Input) -> Result<Self, Self::Error> {
        if !input.value.is_finite() {
            return Err(ValidationError::invalid(
                "Temperature",
                &input.value.to_string(),
            ));
        }

        let min = match input.unit {
            TemperatureUnit::Kelvin => 0.0,
            TemperatureUnit::Celsius => -273.15,
            TemperatureUnit::Fahrenheit => -459.67,
        };

        if input.value < min {
            return Err(ValidationError::invalid(
                "Temperature",
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
        TemperatureInput {
            value: self.value,
            unit: self.unit,
        }
    }
}

impl Temperature {
    pub fn amount(&self) -> f64 {
        self.value
    }
    pub fn unit(&self) -> &TemperatureUnit {
        &self.unit
    }
}

impl TryFrom<&str> for Temperature {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Temperature", value);
        let (val_str, unit_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let val: f64 = val_str.trim().parse().map_err(|_| err())?;
        let unit = match unit_str.trim() {
            "°C" => TemperatureUnit::Celsius,
            "°F" => TemperatureUnit::Fahrenheit,
            "K" => TemperatureUnit::Kelvin,
            _ => return Err(err()),
        };
        Self::new(TemperatureInput { value: val, unit })
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_celsius() {
        let t = Temperature::new(TemperatureInput {
            value: 100.0,
            unit: TemperatureUnit::Celsius,
        })
        .unwrap();
        assert_eq!(t.value(), "100 °C");
    }

    #[test]
    fn accepts_absolute_zero_kelvin() {
        assert!(
            Temperature::new(TemperatureInput {
                value: 0.0,
                unit: TemperatureUnit::Kelvin
            })
            .is_ok()
        );
    }

    #[test]
    fn accepts_absolute_zero_celsius() {
        assert!(
            Temperature::new(TemperatureInput {
                value: -273.15,
                unit: TemperatureUnit::Celsius
            })
            .is_ok()
        );
    }

    #[test]
    fn rejects_below_absolute_zero_kelvin() {
        assert!(
            Temperature::new(TemperatureInput {
                value: -0.01,
                unit: TemperatureUnit::Kelvin
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_below_absolute_zero_celsius() {
        assert!(
            Temperature::new(TemperatureInput {
                value: -273.16,
                unit: TemperatureUnit::Celsius
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_below_absolute_zero_fahrenheit() {
        assert!(
            Temperature::new(TemperatureInput {
                value: -459.68,
                unit: TemperatureUnit::Fahrenheit
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_nan() {
        assert!(
            Temperature::new(TemperatureInput {
                value: f64::NAN,
                unit: TemperatureUnit::Celsius
            })
            .is_err()
        );
    }

    #[test]
    fn try_from_parses_valid() {
        let t = Temperature::try_from("100 °C").unwrap();
        assert_eq!(t.value(), "100 °C");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Temperature::try_from("100").is_err());
    }

    #[test]
    fn try_from_rejects_below_absolute_zero() {
        assert!(Temperature::try_from("-500 K").is_err());
    }
}
