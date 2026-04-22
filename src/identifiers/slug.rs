use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Slug`].
pub type SlugInput = String;

/// Output type for [`Slug`].
pub type SlugOutput = String;

/// A URL-safe slug: lowercase alphanumeric characters and hyphens only.
///
/// On construction the value is trimmed and lowercased. Consecutive hyphens
/// and leading/trailing hyphens are rejected.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::identifiers::Slug;
/// use arvo::traits::ValueObject;
///
/// let slug = Slug::new("hello-world".into()).unwrap();
/// assert_eq!(slug.value(), "hello-world");
///
/// assert!(Slug::new("-bad".into()).is_err());
/// assert!(Slug::new("has--double".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct Slug(String);

impl ValueObject for Slug {
    type Input = SlugInput;
    type Output = SlugOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised = value.trim().to_lowercase();

        if normalised.is_empty() {
            return Err(ValidationError::empty("Slug"));
        }

        if !normalised
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(ValidationError::invalid("Slug", &normalised));
        }

        if normalised.starts_with('-') || normalised.ends_with('-') {
            return Err(ValidationError::invalid("Slug", &normalised));
        }

        if normalised.contains("--") {
            return Err(ValidationError::invalid("Slug", &normalised));
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


impl TryFrom<String> for Slug {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Slug> for String {
    fn from(v: Slug) -> String {
        v.0
    }
}
impl TryFrom<&str> for Slug {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_slug() {
        let s = Slug::new("hello-world".into()).unwrap();
        assert_eq!(s.value(), "hello-world");
    }

    #[test]
    fn normalises_to_lowercase() {
        let s = Slug::new("Hello-World".into()).unwrap();
        assert_eq!(s.value(), "hello-world");
    }

    #[test]
    fn accepts_digits() {
        let s = Slug::new("post-123".into()).unwrap();
        assert_eq!(s.value(), "post-123");
    }

    #[test]
    fn rejects_empty() {
        assert!(Slug::new(String::new()).is_err());
    }

    #[test]
    fn rejects_leading_hyphen() {
        assert!(Slug::new("-bad".into()).is_err());
    }

    #[test]
    fn rejects_trailing_hyphen() {
        assert!(Slug::new("bad-".into()).is_err());
    }

    #[test]
    fn rejects_double_hyphen() {
        assert!(Slug::new("has--double".into()).is_err());
    }

    #[test]
    fn rejects_special_chars() {
        assert!(Slug::new("has space".into()).is_err());
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let s = Slug::new("  hello  ".into()).unwrap();
        assert_eq!(s.value(), "hello");
    }

    #[test]
    fn try_from_str() {
        let s: Slug = "my-slug".try_into().unwrap();
        assert_eq!(s.value(), "my-slug");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Slug::try_from("hello-world").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Slug = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Slug, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
