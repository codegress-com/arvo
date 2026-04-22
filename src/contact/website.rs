use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};
use url::Url;

/// Input type for [`Website`] — a raw string before validation.
pub type WebsiteInput = String;

/// Output type for [`Website`] — a normalised URL string.

/// A validated website URL.
///
/// Accepts `http` and `https` schemes only. On construction the value is
/// parsed and normalised (scheme and host lowercased) so `"HTTPS://Example.COM/"`
/// and `"https://example.com/"` produce equal instances.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::contact::Website;
/// use arvo::traits::ValueObject;
///
/// let site = Website::new("https://Example.COM/path".into()).unwrap();
/// assert_eq!(site.value(), "https://example.com/path");
///
/// assert!(Website::new("ftp://example.com".into()).is_err());
/// assert!(Website::new("not-a-url".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Website(String);

impl ValueObject for Website {
    type Input = WebsiteInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("Website"));
        }

        let parsed =
            Url::parse(trimmed).map_err(|_| ValidationError::invalid("Website", trimmed))?;

        match parsed.scheme() {
            "http" | "https" => {}
            _ => return Err(ValidationError::invalid("Website", trimmed)),
        }

        if parsed.host().is_none() {
            return Err(ValidationError::invalid("Website", trimmed));
        }

        Ok(Self(parsed.to_string()))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for Website {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl Website {
    /// Returns `true` if the scheme is `https`.
    pub fn is_https(&self) -> bool {
        self.0.starts_with("https://")
    }

    /// Returns the host portion of the URL, e.g. `"example.com"`.
    pub fn host(&self) -> &str {
        let after_scheme = self
            .0
            .find("://")
            .map(|i| &self.0[i + 3..])
            .unwrap_or(&self.0);
        after_scheme.split('/').next().unwrap_or("")
    }
}

/// Allows ergonomic construction from a string literal: `"https://example.com".try_into()`

impl TryFrom<String> for Website {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Website> for String {
    fn from(v: Website) -> String {
        v.0
    }
}
impl TryFrom<&str> for Website {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

/// Displays the website as its normalised URL string.
impl std::fmt::Display for Website {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_https_url() {
        let w = Website::new("https://example.com".into()).unwrap();
        assert_eq!(w.value(), "https://example.com/");
    }

    #[test]
    fn accepts_http_url() {
        let w = Website::new("http://example.com".into()).unwrap();
        assert_eq!(w.value(), "http://example.com/");
    }

    #[test]
    fn normalises_host_to_lowercase() {
        let w = Website::new("https://EXAMPLE.COM/Path".into()).unwrap();
        assert_eq!(w.value(), "https://example.com/Path");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let w = Website::new("  https://example.com  ".into()).unwrap();
        assert_eq!(w.value(), "https://example.com/");
    }

    #[test]
    fn rejects_ftp_scheme() {
        assert!(Website::new("ftp://example.com".into()).is_err());
    }

    #[test]
    fn rejects_non_url() {
        assert!(Website::new("not-a-url".into()).is_err());
    }

    #[test]
    fn rejects_empty_string() {
        assert!(Website::new(String::new()).is_err());
    }

    #[test]
    fn equal_after_normalisation() {
        let a = Website::new("https://example.com/".into()).unwrap();
        let b = Website::new("https://example.com/".into()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn is_https_returns_true_for_https() {
        let w = Website::new("https://example.com".into()).unwrap();
        assert!(w.is_https());
    }

    #[test]
    fn is_https_returns_false_for_http() {
        let w = Website::new("http://example.com".into()).unwrap();
        assert!(!w.is_https());
    }

    #[test]
    fn host_returns_domain() {
        let w = Website::new("https://example.com/path".into()).unwrap();
        assert_eq!(w.host(), "example.com");
    }

    #[test]
    fn try_from_str() {
        let w: Website = "https://example.com".try_into().unwrap();
        assert!(w.is_https());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Website::try_from("https://example.com").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Website = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Website, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
