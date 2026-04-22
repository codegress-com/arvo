use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`Locale`].
pub type LocaleInput = String;

/// A BCP 47 language tag (e.g. `"en-US"`, `"cs-CZ"`, `"fr"`).
///
/// Accepts both `-` and `_` as separators. On construction, the language
/// subtag is lowercased, the region subtag (if present) is uppercased, and
/// the separator is normalised to `-`.
///
/// MVP scope: language subtag (2–3 letters) plus optional region subtag
/// (2 letters or 3 digits). Script, variant, and extension subtags are
/// out of scope.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::primitives::Locale;
/// use arvo::traits::ValueObject;
///
/// let locale = Locale::new("en_us".into()).unwrap();
/// assert_eq!(locale.value(), "en-US");
///
/// assert!(Locale::new("x".into()).is_err()); // language subtag too short
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Locale(String);

impl ValueObject for Locale {
    type Input = LocaleInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(ValidationError::empty("Locale"));
        }

        let normalised = trimmed.replace('_', "-");
        let parts: Vec<&str> = normalised.splitn(2, '-').collect();

        let lang = parts[0];
        if lang.len() < 2 || lang.len() > 3 || !lang.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ValidationError::invalid("Locale", &trimmed));
        }
        let lang = lang.to_lowercase();

        let canonical = if parts.len() == 2 {
            let region = parts[1];
            let valid_region = (region.len() == 2
                && region.chars().all(|c| c.is_ascii_alphabetic()))
                || (region.len() == 3 && region.chars().all(|c| c.is_ascii_digit()));
            if !valid_region {
                return Err(ValidationError::invalid("Locale", &trimmed));
            }
            format!("{}-{}", lang, region.to_uppercase())
        } else {
            lang
        };

        Ok(Self(canonical))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for Locale {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl Locale {
    /// Returns the language subtag, e.g. `"en"` from `"en-US"`.
    pub fn language(&self) -> &str {
        self.0.split('-').next().unwrap_or(&self.0)
    }

    /// Returns the region subtag if present, e.g. `Some("US")` from `"en-US"`.
    pub fn region(&self) -> Option<&str> {
        let mut parts = self.0.splitn(2, '-');
        parts.next();
        parts.next()
    }
}

impl TryFrom<String> for Locale {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Locale> for String {
    fn from(v: Locale) -> String {
        v.0
    }
}
impl TryFrom<&str> for Locale {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_language_only() {
        let l = Locale::new("en".into()).unwrap();
        assert_eq!(l.value(), "en");
    }

    #[test]
    fn accepts_language_region_with_dash() {
        let l = Locale::new("en-US".into()).unwrap();
        assert_eq!(l.value(), "en-US");
    }

    #[test]
    fn normalises_underscore_separator() {
        let l = Locale::new("en_us".into()).unwrap();
        assert_eq!(l.value(), "en-US");
    }

    #[test]
    fn lowercases_language_subtag() {
        let l = Locale::new("EN-US".into()).unwrap();
        assert_eq!(l.value(), "en-US");
    }

    #[test]
    fn accepts_three_letter_language() {
        let l = Locale::new("ces".into()).unwrap();
        assert_eq!(l.value(), "ces");
    }

    #[test]
    fn accepts_numeric_region() {
        let l = Locale::new("es-419".into()).unwrap();
        assert_eq!(l.value(), "es-419");
    }

    #[test]
    fn rejects_too_short_language() {
        assert!(Locale::new("e".into()).is_err());
    }

    #[test]
    fn rejects_too_long_language() {
        assert!(Locale::new("engl".into()).is_err());
    }

    #[test]
    fn rejects_invalid_region() {
        assert!(Locale::new("en-X1".into()).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(Locale::new(String::new()).is_err());
    }

    #[test]
    fn language_subtag() {
        let l = Locale::new("en-US".into()).unwrap();
        assert_eq!(l.language(), "en");
    }

    #[test]
    fn language_only_locale() {
        let l = Locale::new("fr".into()).unwrap();
        assert_eq!(l.language(), "fr");
        assert_eq!(l.region(), None);
    }

    #[test]
    fn region_subtag() {
        let l = Locale::new("cs-CZ".into()).unwrap();
        assert_eq!(l.region(), Some("CZ"));
    }

    #[test]
    fn try_from_str() {
        let l: Locale = "cs-CZ".try_into().unwrap();
        assert_eq!(l.value(), "cs-CZ");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Locale::try_from("en-US").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Locale = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Locale, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
