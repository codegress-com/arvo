use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Ean8`].
pub type Ean8Input = String;

/// Output type for [`Ean8`] — 8 bare digits.
pub type Ean8Output = String;

/// A validated EAN-8 barcode number.
///
/// Spaces and hyphens are stripped on construction. The 8th digit is the
/// check digit, validated using the same algorithm as EAN-13 applied to
/// 8 digits (alternating weights 1 and 3, total mod 10 == 0).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Ean8;
/// use arvo::traits::ValueObject;
///
/// let ean = Ean8::new("73513537".into()).unwrap();
/// assert_eq!(ean.value(), "73513537");
/// assert_eq!(ean.check_digit(), 7);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Ean8(String);

fn ean_checksum_valid(digits: &[u8], expected_len: usize) -> bool {
    if digits.len() != expected_len {
        return false;
    }
    let n = digits.len();
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            // Weight depends on distance from right: even distance → 3, odd → 1.
            let weight = if (n - i) % 2 == 0 { 3u32 } else { 1u32 };
            weight * d as u32
        })
        .sum();
    sum % 10 == 0
}

impl ValueObject for Ean8 {
    type Input = Ean8Input;
    type Output = Ean8Output;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let stripped: String = value.chars().filter(|c| c.is_ascii_digit()).collect();

        if stripped.len() != 8 {
            return Err(ValidationError::invalid("Ean8", value.trim()));
        }

        let digits: Vec<u8> = stripped.chars().map(|c| c as u8 - b'0').collect();

        if !ean_checksum_valid(&digits, 8) {
            return Err(ValidationError::invalid("Ean8", &stripped));
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

impl Ean8 {
    /// Returns the check digit (last digit).
    pub fn check_digit(&self) -> u8 {
        self.0.as_bytes().last().map(|b| b - b'0').unwrap_or(0)
    }
}


impl TryFrom<String> for Ean8 {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Ean8> for String {
    fn from(v: Ean8) -> String {
        v.0
    }
}
impl TryFrom<&str> for Ean8 {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Ean8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_ean8() {
        let e = Ean8::new("73513537".into()).unwrap();
        assert_eq!(e.value(), "73513537");
    }

    #[test]
    fn strips_spaces_and_hyphens() {
        let e = Ean8::new("7351-3537".into()).unwrap();
        assert_eq!(e.value(), "73513537");
    }

    #[test]
    fn check_digit_returns_last_digit() {
        let e = Ean8::new("73513537".into()).unwrap();
        assert_eq!(e.check_digit(), 7);
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Ean8::new("1234".into()).is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(Ean8::new("73513530".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let e: Ean8 = "73513537".try_into().unwrap();
        assert_eq!(e.value(), "73513537");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Ean8::try_from("96385074").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Ean8 = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Ean8, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
