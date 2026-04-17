use crate::errors::ValidationError;
use crate::prelude::ValueObject;
use once_cell::sync::Lazy;
use regex::Regex;

/// Compiled email regex — evaluated once at first use.
///
/// Pattern checks for a local part, `@`, a domain, and a TLD of at least
/// 2 characters. Full RFC 5322 compliance is intentionally out of scope.
static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]{2,}$").unwrap());

/// A validated, normalised email address.
///
/// On construction the value is trimmed and lowercased, so
/// `"User@Example.COM"` and `"user@example.com"` produce equal instances.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::contact::EmailAddress;
/// use arvo::traits::ValueObject;
///
/// let email = EmailAddress::new("User@Example.COM".into()).unwrap();
/// assert_eq!(email.value(), "user@example.com");
///
/// assert!(EmailAddress::new("not-an-email".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct EmailAddress(String);

impl ValueObject for EmailAddress {
    type Raw = String;
    type Error = ValidationError;

    fn new(value: Self::Raw) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_lowercase();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("EmailAddress"));
        }

        if !EMAIL_REGEX.is_match(&trimmed) {
            return Err(ValidationError::invalid("EmailAddress", &trimmed));
        }

        Ok(Self(trimmed))
    }

    fn value(&self) -> &Self::Raw {
        &self.0
    }

    fn into_inner(self) -> Self::Raw {
        self.0
    }
}

/// Allows ergonomic construction from a string literal: `"a@b.com".try_into()`
impl TryFrom<&str> for EmailAddress {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

/// Displays the normalized email address as a plain string.
impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_to_lowercase() {
        let e = EmailAddress::new("User@Example.COM".into()).unwrap();
        assert_eq!(e.value(), "user@example.com");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let e = EmailAddress::new("  hello@example.com  ".into()).unwrap();
        assert_eq!(e.value(), "hello@example.com");
    }

    #[test]
    fn rejects_missing_at_sign() {
        assert!(EmailAddress::new("notanemail".into()).is_err());
    }

    #[test]
    fn rejects_empty_string() {
        assert!(EmailAddress::new(String::new()).is_err());
    }

    #[test]
    fn equal_after_normalisation() {
        let a = EmailAddress::new("User@Example.COM".into()).unwrap();
        let b = EmailAddress::new("user@example.com".into()).unwrap();
        assert_eq!(a, b);
    }
}
