use crate::errors::ValidationError;
use crate::traits::ValueObject;
use once_cell::sync::Lazy;
use regex::Regex;

/// Input type for [`EmailAddress`] — a raw string before validation.
pub type EmailAddressInput = String;

/// Output type for [`EmailAddress`] — a normalised lowercase string.
pub type EmailAddressOutput = String;

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
    type Input  = EmailAddressInput;
    type Output = EmailAddressOutput;
    type Error  = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised = value.trim().to_lowercase();

        if normalised.is_empty() {
            return Err(ValidationError::empty("EmailAddress"));
        }

        if !EMAIL_REGEX.is_match(&normalised) {
            return Err(ValidationError::invalid("EmailAddress", &normalised));
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

impl EmailAddress {
    /// Returns the local part of the email address (before `@`), e.g. `"user"`.
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }

    /// Returns the domain part of the email address (after `@`), e.g. `"example.com"`.
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

/// Allows ergonomic construction from a string literal: `"a@b.com".try_into()`
impl TryFrom<&str> for EmailAddress {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

/// Displays the normalised email address as a plain string.
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

    #[test]
    fn local_part_returns_part_before_at() {
        let e = EmailAddress::new("user@example.com".into()).unwrap();
        assert_eq!(e.local_part(), "user");
    }

    #[test]
    fn domain_returns_part_after_at() {
        let e = EmailAddress::new("user@example.com".into()).unwrap();
        assert_eq!(e.domain(), "example.com");
    }

    #[test]
    fn try_from_str() {
        let e: EmailAddress = "hello@example.com".try_into().unwrap();
        assert_eq!(e.value(), "hello@example.com");
    }
}
