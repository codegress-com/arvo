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

/// Output type for [`ExchangeRate`] — canonical `"<FROM>/<TO> <rate>"` string.
pub type ExchangeRateOutput = String;

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
pub struct ExchangeRate {
    from: CurrencyCode,
    to: CurrencyCode,
    rate: Decimal,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for ExchangeRate {
    type Input = ExchangeRateInput;
    type Output = ExchangeRateOutput;
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

    fn value(&self) -> &Self::Output {
        &self.canonical
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

impl std::fmt::Display for ExchangeRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ValueObject;

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
}
