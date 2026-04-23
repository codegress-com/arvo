use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`VatNumber`].
pub type VatNumberInput = String;

/// EU VAT country prefixes (sorted for binary search).
static EU_PREFIXES: &[&str] = &[
    "AT", "BE", "BG", "CY", "CZ", "DE", "DK", "EE", "EL", "ES", "FI", "FR", "HR", "HU", "IE", "IT",
    "LT", "LU", "LV", "MT", "NL", "PL", "PT", "RO", "SE", "SI", "SK", "XI",
];

/// A validated EU VAT number.
///
/// On construction the input is trimmed, uppercased, and internal spaces are
/// stripped. The value must start with a known EU country prefix (2 letters)
/// followed by 2–13 alphanumeric characters.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::VatNumber;
/// use arvo::traits::ValueObject;
///
/// let vat = VatNumber::new("CZ12345678".into()).unwrap();
/// assert_eq!(vat.value(), "CZ12345678");
/// assert_eq!(vat.country_prefix(), "CZ");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct VatNumber(String);

impl ValueObject for VatNumber {
    type Input = VatNumberInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised: String = value
            .trim()
            .to_uppercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        if normalised.is_empty() {
            return Err(ValidationError::empty("VatNumber"));
        }

        if normalised.len() < 4 {
            return Err(ValidationError::invalid("VatNumber", &normalised));
        }

        let prefix = &normalised[..2];
        if !prefix.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ValidationError::invalid("VatNumber", &normalised));
        }

        if EU_PREFIXES.binary_search(&prefix).is_err() {
            return Err(ValidationError::invalid("VatNumber", &normalised));
        }

        let suffix = &normalised[2..];
        if suffix.len() < 2 || suffix.len() > 13 {
            return Err(ValidationError::invalid("VatNumber", &normalised));
        }

        if !suffix.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(ValidationError::invalid("VatNumber", &normalised));
        }

        Ok(Self(normalised))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for VatNumber {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl VatNumber {
    /// Returns the 2-letter EU country prefix, e.g. `"CZ"`.
    pub fn country_prefix(&self) -> &str {
        &self.0[..2]
    }
}

impl TryFrom<String> for VatNumber {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<VatNumber> for String {
    fn from(v: VatNumber) -> String {
        v.0
    }
}
impl TryFrom<&str> for VatNumber {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for VatNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_czech_vat() {
        let v = VatNumber::new("CZ12345678".into()).unwrap();
        assert_eq!(v.value(), "CZ12345678");
    }

    #[test]
    fn accepts_german_vat() {
        assert!(VatNumber::new("DE123456789".into()).is_ok());
    }

    #[test]
    fn accepts_xi_prefix() {
        assert!(VatNumber::new("XI123456789".into()).is_ok());
    }

    #[test]
    fn normalises_to_uppercase() {
        let v = VatNumber::new("cz12345678".into()).unwrap();
        assert_eq!(v.value(), "CZ12345678");
    }

    #[test]
    fn strips_internal_spaces() {
        let v = VatNumber::new("CZ 1234 5678".into()).unwrap();
        assert_eq!(v.value(), "CZ12345678");
    }

    #[test]
    fn country_prefix_accessor() {
        let v = VatNumber::new("CZ12345678".into()).unwrap();
        assert_eq!(v.country_prefix(), "CZ");
    }

    #[test]
    fn rejects_empty() {
        assert!(VatNumber::new(String::new()).is_err());
    }

    #[test]
    fn rejects_unknown_prefix() {
        assert!(VatNumber::new("US12345678".into()).is_err());
    }

    #[test]
    fn rejects_suffix_too_short() {
        assert!(VatNumber::new("CZ1".into()).is_err());
    }

    #[test]
    fn rejects_suffix_too_long() {
        assert!(VatNumber::new("CZ12345678901234".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let v: VatNumber = "DE123456789".try_into().unwrap();
        assert_eq!(v.value(), "DE123456789");
    }
}
