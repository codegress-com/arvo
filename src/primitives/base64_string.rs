use crate::errors::ValidationError;
use crate::traits::ValueObject;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

/// Input type for [`Base64String`].
pub type Base64StringInput = String;

/// Output type for [`Base64String`].
pub type Base64StringOutput = String;

/// A validated standard Base64-encoded string.
///
/// Accepts the standard alphabet (`A–Z`, `a–z`, `0–9`, `+`, `/`) with `=`
/// padding. Surrounding whitespace is trimmed. The length after trimming must
/// be a multiple of 4.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::Base64String;
/// use arvo::traits::ValueObject;
///
/// let b = Base64String::new("aGVsbG8=".into()).unwrap();
/// assert_eq!(b.decode(), b"hello");
///
/// assert!(Base64String::new("not!!base64".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Base64String(String);

impl ValueObject for Base64String {
    type Input = Base64StringInput;
    type Output = Base64StringOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(ValidationError::empty("Base64String"));
        }
        STANDARD
            .decode(&trimmed)
            .map_err(|_| ValidationError::invalid("Base64String", &trimmed))?;
        Ok(Self(trimmed))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl Base64String {
    /// Decodes the Base64 string and returns the raw bytes.
    pub fn decode(&self) -> Vec<u8> {
        STANDARD.decode(&self.0).expect("already validated")
    }
}

impl TryFrom<&str> for Base64String {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Base64String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_base64() {
        let b = Base64String::new("aGVsbG8=".into()).unwrap();
        assert_eq!(b.value(), "aGVsbG8=");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let b = Base64String::new("  aGVsbG8=  ".into()).unwrap();
        assert_eq!(b.value(), "aGVsbG8=");
    }

    #[test]
    fn decode_returns_raw_bytes() {
        let b = Base64String::new("aGVsbG8=".into()).unwrap();
        assert_eq!(b.decode(), b"hello");
    }

    #[test]
    fn rejects_invalid_chars() {
        assert!(Base64String::new("not!!valid".into()).is_err());
    }

    #[test]
    fn rejects_wrong_padding() {
        assert!(Base64String::new("aGVsbG8".into()).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(Base64String::new(String::new()).is_err());
    }

    #[test]
    fn try_from_str() {
        let b: Base64String = "aGVsbG8=".try_into().unwrap();
        assert_eq!(b.decode(), b"hello");
    }
}
