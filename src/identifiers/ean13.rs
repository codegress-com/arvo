use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`Ean13`].
pub type Ean13Input = String;

/// Output type for [`Ean13`] — 13 bare digits.

/// A validated EAN-13 barcode number.
///
/// Spaces and hyphens are stripped on construction. The 13th digit is the
/// check digit, validated using the EAN-13 algorithm (alternating weights
/// 1 and 3, total mod 10 == 0).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Ean13;
/// use arvo::traits::ValueObject;
///
/// let ean = Ean13::new("4006381333931".into()).unwrap();
/// assert_eq!(ean.value(), "4006381333931");
/// assert_eq!(ean.check_digit(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Ean13(String);

fn ean_checksum_valid(digits: &[u8], expected_len: usize) -> bool {
    if digits.len() != expected_len {
        return false;
    }
    let n = digits.len();
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            let weight = if (n - i) % 2 == 0 { 3u32 } else { 1u32 };
            weight * d as u32
        })
        .sum();
    sum % 10 == 0
}

impl ValueObject for Ean13 {
    type Input = Ean13Input;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let stripped: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

        if stripped.len() != 13 {
            return Err(ValidationError::invalid("Ean13", value.trim()));
        }

        let digits: Vec<u8> = stripped.chars().map(|c| c as u8 - b'0').collect();

        if !ean_checksum_valid(&digits, 13) {
            return Err(ValidationError::invalid("Ean13", &stripped));
        }

        Ok(Self(stripped))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for Ean13 {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl Ean13 {
    /// Returns the check digit (last digit).
    pub fn check_digit(&self) -> u8 {
        self.0.as_bytes().last().map(|b| b - b'0').unwrap_or(0)
    }
}

impl TryFrom<String> for Ean13 {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Ean13> for String {
    fn from(v: Ean13) -> String {
        v.0
    }
}
impl TryFrom<&str> for Ean13 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Ean13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_ean13() {
        let e = Ean13::new("4006381333931".into()).unwrap();
        assert_eq!(e.value(), "4006381333931");
    }

    #[test]
    fn strips_spaces_and_hyphens() {
        let e = Ean13::new("4006381-333931".into()).unwrap();
        assert_eq!(e.value(), "4006381333931");
    }

    #[test]
    fn check_digit_returns_last_digit() {
        let e = Ean13::new("4006381333931".into()).unwrap();
        assert_eq!(e.check_digit(), 1);
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Ean13::new("12345".into()).is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(Ean13::new("4006381333930".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let e: Ean13 = "4006381333931".try_into().unwrap();
        assert_eq!(e.value(), "4006381333931");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Ean13::try_from("5901234123457").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Ean13 = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Ean13, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
