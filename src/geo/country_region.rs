use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`CountryRegion`].
pub type CountryRegionInput = String;

/// Output type for [`CountryRegion`].

/// A validated ISO 3166-2 subdivision code.
///
/// **Format:** two uppercase ASCII letters (country code), a hyphen, then
/// one to eight uppercase ASCII alphanumeric characters (subdivision code).
/// Example: `"CZ-PR"`, `"US-CA"`, `"GB-ENG"`.
///
/// **Normalisation:** trimmed and uppercased on construction.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::geo::CountryRegion;
/// use arvo::traits::ValueObject;
///
/// let region = CountryRegion::new("cz-pr".into())?;
/// assert_eq!(region.value(), "CZ-PR");
///
/// let region: CountryRegion = "US-CA".try_into()?;
/// assert_eq!(region.country_code(), "US");
/// assert_eq!(region.subdivision_code(), "CA");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct CountryRegion(String);

impl ValueObject for CountryRegion {
    type Input = CountryRegionInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let upper = value.trim().to_uppercase();

        if upper.is_empty() {
            return Err(ValidationError::empty("CountryRegion"));
        }

        if !is_valid_iso3166_2(&upper) {
            return Err(ValidationError::invalid("CountryRegion", &upper));
        }

        Ok(Self(upper))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for CountryRegion {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

fn is_valid_iso3166_2(s: &str) -> bool {
    let Some(dash) = s.find('-') else {
        return false;
    };

    let country = &s[..dash];
    let subdivision = &s[dash + 1..];

    if country.len() != 2 || !country.chars().all(|c| c.is_ascii_uppercase()) {
        return false;
    }

    let sub_len = subdivision.len();
    if !(1..=8).contains(&sub_len) {
        return false;
    }

    subdivision.chars().all(|c| c.is_ascii_alphanumeric())
}

impl CountryRegion {
    /// Returns the 2-letter country code portion, e.g. `"CZ"`.
    pub fn country_code(&self) -> &str {
        self.0.split('-').next().unwrap_or("")
    }

    /// Returns the subdivision code portion, e.g. `"PR"`.
    pub fn subdivision_code(&self) -> &str {
        self.0.split('-').nth(1).unwrap_or("")
    }
}

impl TryFrom<String> for CountryRegion {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<CountryRegion> for String {
    fn from(v: CountryRegion) -> String {
        v.0
    }
}
impl TryFrom<&str> for CountryRegion {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for CountryRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_code() {
        let r = CountryRegion::new("CZ-PR".into()).unwrap();
        assert_eq!(r.value(), "CZ-PR");
    }

    #[test]
    fn normalises_to_uppercase() {
        let r = CountryRegion::new("cz-pr".into()).unwrap();
        assert_eq!(r.value(), "CZ-PR");
    }

    #[test]
    fn accepts_longer_subdivision() {
        assert!(CountryRegion::new("GB-ENG".into()).is_ok());
    }

    #[test]
    fn accepts_numeric_subdivision() {
        assert!(CountryRegion::new("CN-11".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(CountryRegion::new(String::new()).is_err());
    }

    #[test]
    fn rejects_missing_dash() {
        assert!(CountryRegion::new("CZPR".into()).is_err());
    }

    #[test]
    fn rejects_three_letter_country() {
        assert!(CountryRegion::new("CZE-PR".into()).is_err());
    }

    #[test]
    fn rejects_empty_subdivision() {
        assert!(CountryRegion::new("CZ-".into()).is_err());
    }

    #[test]
    fn rejects_subdivision_too_long() {
        assert!(CountryRegion::new("CZ-TOOLONGCODE".into()).is_err());
    }

    #[test]
    fn accessors() {
        let r = CountryRegion::new("US-CA".into()).unwrap();
        assert_eq!(r.country_code(), "US");
        assert_eq!(r.subdivision_code(), "CA");
    }

    #[test]
    fn try_from_str() {
        let r: CountryRegion = "DE-BY".try_into().unwrap();
        assert_eq!(r.value(), "DE-BY");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = CountryRegion::try_from("CZ-PR").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: CountryRegion = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<CountryRegion, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
