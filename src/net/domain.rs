use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`Domain`].
pub type DomainInput = String;

/// Output type for [`Domain`].

/// A validated domain name without a scheme (e.g. `"example.com"`).
///
/// **Normalisation:** trimmed, lowercased.
/// **Validation:** labels separated by `.`, each label 1–63 ASCII alphanumeric
/// or hyphen characters, not starting or ending with a hyphen, total length
/// ≤ 253 characters, at least two labels.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::Domain;
/// use arvo::traits::ValueObject;
///
/// let domain = Domain::new("Example.COM".into())?;
/// assert_eq!(domain.value(), "example.com");
///
/// let domain: Domain = "api.example.com".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Domain(String);

impl ValueObject for Domain {
    type Input = DomainInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let normalised = value.trim().to_lowercase();

        if normalised.is_empty() {
            return Err(ValidationError::empty("Domain"));
        }

        if !is_valid_domain(&normalised) {
            return Err(ValidationError::invalid("Domain", &normalised));
        }

        Ok(Self(normalised))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for Domain {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

fn is_valid_domain(s: &str) -> bool {
    if s.len() > 253 {
        return false;
    }

    let labels: Vec<&str> = s.split('.').collect();

    if labels.len() < 2 {
        return false;
    }

    for label in &labels {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if label.starts_with('-') || label.ends_with('-') {
            return false;
        }
        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

impl TryFrom<String> for Domain {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<Domain> for String {
    fn from(v: Domain) -> String {
        v.0
    }
}
impl TryFrom<&str> for Domain {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_domain() {
        let d = Domain::new("example.com".into()).unwrap();
        assert_eq!(d.value(), "example.com");
    }

    #[test]
    fn normalises_to_lowercase() {
        let d = Domain::new("Example.COM".into()).unwrap();
        assert_eq!(d.value(), "example.com");
    }

    #[test]
    fn accepts_subdomain() {
        assert!(Domain::new("api.example.com".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(Domain::new(String::new()).is_err());
    }

    #[test]
    fn rejects_single_label() {
        assert!(Domain::new("localhost".into()).is_err());
    }

    #[test]
    fn rejects_leading_hyphen_in_label() {
        assert!(Domain::new("-example.com".into()).is_err());
    }

    #[test]
    fn rejects_trailing_hyphen_in_label() {
        assert!(Domain::new("example-.com".into()).is_err());
    }

    #[test]
    fn rejects_empty_label() {
        assert!(Domain::new("example..com".into()).is_err());
    }

    #[test]
    fn rejects_url_with_scheme() {
        assert!(Domain::new("https://example.com".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let d: Domain = "example.org".try_into().unwrap();
        assert_eq!(d.value(), "example.org");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Domain::try_from("example.com").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: Domain = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<Domain, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
