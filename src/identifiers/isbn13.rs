use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Isbn13`].
pub type Isbn13Input = String;

/// Output type for [`Isbn13`] — 13 bare digits.
pub type Isbn13Output = String;

/// A validated ISBN-13 number.
///
/// Hyphens and spaces are stripped on construction. Must start with `978`
/// or `979`. Check digit validated using the EAN-13 algorithm (alternating
/// weights 1 and 3, total mod 10 == 0).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Isbn13;
/// use arvo::traits::ValueObject;
///
/// let isbn = Isbn13::new("978-0-306-40615-7".into()).unwrap();
/// assert_eq!(isbn.value(), "9780306406157");
/// assert_eq!(isbn.prefix(), "978");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Isbn13(String);

impl ValueObject for Isbn13 {
    type Input = Isbn13Input;
    type Output = Isbn13Output;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let stripped: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

        if stripped.len() != 13 {
            return Err(ValidationError::invalid("Isbn13", value.trim()));
        }

        if !stripped.starts_with("978") && !stripped.starts_with("979") {
            return Err(ValidationError::invalid("Isbn13", &stripped));
        }

        let digits: Vec<u8> = stripped.chars().map(|c| c as u8 - b'0').collect();
        let sum: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                let weight = if i % 2 == 0 { 1u32 } else { 3u32 };
                weight * d as u32
            })
            .sum();

        if sum % 10 != 0 {
            return Err(ValidationError::invalid("Isbn13", &stripped));
        }

        Ok(Self(stripped))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl Isbn13 {
    /// Returns the GS1 prefix — `"978"` or `"979"`.
    pub fn prefix(&self) -> &str {
        &self.0[..3]
    }
}


impl TryFrom<String> for Isbn13 {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Isbn13> for String {
    fn from(v: Isbn13) -> String {
        v.0
    }
}
impl TryFrom<&str> for Isbn13 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Isbn13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_isbn13_with_hyphens() {
        let i = Isbn13::new("978-0-306-40615-7".into()).unwrap();
        assert_eq!(i.value(), "9780306406157");
    }

    #[test]
    fn accepts_bare_digits() {
        let i = Isbn13::new("9780306406157".into()).unwrap();
        assert_eq!(i.value(), "9780306406157");
    }

    #[test]
    fn prefix_978() {
        let i = Isbn13::new("9780306406157".into()).unwrap();
        assert_eq!(i.prefix(), "978");
    }

    #[test]
    fn prefix_979() {
        let i = Isbn13::new("9791032309056".into()).unwrap();
        assert_eq!(i.prefix(), "979");
    }

    #[test]
    fn rejects_wrong_prefix() {
        assert!(Isbn13::new("1234567890123".into()).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Isbn13::new("978030640615".into()).is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(Isbn13::new("9780306406150".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let i: Isbn13 = "9780306406157".try_into().unwrap();
        assert_eq!(i.value(), "9780306406157");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Isbn13::try_from("9780306406157").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Isbn13 = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Isbn13, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
