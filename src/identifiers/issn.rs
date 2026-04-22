use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Issn`].
pub type IssnInput = String;

/// Output type for [`Issn`] — canonical `XXXX-XXXX` form.
pub type IssnOutput = String;

/// A validated ISSN (International Standard Serial Number).
///
/// Spaces and hyphens are stripped on construction. The check character
/// (last position) may be `X` (representing 10) and is uppercased.
/// Output is formatted as `XXXX-XXXX`.
///
/// Validated using the ISSN weighted sum: first 7 characters multiplied
/// by weights 8 down to 2; total mod 11 must be 0 (`X` = 10).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Issn;
/// use arvo::traits::ValueObject;
///
/// let issn = Issn::new("0317-8471".into()).unwrap();
/// assert_eq!(issn.value(), "0317-8471");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Issn(String);

impl ValueObject for Issn {
    type Input = IssnInput;
    type Output = IssnOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let stripped: String = value
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == 'X' || *c == 'x')
            .map(|c| c.to_ascii_uppercase())
            .collect();

        if stripped.len() != 8 {
            return Err(ValidationError::invalid("Issn", value.trim()));
        }

        let first7 = &stripped[..7];
        let check_char = stripped.as_bytes()[7];

        if !first7.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::invalid("Issn", &stripped));
        }
        if !check_char.is_ascii_digit() && check_char != b'X' {
            return Err(ValidationError::invalid("Issn", &stripped));
        }

        let check_value: u32 = if check_char == b'X' {
            10
        } else {
            (check_char - b'0') as u32
        };

        let sum: u32 = first7
            .chars()
            .enumerate()
            .map(|(i, c)| (8 - i as u32) * (c as u8 - b'0') as u32)
            .sum::<u32>()
            + check_value;

        if sum % 11 != 0 {
            return Err(ValidationError::invalid("Issn", &stripped));
        }

        let canonical = format!("{}-{}", &stripped[..4], &stripped[4..]);
        Ok(Self(canonical))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}


impl TryFrom<String> for Issn {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Issn> for String {
    fn from(v: Issn) -> String {
        v.0
    }
}
impl TryFrom<&str> for Issn {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Issn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_issn_with_hyphen() {
        let i = Issn::new("0317-8471".into()).unwrap();
        assert_eq!(i.value(), "0317-8471");
    }

    #[test]
    fn accepts_bare_digits() {
        let i = Issn::new("03178471".into()).unwrap();
        assert_eq!(i.value(), "0317-8471");
    }

    #[test]
    fn formats_output_with_hyphen() {
        let i = Issn::new("03178471".into()).unwrap();
        assert_eq!(i.value(), "0317-8471");
    }

    #[test]
    fn accepts_x_check_digit() {
        let i = Issn::new("0000-006X".into()).unwrap();
        assert_eq!(i.value(), "0000-006X");
    }

    #[test]
    fn normalises_lowercase_x() {
        let i = Issn::new("0000006x".into()).unwrap();
        assert_eq!(i.value(), "0000-006X");
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Issn::new("031784".into()).is_err());
    }

    #[test]
    fn rejects_invalid_checksum() {
        assert!(Issn::new("0317-8470".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let i: Issn = "0317-8471".try_into().unwrap();
        assert_eq!(i.value(), "0317-8471");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Issn::try_from("0317-8471").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Issn = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Issn, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
