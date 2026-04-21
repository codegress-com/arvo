use rust_decimal::Decimal;

use crate::errors::ValidationError;
use crate::traits::ValueObject;

use super::currency_code::CurrencyCode;

/// Input type for [`Money`] construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoneyInput {
    /// Monetary amount. Negative values represent debts.
    pub amount: Decimal,
    /// ISO 4217 currency code.
    pub currency: CurrencyCode,
}

/// Output type for [`Money`] — canonical `"<amount> <currency>"` string.
pub type MoneyOutput = String;

/// A validated monetary amount with an associated currency.
///
/// `amount` may be any finite `Decimal` value; negative amounts represent debts.
/// The `currency` must be a valid [`CurrencyCode`]. The canonical output is
/// formatted as `"<amount> <CURRENCY>"` with two decimal places, e.g. `"10.00 EUR"`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::{CurrencyCode, Money, MoneyInput};
/// use arvo::traits::ValueObject;
///
/// let money = Money::new(MoneyInput {
///     amount: "10.50".parse().unwrap(),
///     currency: CurrencyCode::new("EUR".into()).unwrap(),
/// }).unwrap();
///
/// assert_eq!(money.value(), "10.50 EUR");
/// assert_eq!(money.currency().value(), "EUR");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Money {
    amount: Decimal,
    currency: CurrencyCode,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for Money {
    type Input = MoneyInput;
    type Output = MoneyOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let canonical = format!("{:.2} {}", value.amount, value.currency);
        Ok(Self {
            amount: value.amount,
            currency: value.currency,
            canonical,
        })
    }

    fn value(&self) -> &Self::Output {
        &self.canonical
    }

    fn into_inner(self) -> Self::Input {
        MoneyInput {
            amount: self.amount,
            currency: self.currency,
        }
    }
}

impl Money {
    /// Returns the monetary amount.
    pub fn amount(&self) -> &Decimal {
        &self.amount
    }

    /// Returns the currency code.
    pub fn currency(&self) -> &CurrencyCode {
        &self.currency
    }
}

impl std::fmt::Display for Money {
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
    fn constructs_valid_money() {
        let m = Money::new(MoneyInput {
            amount: "10.50".parse().unwrap(),
            currency: eur(),
        })
        .unwrap();
        assert_eq!(m.value(), "10.50 EUR");
    }

    #[test]
    fn formats_with_two_decimal_places() {
        let m = Money::new(MoneyInput {
            amount: "100".parse().unwrap(),
            currency: usd(),
        })
        .unwrap();
        assert_eq!(m.value(), "100.00 USD");
    }

    #[test]
    fn allows_negative_amount() {
        let m = Money::new(MoneyInput {
            amount: "-5.00".parse().unwrap(),
            currency: eur(),
        })
        .unwrap();
        assert_eq!(m.value(), "-5.00 EUR");
    }

    #[test]
    fn allows_zero_amount() {
        let m = Money::new(MoneyInput {
            amount: Decimal::ZERO,
            currency: eur(),
        })
        .unwrap();
        assert_eq!(m.value(), "0.00 EUR");
    }

    #[test]
    fn amount_accessor() {
        let m = Money::new(MoneyInput {
            amount: "42.00".parse().unwrap(),
            currency: eur(),
        })
        .unwrap();
        assert_eq!(m.amount(), &"42.00".parse::<Decimal>().unwrap());
    }

    #[test]
    fn currency_accessor() {
        let m = Money::new(MoneyInput {
            amount: Decimal::ZERO,
            currency: eur(),
        })
        .unwrap();
        assert_eq!(m.currency().value(), "EUR");
    }

    #[test]
    fn display_matches_value() {
        let m = Money::new(MoneyInput {
            amount: "9.99".parse().unwrap(),
            currency: usd(),
        })
        .unwrap();
        assert_eq!(m.to_string(), m.value().to_owned());
    }

    #[test]
    fn into_inner_roundtrip() {
        let input = MoneyInput {
            amount: "1.00".parse().unwrap(),
            currency: eur(),
        };
        let m = Money::new(input.clone()).unwrap();
        assert_eq!(m.into_inner(), input);
    }
}
