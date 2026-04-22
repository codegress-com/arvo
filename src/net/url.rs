use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Url`].
pub type UrlInput = String;

/// Output type for [`Url`].
pub type UrlOutput = String;

/// A validated URL. Accepts `http`, `https`, `ftp`, `ftps`, `ws`, and `wss` schemes.
/// Scheme and host are normalised to lowercase on construction.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::Url;
/// use arvo::traits::ValueObject;
///
/// let url = Url::new("HTTPS://Example.COM/path".into())?;
/// assert_eq!(url.value(), "https://example.com/path");
/// assert_eq!(url.scheme(), "https");
/// assert_eq!(url.host(), "example.com");
///
/// let url: Url = "https://example.com".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Url(String);

const ALLOWED_SCHEMES: &[&str] = &["ftp", "ftps", "http", "https", "ws", "wss"];

impl ValueObject for Url {
    type Input = UrlInput;
    type Output = UrlOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("Url"));
        }

        let parsed =
            ::url::Url::parse(trimmed).map_err(|_| ValidationError::invalid("Url", trimmed))?;

        if parsed.host_str().is_none() {
            return Err(ValidationError::invalid("Url", trimmed));
        }

        let scheme = parsed.scheme();
        if ALLOWED_SCHEMES.binary_search(&scheme).is_err() {
            return Err(ValidationError::invalid("Url", trimmed));
        }

        let canonical = parsed.to_string();

        Ok(Self(canonical))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl Url {
    /// Returns the scheme, e.g. `"https"`.
    pub fn scheme(&self) -> &str {
        self.0.split("://").next().unwrap_or("")
    }

    /// Returns the host without port, e.g. `"example.com"`.
    pub fn host(&self) -> &str {
        let after_scheme = self.0.split("://").nth(1).unwrap_or("");
        let host_and_port = after_scheme
            .split('/')
            .next()
            .unwrap_or("")
            .split('?')
            .next()
            .unwrap_or("");
        if host_and_port.starts_with('[') {
            // IPv6 literal: "[::1]:8080" → "[::1]"
            if let Some(i) = host_and_port.find(']') {
                return &host_and_port[..=i];
            }
            return host_and_port;
        }
        host_and_port.split(':').next().unwrap_or(host_and_port)
    }
}

impl TryFrom<&str> for Url {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_https_url() {
        let url = Url::new("https://example.com/path".into()).unwrap();
        assert_eq!(url.value(), "https://example.com/path");
    }

    #[test]
    fn normalises_scheme_and_host() {
        let url = Url::new("HTTPS://EXAMPLE.COM/path".into()).unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host(), "example.com");
    }

    #[test]
    fn accepts_http() {
        assert!(Url::new("http://example.com".into()).is_ok());
    }

    #[test]
    fn accepts_ftp() {
        assert!(Url::new("ftp://files.example.com/file.txt".into()).is_ok());
    }

    #[test]
    fn accepts_ws() {
        assert!(Url::new("ws://example.com/socket".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(Url::new(String::new()).is_err());
    }

    #[test]
    fn rejects_invalid_url() {
        assert!(Url::new("not-a-url".into()).is_err());
    }

    #[test]
    fn rejects_disallowed_scheme() {
        assert!(Url::new("mailto:user@example.com".into()).is_err());
        assert!(Url::new("file:///etc/passwd".into()).is_err());
    }

    #[test]
    fn rejects_no_host() {
        assert!(Url::new("https://".into()).is_err());
    }

    #[test]
    fn host_strips_port() {
        let url = Url::new("https://example.com:8080/path".into()).unwrap();
        assert_eq!(url.host(), "example.com");
    }

    #[test]
    fn try_from_str() {
        let url: Url = "https://example.com".try_into().unwrap();
        assert_eq!(url.scheme(), "https");
    }
}
