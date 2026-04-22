use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`HexColor`].
pub type HexColorInput = String;

/// Output type for [`HexColor`] — always a 7-character `#RRGGBB` string.
pub type HexColorOutput = String;

/// A CSS hex color in canonical `#RRGGBB` form, normalised to uppercase.
///
/// Accepts both 6-digit (`#FF0000`) and 3-digit shorthand (`#F00`) input.
/// The 3-digit form is expanded by doubling each channel digit.
/// The `#` prefix is required.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::HexColor;
/// use arvo::traits::ValueObject;
///
/// let red = HexColor::new("#f00".into()).unwrap();
/// assert_eq!(red.value(), "#FF0000");
/// assert_eq!(red.r(), 255);
///
/// assert!(HexColor::new("FF0000".into()).is_err()); // missing #
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct HexColor(String);

impl ValueObject for HexColor {
    type Input = HexColorInput;
    type Output = HexColorOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let s = value.trim().to_uppercase();

        if s.is_empty() {
            return Err(ValidationError::empty("HexColor"));
        }

        let hex = s
            .strip_prefix('#')
            .ok_or_else(|| ValidationError::invalid("HexColor", &s))?;

        let expanded = match hex.len() {
            3 => {
                if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(ValidationError::invalid("HexColor", &s));
                }
                let chars: Vec<char> = hex.chars().collect();
                format!("#{0}{0}{1}{1}{2}{2}", chars[0], chars[1], chars[2])
            }
            6 => {
                if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(ValidationError::invalid("HexColor", &s));
                }
                s
            }
            _ => return Err(ValidationError::invalid("HexColor", &s)),
        };

        Ok(Self(expanded))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl HexColor {
    fn channel(s: &str, offset: usize) -> u8 {
        u8::from_str_radix(&s[offset..offset + 2], 16).unwrap_or(0)
    }

    /// Red channel value (0–255).
    pub fn r(&self) -> u8 {
        Self::channel(&self.0, 1)
    }

    /// Green channel value (0–255).
    pub fn g(&self) -> u8 {
        Self::channel(&self.0, 3)
    }

    /// Blue channel value (0–255).
    pub fn b(&self) -> u8 {
        Self::channel(&self.0, 5)
    }

    /// Returns the RGB channels as a tuple `(r, g, b)`.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }
}


impl TryFrom<String> for HexColor {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<HexColor> for String {
    fn from(v: HexColor) -> String {
        v.0
    }
}
impl TryFrom<&str> for HexColor {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_six_digit_form() {
        let c = HexColor::new("#FF0000".into()).unwrap();
        assert_eq!(c.value(), "#FF0000");
    }

    #[test]
    fn normalises_to_uppercase() {
        let c = HexColor::new("#ff0000".into()).unwrap();
        assert_eq!(c.value(), "#FF0000");
    }

    #[test]
    fn expands_three_digit_shorthand() {
        let c = HexColor::new("#F0A".into()).unwrap();
        assert_eq!(c.value(), "#FF00AA");
    }

    #[test]
    fn expands_three_digit_lowercase() {
        let c = HexColor::new("#f0a".into()).unwrap();
        assert_eq!(c.value(), "#FF00AA");
    }

    #[test]
    fn rejects_missing_hash() {
        assert!(HexColor::new("FF0000".into()).is_err());
    }

    #[test]
    fn rejects_invalid_chars() {
        assert!(HexColor::new("#GGGGGG".into()).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(HexColor::new("#FFFF".into()).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(HexColor::new(String::new()).is_err());
    }

    #[test]
    fn rgb_channels() {
        let c = HexColor::new("#1A2B3C".into()).unwrap();
        assert_eq!(c.r(), 0x1A);
        assert_eq!(c.g(), 0x2B);
        assert_eq!(c.b(), 0x3C);
    }

    #[test]
    fn try_from_str() {
        let c: HexColor = "#ABC".try_into().unwrap();
        assert_eq!(c.value(), "#AABBCC");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = HexColor::try_from("#ff0000").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: HexColor = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<HexColor, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
