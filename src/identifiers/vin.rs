use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Vin`].
pub type VinInput = String;

/// Output type for [`Vin`] — 17 uppercase characters.
pub type VinOutput = String;

/// A validated Vehicle Identification Number (VIN) per ISO 3779.
///
/// Trimmed and uppercased on construction. Must be exactly 17 characters
/// from the VIN alphabet (letters and digits; `I`, `O`, `Q` forbidden).
/// The check digit at position 9 (1-indexed) is validated using the
/// standard transliteration table and positional weights.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Vin;
/// use arvo::traits::ValueObject;
///
/// let vin = Vin::new("1HGBH41JXMN109186".into()).unwrap();
/// assert_eq!(vin.wmi(), "1HG");
/// assert_eq!(vin.vds(), "BH41JX");
/// assert_eq!(vin.vis(), "MN109186");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Vin(String);

fn transliterate(c: char) -> Option<u32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        'E' => Some(5),
        'F' => Some(6),
        'G' => Some(7),
        'H' => Some(8),
        'J' => Some(1),
        'K' => Some(2),
        'L' => Some(3),
        'M' => Some(4),
        'N' => Some(5),
        'P' => Some(7),
        'R' => Some(9),
        'S' => Some(2),
        'T' => Some(3),
        'U' => Some(4),
        'V' => Some(5),
        'W' => Some(6),
        'X' => Some(7),
        'Y' => Some(8),
        'Z' => Some(9),
        _ => None,
    }
}

const WEIGHTS: [u32; 17] = [8, 7, 6, 5, 4, 3, 2, 10, 0, 9, 8, 7, 6, 5, 4, 3, 2];

impl ValueObject for Vin {
    type Input = VinInput;
    type Output = VinOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised = value.trim().to_uppercase();

        if normalised.len() != 17 {
            return Err(ValidationError::invalid("Vin", &normalised));
        }

        // Validate alphabet — I, O, Q forbidden
        for c in normalised.chars() {
            if c == 'I' || c == 'O' || c == 'Q' {
                return Err(ValidationError::invalid("Vin", &normalised));
            }
            if transliterate(c).is_none() {
                return Err(ValidationError::invalid("Vin", &normalised));
            }
        }

        // Compute weighted sum (position 9 weight = 0, excluded from sum)
        let sum: u32 = normalised
            .chars()
            .zip(WEIGHTS.iter())
            .map(|(c, &w)| transliterate(c).unwrap_or(0) * w)
            .sum();

        let remainder = sum % 11;
        let check_char = normalised.as_bytes()[8] as char;
        let expected = if remainder == 10 {
            'X'
        } else {
            char::from_digit(remainder, 10).unwrap()
        };

        if check_char != expected {
            return Err(ValidationError::invalid("Vin", &normalised));
        }

        Ok(Self(normalised))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl Vin {
    /// World Manufacturer Identifier — first 3 characters.
    pub fn wmi(&self) -> &str {
        &self.0[..3]
    }

    /// Vehicle Descriptor Section — characters 4–9 (1-indexed).
    pub fn vds(&self) -> &str {
        &self.0[3..9]
    }

    /// Vehicle Identifier Section — last 8 characters.
    pub fn vis(&self) -> &str {
        &self.0[9..]
    }

    /// Model year character — position 10 (index 9).
    pub fn model_year(&self) -> char {
        self.0.as_bytes()[9] as char
    }
}


impl TryFrom<String> for Vin {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Vin> for String {
    fn from(v: Vin) -> String {
        v.0
    }
}
impl TryFrom<&str> for Vin {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Vin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_VIN: &str = "1HGBH41JXMN109186";

    #[test]
    fn accepts_valid_vin() {
        let v = Vin::new(VALID_VIN.into()).unwrap();
        assert_eq!(v.value(), VALID_VIN);
    }

    #[test]
    fn normalises_to_uppercase() {
        let v = Vin::new("1hgbh41jxmn109186".into()).unwrap();
        assert_eq!(v.value(), VALID_VIN);
    }

    #[test]
    fn wmi_returns_first_3() {
        let v = Vin::new(VALID_VIN.into()).unwrap();
        assert_eq!(v.wmi(), "1HG");
    }

    #[test]
    fn vds_returns_chars_4_to_9() {
        let v = Vin::new(VALID_VIN.into()).unwrap();
        assert_eq!(v.vds(), "BH41JX");
    }

    #[test]
    fn vis_returns_last_8() {
        let v = Vin::new(VALID_VIN.into()).unwrap();
        assert_eq!(v.vis(), "MN109186");
    }

    #[test]
    fn model_year_returns_10th_char() {
        let v = Vin::new(VALID_VIN.into()).unwrap();
        assert_eq!(v.model_year(), 'M');
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(Vin::new("1HGBH41JXMN10918".into()).is_err());
    }

    #[test]
    fn rejects_forbidden_letter_i() {
        assert!(Vin::new("1HGBH41IXMN109186".into()).is_err());
    }

    #[test]
    fn rejects_forbidden_letter_o() {
        assert!(Vin::new("1HGBH41OXMN109186".into()).is_err());
    }

    #[test]
    fn rejects_forbidden_letter_q() {
        assert!(Vin::new("1HGBH41QXMN109186".into()).is_err());
    }

    #[test]
    fn rejects_invalid_check_digit() {
        assert!(Vin::new("1HGBH41JAMN109186".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let v: Vin = VALID_VIN.try_into().unwrap();
        assert_eq!(v.value(), VALID_VIN);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Vin::try_from("1HGBH41JXMN109186").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Vin = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Vin, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
