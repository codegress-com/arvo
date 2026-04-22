use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Iban`].
pub type IbanInput = String;

/// Output type for [`Iban`] — canonical uppercase string without spaces.
pub type IbanOutput = String;

/// A validated IBAN (International Bank Account Number).
///
/// On construction all spaces are stripped and the value is uppercased. The
/// mod-97 algorithm is used to validate the check digits: the first four
/// characters are moved to the end, each letter is replaced by its numeric
/// value (`A=10` … `Z=35`), and the resulting number mod 97 must equal 1.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::Iban;
/// use arvo::traits::ValueObject;
///
/// let iban = Iban::new("GB82 WEST 1234 5698 7654 32".into()).unwrap();
/// assert_eq!(iban.value(), "GB82WEST12345698765432");
/// assert_eq!(iban.country_code(), "GB");
/// assert_eq!(iban.check_digits(), "82");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Iban(String);

impl ValueObject for Iban {
    type Input = IbanInput;
    type Output = IbanOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let stripped: String = value
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_uppercase())
            .collect();

        if stripped.is_empty() {
            return Err(ValidationError::empty("Iban"));
        }

        let len = stripped.len();
        if !(15..=34).contains(&len) {
            return Err(ValidationError::invalid("Iban", &stripped));
        }

        let bytes = stripped.as_bytes();
        if !bytes[0].is_ascii_alphabetic() || !bytes[1].is_ascii_alphabetic() {
            return Err(ValidationError::invalid("Iban", &stripped));
        }
        if !bytes[2].is_ascii_digit() || !bytes[3].is_ascii_digit() {
            return Err(ValidationError::invalid("Iban", &stripped));
        }
        if !stripped[4..].chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(ValidationError::invalid("Iban", &stripped));
        }

        if iban_mod97(&stripped) != 1 {
            return Err(ValidationError::invalid("Iban", &stripped));
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

impl Iban {
    /// Returns the 2-letter country code, e.g. `"GB"`.
    pub fn country_code(&self) -> &str {
        &self.0[..2]
    }

    /// Returns the 2-digit check digits, e.g. `"82"`.
    pub fn check_digits(&self) -> &str {
        &self.0[2..4]
    }

    /// Returns the Basic Bank Account Number (BBAN), characters 5 onwards.
    pub fn bban(&self) -> &str {
        &self.0[4..]
    }
}

impl TryFrom<&str> for Iban {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Iban {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn iban_mod97(iban: &str) -> u64 {
    // Rearrange: move first 4 characters to the end.
    let rearranged: String = iban[4..].chars().chain(iban[..4].chars()).collect();

    let mut remainder: u64 = 0;
    for c in rearranged.chars() {
        if c.is_ascii_digit() {
            remainder = (remainder * 10 + (c as u64 - b'0' as u64)) % 97;
        } else {
            // Letter: A=10, B=11, ..., Z=35 (always 2 digits)
            let val = c as u64 - b'A' as u64 + 10;
            remainder = (remainder * 100 + val) % 97;
        }
    }
    remainder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_gb_iban() {
        let i = Iban::new("GB82WEST12345698765432".into()).unwrap();
        assert_eq!(i.value(), "GB82WEST12345698765432");
    }

    #[test]
    fn strips_spaces() {
        let i = Iban::new("GB82 WEST 1234 5698 7654 32".into()).unwrap();
        assert_eq!(i.value(), "GB82WEST12345698765432");
    }

    #[test]
    fn normalises_to_uppercase() {
        let i = Iban::new("gb82west12345698765432".into()).unwrap();
        assert_eq!(i.value(), "GB82WEST12345698765432");
    }

    #[test]
    fn country_code_accessor() {
        let i = Iban::new("GB82WEST12345698765432".into()).unwrap();
        assert_eq!(i.country_code(), "GB");
    }

    #[test]
    fn check_digits_accessor() {
        let i = Iban::new("GB82WEST12345698765432".into()).unwrap();
        assert_eq!(i.check_digits(), "82");
    }

    #[test]
    fn bban_accessor() {
        let i = Iban::new("GB82WEST12345698765432".into()).unwrap();
        assert_eq!(i.bban(), "WEST12345698765432");
    }

    #[test]
    fn accepts_german_iban() {
        assert!(Iban::new("DE89370400440532013000".into()).is_ok());
    }

    #[test]
    fn accepts_czech_iban() {
        assert!(Iban::new("CZ6508000000192000145399".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(Iban::new(String::new()).is_err());
    }

    #[test]
    fn rejects_too_short() {
        assert!(Iban::new("GB82WEST123".into()).is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(Iban::new("GB83WEST12345698765432".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let i: Iban = "GB82WEST12345698765432".try_into().unwrap();
        assert_eq!(i.value(), "GB82WEST12345698765432");
    }
}
