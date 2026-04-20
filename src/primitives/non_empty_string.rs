use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`NonEmptyString`].
pub type NonEmptyStringInput = String;

/// Output type for [`NonEmptyString`].
pub type NonEmptyStringOutput = String;

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
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NonEmptyString(String);

impl ValueObject for NonEmptyString {
    type Input = NonEmptyStringInput;
    type Output = NonEmptyStringOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(ValidationError::empty("NonEmptyString"));
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
}
