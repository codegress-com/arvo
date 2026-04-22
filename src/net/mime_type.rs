use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`MimeType`].
pub type MimeTypeInput = String;

/// Output type for [`MimeType`].
pub type MimeTypeOutput = String;

/// A validated MIME type (e.g. `"image/png"`, `"application/json"`).
///
/// **Normalisation:** trimmed, lowercased.
/// **Validation:** `type/subtype` format; type and subtype consist of
/// ASCII alphanumeric characters, hyphens, dots, or plus signs.
/// Parameters (e.g. `; charset=utf-8`) are accepted and preserved.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::MimeType;
/// use arvo::traits::ValueObject;
///
/// let mime = MimeType::new("image/png".into())?;
/// assert_eq!(mime.value(), "image/png");
/// assert_eq!(mime.type_part(), "image");
/// assert_eq!(mime.subtype(), "png");
///
/// let mime: MimeType = "application/json".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct MimeType(String);

impl ValueObject for MimeType {
    type Input = MimeTypeInput;
    type Output = MimeTypeOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised = value.trim().to_lowercase();

        if normalised.is_empty() {
            return Err(ValidationError::empty("MimeType"));
        }

        if !is_valid_mime(&normalised) {
            return Err(ValidationError::invalid("MimeType", &normalised));
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

fn is_valid_mime(s: &str) -> bool {
    // Split off optional parameters (; charset=utf-8)
    let base = s.split(';').next().unwrap_or("").trim();

    let Some(slash) = base.find('/') else {
        return false;
    };

    let type_part = &base[..slash];
    let subtype = &base[slash + 1..];

    if type_part.is_empty() || subtype.is_empty() {
        return false;
    }

    let is_token_char = |c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '+' | '_');

    type_part.chars().all(is_token_char) && subtype.chars().all(is_token_char)
}

impl MimeType {
    /// Returns the type part, e.g. `"image"`.
    pub fn type_part(&self) -> &str {
        self.0.split('/').next().unwrap_or("")
    }

    /// Returns the subtype part, e.g. `"png"` (without parameters).
    pub fn subtype(&self) -> &str {
        let after_slash = self.0.split('/').nth(1).unwrap_or("");
        after_slash.split(';').next().unwrap_or("").trim()
    }
}


impl TryFrom<String> for MimeType {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<MimeType> for String {
    fn from(v: MimeType) -> String {
        v.0
    }
}
impl TryFrom<&str> for MimeType {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for MimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_image_png() {
        let m = MimeType::new("image/png".into()).unwrap();
        assert_eq!(m.value(), "image/png");
    }

    #[test]
    fn normalises_to_lowercase() {
        let m = MimeType::new("Application/JSON".into()).unwrap();
        assert_eq!(m.value(), "application/json");
    }

    #[test]
    fn accepts_with_parameter() {
        assert!(MimeType::new("text/html; charset=utf-8".into()).is_ok());
    }

    #[test]
    fn accepts_vendor_type() {
        assert!(MimeType::new("application/vnd.api+json".into()).is_ok());
    }

    #[test]
    fn type_part_and_subtype() {
        let m = MimeType::new("image/png".into()).unwrap();
        assert_eq!(m.type_part(), "image");
        assert_eq!(m.subtype(), "png");
    }

    #[test]
    fn subtype_without_parameter() {
        let m = MimeType::new("text/html; charset=utf-8".into()).unwrap();
        assert_eq!(m.subtype(), "html");
    }

    #[test]
    fn rejects_empty() {
        assert!(MimeType::new(String::new()).is_err());
    }

    #[test]
    fn rejects_missing_slash() {
        assert!(MimeType::new("imagepng".into()).is_err());
    }

    #[test]
    fn rejects_empty_subtype() {
        assert!(MimeType::new("image/".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let m: MimeType = "text/plain".try_into().unwrap();
        assert_eq!(m.value(), "text/plain");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = MimeType::try_from("image/png").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: MimeType = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<MimeType, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
