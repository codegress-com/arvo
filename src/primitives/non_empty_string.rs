use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`NonEmptyString`].
pub type NonEmptyStringInput = String;

/// Output type for [`NonEmptyString`].

/// A non-empty, trimmed string.
///
/// Surrounding whitespace is stripped on construction. A string that consists
/// entirely of whitespace is rejected.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::NonEmptyString;
/// use arvo::traits::ValueObject;
///
/// let s = NonEmptyString::new("  hello  ".into()).unwrap();
/// assert_eq!(s.value(), "hello");
///
/// assert!(NonEmptyString::new("   ".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct NonEmptyString(String);

impl ValueObject for NonEmptyString {
    type Input = NonEmptyStringInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(ValidationError::empty("NonEmptyString"));
        }
        Ok(Self(trimmed))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for NonEmptyString {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<NonEmptyString> for String {
    fn from(v: NonEmptyString) -> String {
        v.0
    }
}
impl TryFrom<&str> for NonEmptyString {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_string() {
        let s = NonEmptyString::new("hello".into()).unwrap();
        assert_eq!(s.value(), "hello");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let s = NonEmptyString::new("  hello  ".into()).unwrap();
        assert_eq!(s.value(), "hello");
    }

    #[test]
    fn rejects_empty_string() {
        assert!(NonEmptyString::new(String::new()).is_err());
    }

    #[test]
    fn rejects_whitespace_only() {
        assert!(NonEmptyString::new("   ".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let s: NonEmptyString = "world".try_into().unwrap();
        assert_eq!(s.value(), "world");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = NonEmptyString::try_from("hello").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: NonEmptyString = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<NonEmptyString, _> = serde_json::from_str("\"\"");
        assert!(result.is_err());
    }
}
