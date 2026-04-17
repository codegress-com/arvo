use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`CountryCode`] — a raw string before validation.
pub type CountryCodeInput = String;

/// Output type for [`CountryCode`] — a normalised uppercase string.
pub type CountryCodeOutput = String;

/// A validated ISO 3166-1 alpha-2 country code.
///
/// On construction the value is trimmed and uppercased, so `"cz"` and `"CZ"`
/// produce equal instances. Only ASCII letters are accepted — digits and
/// special characters are rejected.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::contact::CountryCode;
/// use arvo::traits::ValueObject;
///
/// let code = CountryCode::new("cz".into()).unwrap();
/// assert_eq!(code.value(), "CZ");
///
/// assert!(CountryCode::new("USA".into()).is_err()); // 3 letters — invalid
/// assert!(CountryCode::new("C1".into()).is_err());  // digit — invalid
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CountryCode(String);

impl ValueObject for CountryCode {
    type Input = CountryCodeInput;
    type Output = CountryCodeOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised = value.trim().to_uppercase();

        // ISO 3166-1 alpha-2 is exactly 2 ASCII letters, nothing else.
        let valid = normalised.len() == 2 && normalised.chars().all(|c| c.is_ascii_alphabetic());

        if !valid {
            return Err(ValidationError::invalid("CountryCode", &normalised));
        }

        Ok(Self(normalised))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

/// Allows ergonomic construction from a string literal: `"CZ".try_into()`
impl TryFrom<&str> for CountryCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

/// Displays the country code as an uppercase two-letter string.
impl std::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_to_uppercase() {
        let c = CountryCode::new("cz".into()).unwrap();
        assert_eq!(c.value(), "CZ");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let c = CountryCode::new("  de  ".into()).unwrap();
        assert_eq!(c.value(), "DE");
    }

    #[test]
    fn rejects_three_letter_code() {
        assert!(CountryCode::new("USA".into()).is_err());
    }

    #[test]
    fn rejects_single_letter() {
        assert!(CountryCode::new("C".into()).is_err());
    }

    #[test]
    fn rejects_digit_in_code() {
        assert!(CountryCode::new("C1".into()).is_err());
    }

    #[test]
    fn rejects_empty_string() {
        assert!(CountryCode::new(String::new()).is_err());
    }

    #[test]
    fn equal_after_normalisation() {
        let a = CountryCode::new("cz".into()).unwrap();
        let b = CountryCode::new("CZ".into()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn try_from_str() {
        let c: CountryCode = "DE".try_into().unwrap();
        assert_eq!(c.value(), "DE");
    }
}
