use rust_decimal::Decimal;

use crate::errors::ValidationError;
use crate::traits::ValueObject;

use super::currency_code::CurrencyCode;

/// Input type for [`ExchangeRate`] construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExchangeRateInput {
    /// Source currency.
    pub from: CurrencyCode,
    /// Target currency.
    pub to: CurrencyCode,
    /// Exchange rate — must be strictly positive.
    pub rate: Decimal,
}

/// A validated currency exchange rate.
///
/// The `rate` must be strictly positive (> 0) and `from` must differ from `to`.
/// The canonical output is formatted as `"<FROM>/<TO> <rate>"`,
/// e.g. `"EUR/USD 1.0850"`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::{CurrencyCode, ExchangeRate, ExchangeRateInput};
/// use arvo::traits::ValueObject;
///
/// let rate = ExchangeRate::new(ExchangeRateInput {
///     from: CurrencyCode::new("EUR".into()).unwrap(),
///     to:   CurrencyCode::new("USD".into()).unwrap(),
///     rate: "1.0850".parse().unwrap(),
/// }).unwrap();
///
/// assert_eq!(rate.value(), "EUR/USD 1.0850");
/// assert_eq!(rate.from().value(), "EUR");
/// assert_eq!(rate.to().value(), "USD");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct ExchangeRate {
    from: CurrencyCode,
    to: CurrencyCode,
    rate: Decimal,
    canonical: String,
}

impl ValueObject for ExchangeRate {
    type Input = ExchangeRateInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value.from == value.to {
            return Err(ValidationError::invalid(
                "ExchangeRate",
                &format!("{}/{}", value.from, value.to),
            ));
        }

        if value.rate <= Decimal::ZERO {
            return Err(ValidationError::invalid(
                "ExchangeRate",
                &value.rate.to_string(),
            ));
        }

        let canonical = format!("{}/{} {}", value.from, value.to, value.rate);
        Ok(Self {
            from: value.from,
            to: value.to,
            rate: value.rate,
            canonical,
        })
    }

    fn into_inner(self) -> Self::Input {
        ExchangeRateInput {
            from: self.from,
            to: self.to,
            rate: self.rate,
        }
    }
}

impl ExchangeRate {
    pub fn value(&self) -> &str {
        &self.canonical
    }

    /// Returns the source currency.
    pub fn from(&self) -> &CurrencyCode {
        &self.from
    }

    /// Returns the target currency.
    pub fn to(&self) -> &CurrencyCode {
        &self.to
    }

    /// Returns the exchange rate.
    pub fn rate(&self) -> &Decimal {
        &self.rate
    }
}

impl TryFrom<&str> for ExchangeRate {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("ExchangeRate", value);
        let (pair_str, rate_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let (from_str, to_str) = pair_str.split_once('/').ok_or_else(err)?;
        let from = CurrencyCode::new(from_str.to_owned()).map_err(|_| err())?;
        let to = CurrencyCode::new(to_str.to_owned()).map_err(|_| err())?;
        let rate: rust_decimal::Decimal = rate_str.trim().parse().map_err(|_| err())?;
        Self::new(ExchangeRateInput { from, to, rate })
    }
}

#[cfg(feature = "serde")]
impl From<ExchangeRate> for String {
    fn from(v: ExchangeRate) -> String {
        v.canonical
    }
}

impl TryFrom<String> for ExchangeRate {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl std::fmt::Display for ExchangeRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PrimitiveValue, ValueObject};

    fn eur() -> CurrencyCode {
        CurrencyCode::new("EUR".into()).unwrap()
    }

    fn usd() -> CurrencyCode {
        CurrencyCode::new("USD".into()).unwrap()
    }

    #[test]
    fn constructs_valid_rate() {
        let r = ExchangeRate::new(ExchangeRateInput {
            from: eur(),
            to: usd(),
            rate: "1.0850".parse().unwrap(),
        })
        .unwrap();
        assert_eq!(r.value(), "EUR/USD 1.0850");
    }

    #[test]
    fn from_accessor() {
        let r = ExchangeRate::new(ExchangeRateInput {
            from: eur(),
            to: usd(),
            rate: "1.0850".parse().unwrap(),
        })
        .unwrap();
        assert_eq!(r.from().value(), "EUR");
    }

    #[test]
    fn to_accessor() {
        let r = ExchangeRate::new(ExchangeRateInput {
            from: eur(),
            to: usd(),
            rate: "1.0850".parse().unwrap(),
        })
        .unwrap();
        assert_eq!(r.to().value(), "USD");
    }

    #[test]
    fn rate_accessor() {
        let rate_val: Decimal = "1.0850".parse().unwrap();
        let r = ExchangeRate::new(ExchangeRateInput {
            from: eur(),
            to: usd(),
            rate: rate_val,
        })
        .unwrap();
        assert_eq!(*r.rate(), "1.0850".parse::<Decimal>().unwrap());
    }

    #[test]
    fn rejects_zero_rate() {
        assert!(
            ExchangeRate::new(ExchangeRateInput {
                from: eur(),
                to: usd(),
                rate: Decimal::ZERO,
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_negative_rate() {
        assert!(
            ExchangeRate::new(ExchangeRateInput {
                from: eur(),
                to: usd(),
                rate: "-1".parse().unwrap(),
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_same_currency() {
        assert!(
            ExchangeRate::new(ExchangeRateInput {
                from: eur(),
                to: eur(),
                rate: "1".parse().unwrap(),
            })
            .is_err()
        );
    }

    #[test]
    fn display_matches_value() {
        let r = ExchangeRate::new(ExchangeRateInput {
            from: eur(),
            to: usd(),
            rate: "1.0850".parse().unwrap(),
        })
        .unwrap();
        assert_eq!(r.to_string(), r.value().to_owned());
    }

    #[test]
    fn try_from_parses_valid() {
        let r = ExchangeRate::try_from("EUR/USD 1.0850").unwrap();
        assert_eq!(r.value(), "EUR/USD 1.0850");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(ExchangeRate::try_from("EURUSD1.0850").is_err());
    }

    #[test]
    fn try_from_rejects_missing_slash() {
        assert!(ExchangeRate::try_from("EURUSD 1.0850").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = ExchangeRate::try_from("EUR/USD 1.0850").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: ExchangeRate = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = ExchangeRate::try_from("EUR/USD 1.0850").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("EUR/USD 1.0850"));
    }
}
