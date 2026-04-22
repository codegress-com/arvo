use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Bic`].
pub type BicInput = String;

/// Output type for [`Bic`] — canonical uppercase string.
pub type BicOutput = String;

/// A validated BIC (Bank Identifier Code), also known as SWIFT code.
///
/// On construction the input is trimmed and uppercased. A BIC is either 8 or
/// 11 alphanumeric characters with the following structure:
/// - positions 1–4: bank code (4 letters)
/// - positions 5–6: country code (2 letters)
/// - positions 7–8: location code (2 alphanumeric characters)
/// - positions 9–11: optional branch code (3 alphanumeric characters)
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::Bic;
/// use arvo::traits::ValueObject;
///
/// let bic = Bic::new("DEUTDEDB".into()).unwrap();
/// assert_eq!(bic.value(), "DEUTDEDB");
/// assert_eq!(bic.bank_code(), "DEUT");
/// assert_eq!(bic.country_code(), "DE");
/// assert_eq!(bic.branch_code(), None);
///
/// let bic11 = Bic::new("DEUTDEDBBER".into()).unwrap();
/// assert_eq!(bic11.branch_code(), Some("BER"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Bic(String);

impl ValueObject for Bic {
    type Input = BicInput;
    type Output = BicOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let upper = value.trim().to_uppercase();

        if upper.is_empty() {
            return Err(ValidationError::empty("Bic"));
        }

        let len = upper.len();
        if len != 8 && len != 11 {
            return Err(ValidationError::invalid("Bic", &upper));
        }

        if !upper.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(ValidationError::invalid("Bic", &upper));
        }

        // Positions 0–3: bank code — 4 letters
        if !upper[..4].chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ValidationError::invalid("Bic", &upper));
        }

        // Positions 4–5: country code — 2 letters
        if !upper[4..6].chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ValidationError::invalid("Bic", &upper));
        }

        // Positions 6–7: location code — already validated as alphanumeric above

        Ok(Self(upper))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl Bic {
    /// Returns the 4-letter bank code (positions 1–4).
    pub fn bank_code(&self) -> &str {
        &self.0[..4]
    }

    /// Returns the 2-letter country code (positions 5–6).
    pub fn country_code(&self) -> &str {
        &self.0[4..6]
    }

    /// Returns the 2-character location code (positions 7–8).
    pub fn location_code(&self) -> &str {
        &self.0[6..8]
    }

    /// Returns the 3-character branch code (positions 9–11) if present.
    pub fn branch_code(&self) -> Option<&str> {
        if self.0.len() == 11 {
            Some(&self.0[8..])
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Bic {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Bic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_8_char_bic() {
        let b = Bic::new("DEUTDEDB".into()).unwrap();
        assert_eq!(b.value(), "DEUTDEDB");
    }

    #[test]
    fn accepts_11_char_bic() {
        let b = Bic::new("DEUTDEDBBER".into()).unwrap();
        assert_eq!(b.value(), "DEUTDEDBBER");
    }

    #[test]
    fn normalises_to_uppercase() {
        let b = Bic::new("deutdedb".into()).unwrap();
        assert_eq!(b.value(), "DEUTDEDB");
    }

    #[test]
    fn trims_whitespace() {
        let b = Bic::new("  DEUTDEDB  ".into()).unwrap();
        assert_eq!(b.value(), "DEUTDEDB");
    }

    #[test]
    fn bank_code_accessor() {
        let b = Bic::new("DEUTDEDB".into()).unwrap();
        assert_eq!(b.bank_code(), "DEUT");
    }

    #[test]
    fn country_code_accessor() {
        let b = Bic::new("DEUTDEDB".into()).unwrap();
        assert_eq!(b.country_code(), "DE");
    }

    #[test]
    fn location_code_accessor() {
        let b = Bic::new("DEUTDEDB".into()).unwrap();
        assert_eq!(b.location_code(), "DB");
    }

    #[test]
    fn branch_code_none_for_8_char() {
        let b = Bic::new("DEUTDEDB".into()).unwrap();
        assert_eq!(b.branch_code(), None);
    }

    #[test]
    fn branch_code_some_for_11_char() {
        let b = Bic::new("DEUTDEDBBER".into()).unwrap();
        assert_eq!(b.branch_code(), Some("BER"));
    }

    #[test]
    fn rejects_empty() {
        assert!(Bic::new(String::new()).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Bic::new("DEUTDED".into()).is_err());
        assert!(Bic::new("DEUTDEDB1".into()).is_err());
    }

    #[test]
    fn rejects_digits_in_bank_code() {
        assert!(Bic::new("1EUTDEDB".into()).is_err());
    }

    #[test]
    fn rejects_digits_in_country_code() {
        assert!(Bic::new("DEUT1EDB".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let b: Bic = "DEUTDEDB".try_into().unwrap();
        assert_eq!(b.value(), "DEUTDEDB");
    }
}
