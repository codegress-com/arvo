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

    /// Returns the sum of `self` and `other`. Fails if currencies differ.
    pub fn add(&self, other: &Money) -> Result<Money, ValidationError> {
        if self.currency != other.currency {
            return Err(ValidationError::invalid(
                "Money",
                &format!("cannot add {} and {}", self.currency, other.currency),
            ));
        }
        let sum = self.amount + other.amount;
        let canonical = format!("{:.2} {}", sum, self.currency);
        Ok(Money { amount: sum, currency: self.currency.clone(), canonical })
    }

    /// Returns the difference `self - other`. Fails if currencies differ.
    pub fn sub(&self, other: &Money) -> Result<Money, ValidationError> {
        if self.currency != other.currency {
            return Err(ValidationError::invalid(
                "Money",
                &format!("cannot subtract {} and {}", self.currency, other.currency),
            ));
        }
        let diff = self.amount - other.amount;
        let canonical = format!("{:.2} {}", diff, self.currency);
        Ok(Money { amount: diff, currency: self.currency.clone(), canonical })
    }

    /// Returns the negation of this amount (e.g. a debt).
    pub fn neg(&self) -> Money {
        let negated = -self.amount;
        let canonical = format!("{:.2} {}", negated, self.currency);
        Money { amount: negated, currency: self.currency.clone(), canonical }
    }
}

impl TryFrom<&str> for Money {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("Money", value);
        let (amount_str, currency_str) = value.trim().rsplit_once(' ').ok_or_else(err)?;
        let amount: rust_decimal::Decimal = amount_str.trim().parse().map_err(|_| err())?;
        let currency = CurrencyCode::new(currency_str.trim().to_owned()).map_err(|_| err())?;
        Self::new(MoneyInput { amount, currency })
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
    fn add_same_currency() {
        let a = Money::new(MoneyInput { amount: "10.00".parse().unwrap(), currency: eur() }).unwrap();
        let b = Money::new(MoneyInput { amount: "5.50".parse().unwrap(), currency: eur() }).unwrap();
        let result = a.add(&b).unwrap();
        assert_eq!(result.value(), "15.50 EUR");
    }

    #[test]
    fn add_different_currencies_fails() {
        let a = Money::new(MoneyInput { amount: "10.00".parse().unwrap(), currency: eur() }).unwrap();
        let b = Money::new(MoneyInput { amount: "5.00".parse().unwrap(), currency: usd() }).unwrap();
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn sub_same_currency() {
        let a = Money::new(MoneyInput { amount: "10.00".parse().unwrap(), currency: eur() }).unwrap();
        let b = Money::new(MoneyInput { amount: "3.00".parse().unwrap(), currency: eur() }).unwrap();
        let result = a.sub(&b).unwrap();
        assert_eq!(result.value(), "7.00 EUR");
    }

    #[test]
    fn neg_returns_negated_amount() {
        let m = Money::new(MoneyInput { amount: "10.00".parse().unwrap(), currency: eur() }).unwrap();
        assert_eq!(m.neg().value(), "-10.00 EUR");
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

    #[test]
    fn try_from_parses_valid() {
        let m = Money::try_from("10.50 EUR").unwrap();
        assert_eq!(m.value(), "10.50 EUR");
    }

    #[test]
    fn try_from_rejects_no_space() {
        assert!(Money::try_from("10.50EUR").is_err());
    }

    #[test]
    fn try_from_rejects_invalid_currency() {
        assert!(Money::try_from("10.50 INVALID").is_err());
    }
}
