use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Isbn10`].
pub type Isbn10Input = String;

/// Output type for [`Isbn10`] — 10 characters (9 digits + check char `0–9` or `X`).
pub type Isbn10Output = String;

/// A validated ISBN-10 number.
///
/// Hyphens and spaces are stripped on construction. The check character
/// (last position) may be `X` (representing 10) and is uppercased.
/// Validated using the ISBN-10 weighted sum: positions 1–9 multiplied by
/// weights 10 down to 2, plus the check value; total mod 11 must be 0.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Isbn10;
/// use arvo::traits::ValueObject;
///
/// let isbn = Isbn10::new("0-306-40615-2".into()).unwrap();
/// assert_eq!(isbn.value(), "0306406152");
///
/// let isbn_x = Isbn10::new("0-19-852663-6".into()).unwrap();
/// assert_eq!(isbn_x.value(), "0198526636");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Isbn10(String);

impl ValueObject for Isbn10 {
    type Input = Isbn10Input;
    type Output = Isbn10Output;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let stripped: String = value
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == 'X' || *c == 'x')
            .map(|c| c.to_ascii_uppercase())
            .collect();

        if stripped.len() != 10 {
            return Err(ValidationError::invalid("Isbn10", value.trim()));
        }

        // First 9 must be digits; last may be digit or X
        let first9 = &stripped[..9];
        let check_char = stripped.as_bytes()[9];

        if !first9.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::invalid("Isbn10", &stripped));
        }
        if !check_char.is_ascii_digit() && check_char != b'X' {
            return Err(ValidationError::invalid("Isbn10", &stripped));
        }

        let check_value: u32 = if check_char == b'X' {
            10
        } else {
            (check_char - b'0') as u32
        };

        let sum: u32 = first9
            .chars()
            .enumerate()
            .map(|(i, c)| (10 - i as u32) * (c as u8 - b'0') as u32)
            .sum::<u32>()
            + check_value;

        if sum % 11 != 0 {
            return Err(ValidationError::invalid("Isbn10", &stripped));
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


impl TryFrom<String> for Isbn10 {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Isbn10> for String {
    fn from(v: Isbn10) -> String {
        v.0
    }
}
impl TryFrom<&str> for Isbn10 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Isbn10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_isbn10_with_hyphens() {
        let i = Isbn10::new("0-306-40615-2".into()).unwrap();
        assert_eq!(i.value(), "0306406152");
    }

    #[test]
    fn accepts_bare_digits() {
        let i = Isbn10::new("0306406152".into()).unwrap();
        assert_eq!(i.value(), "0306406152");
    }

    #[test]
    fn accepts_x_check_digit() {
        let i = Isbn10::new("047191536X".into()).unwrap();
        assert_eq!(i.value(), "047191536X");
    }

    #[test]
    fn normalises_lowercase_x() {
        let i = Isbn10::new("047191536x".into()).unwrap();
        assert_eq!(i.value(), "047191536X");
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Isbn10::new("030640615".into()).is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(Isbn10::new("0306406153".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let i: Isbn10 = "0306406152".try_into().unwrap();
        assert_eq!(i.value(), "0306406152");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Isbn10::try_from("0306406152").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Isbn10 = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Isbn10, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
