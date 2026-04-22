use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`ApiKey`].
pub type ApiKeyInput = String;

/// Output type for [`ApiKey`].
pub type ApiKeyOutput = String;

/// A validated API key — non-empty, trimmed.
///
/// `Display` shows a masked version with only the last 4 characters visible
/// (e.g. `"****abcd"`). `value()` returns the full key.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::ApiKey;
/// use arvo::traits::ValueObject;
///
/// let key = ApiKey::new("sk-1234567890abcd".into())?;
/// assert_eq!(key.value(), "sk-1234567890abcd");
/// assert_eq!(key.to_string(), "************abcd");
/// assert_eq!(key.last_four(), "abcd");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct ApiKey(String);

impl ValueObject for ApiKey {
    type Input = ApiKeyInput;
    type Output = ApiKeyOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("ApiKey"));
        }

        Ok(Self(trimmed))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl ApiKey {
    /// Returns the last 4 characters of the key.
    pub fn last_four(&self) -> &str {
        let len = self.0.len();
        if len <= 4 {
            &self.0
        } else {
            &self.0[len - 4..]
        }
    }

    /// Returns the masked representation: `****` prefix + last 4 chars.
    pub fn masked(&self) -> String {
        let len = self.0.len();
        let mask_len = len.saturating_sub(4);
        format!("{}{}", "*".repeat(mask_len), self.last_four())
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.masked())
    }
}

impl TryFrom<&str> for ApiKey {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_key() {
        let key = ApiKey::new("sk-1234567890abcd".into()).unwrap();
        assert_eq!(key.value(), "sk-1234567890abcd");
    }

    #[test]
    fn trims_whitespace() {
        let key = ApiKey::new("  mykey  ".into()).unwrap();
        assert_eq!(key.value(), "mykey");
    }

    #[test]
    fn rejects_empty() {
        assert!(ApiKey::new(String::new()).is_err());
    }

    #[test]
    fn rejects_whitespace_only() {
        assert!(ApiKey::new("   ".into()).is_err());
    }

    #[test]
    fn last_four() {
        let key = ApiKey::new("sk-1234567890abcd".into()).unwrap();
        assert_eq!(key.last_four(), "abcd");
    }

    #[test]
    fn last_four_short_key() {
        let key = ApiKey::new("abc".into()).unwrap();
        assert_eq!(key.last_four(), "abc");
    }

    #[test]
    fn masked_display() {
        let key = ApiKey::new("sk-1234567890abcd".into()).unwrap();
        // "sk-1234567890abcd" is 18 chars, last 4 = "abcd", masked = 14 stars + "abcd"
        assert_eq!(key.to_string(), "*************abcd");
    }

    #[test]
    fn display_masks_key() {
        let key = ApiKey::new("secret".into()).unwrap();
        let displayed = key.to_string();
        assert!(displayed.ends_with("cret"));
        assert!(displayed.starts_with("**"));
    }
}
