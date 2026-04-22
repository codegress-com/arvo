use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`CreditCardNumber`].
pub type CreditCardNumberInput = String;

/// Output type for [`CreditCardNumber`] — digits only, no separators.

/// A validated credit card number using the Luhn algorithm.
///
/// On construction spaces and hyphens are stripped; only digits are kept.
/// The Luhn algorithm is then applied: every second digit from the right is
/// doubled; if the result exceeds 9, subtract 9; the total must be divisible
/// by 10. Valid cards have 13–19 digits.
///
/// `Display` renders the masked form (last 4 digits visible); `value()` returns
/// the full digit string — treat it as sensitive data.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::CreditCardNumber;
/// use arvo::traits::ValueObject;
///
/// let card = CreditCardNumber::new("4532015112830366".into()).unwrap();
/// assert_eq!(card.last_four(), "0366");
/// assert_eq!(card.masked(), "**** **** **** 0366");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct CreditCardNumber(String);

impl ValueObject for CreditCardNumber {
    type Input = CreditCardNumberInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

        if digits.is_empty() {
            return Err(ValidationError::empty("CreditCardNumber"));
        }

        let len = digits.len();
        if !(13..=19).contains(&len) {
            return Err(ValidationError::invalid("CreditCardNumber", &digits));
        }

        if !luhn_valid(&digits) {
            return Err(ValidationError::invalid("CreditCardNumber", &digits));
        }

        Ok(Self(digits))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for CreditCardNumber {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl CreditCardNumber {
    /// Returns the last 4 digits, e.g. `"0366"`.
    pub fn last_four(&self) -> &str {
        let len = self.0.len();
        &self.0[len - 4..]
    }

    /// Returns a masked representation with only the last 4 digits visible.
    ///
    /// Digits are grouped in blocks of 4 separated by spaces, e.g.
    /// `"**** **** **** 0366"`.
    pub fn masked(&self) -> String {
        let len = self.0.len();
        let masked_count = len - 4;
        let full: String = "*".repeat(masked_count) + &self.0[masked_count..];
        full.chars().enumerate().fold(
            String::with_capacity(full.len() + full.len() / 4),
            |mut s, (i, c)| {
                if i > 0 && i % 4 == 0 {
                    s.push(' ');
                }
                s.push(c);
                s
            },
        )
    }
}

impl std::fmt::Display for CreditCardNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.masked())
    }
}

fn luhn_valid(digits: &str) -> bool {
    let sum: u32 = digits
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut d = (c as u8 - b'0') as u32;
            if i % 2 == 1 {
                d *= 2;
                if d > 9 {
                    d -= 9;
                }
            }
            d
        })
        .sum();
    sum % 10 == 0
}

impl TryFrom<String> for CreditCardNumber {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<CreditCardNumber> for String {
    fn from(v: CreditCardNumber) -> String {
        v.0
    }
}
impl TryFrom<&str> for CreditCardNumber {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_visa() {
        // 4532015112830366 — valid Luhn
        let c = CreditCardNumber::new("4532015112830366".into()).unwrap();
        assert_eq!(c.value(), "4532015112830366");
    }

    #[test]
    fn accepts_with_spaces() {
        let c = CreditCardNumber::new("4532 0151 1283 0366".into()).unwrap();
        assert_eq!(c.value(), "4532015112830366");
    }

    #[test]
    fn accepts_with_hyphens() {
        let c = CreditCardNumber::new("4532-0151-1283-0366".into()).unwrap();
        assert_eq!(c.value(), "4532015112830366");
    }

    #[test]
    fn last_four() {
        let c = CreditCardNumber::new("4532015112830366".into()).unwrap();
        assert_eq!(c.last_four(), "0366");
    }

    #[test]
    fn masked_16_digit() {
        let c = CreditCardNumber::new("4532015112830366".into()).unwrap();
        assert_eq!(c.masked(), "**** **** **** 0366");
    }

    #[test]
    fn display_is_masked() {
        let c = CreditCardNumber::new("4532015112830366".into()).unwrap();
        assert_eq!(c.to_string(), "**** **** **** 0366");
    }

    #[test]
    fn rejects_empty() {
        assert!(CreditCardNumber::new(String::new()).is_err());
    }

    #[test]
    fn rejects_too_short() {
        assert!(CreditCardNumber::new("123456789012".into()).is_err());
    }

    #[test]
    fn rejects_invalid_luhn() {
        // Change last digit to break Luhn
        assert!(CreditCardNumber::new("4532015112830367".into()).is_err());
    }

    #[test]
    fn rejects_too_long() {
        assert!(CreditCardNumber::new("45320151128303660000".into()).is_err());
    }
}
