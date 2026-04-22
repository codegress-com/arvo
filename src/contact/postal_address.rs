use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

use super::country_code::CountryCode;

/// Input type for [`PostalAddress`] construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostalAddressInput {
    /// Street name and number, e.g. `"Václavské náměstí 1"`.
    pub street: String,
    /// City or locality name, e.g. `"Prague"`.
    pub city: String,
    /// Postal / ZIP code, e.g. `"110 00"`.
    pub zip: String,
    /// ISO 3166-1 alpha-2 country code.
    pub country: CountryCode,
}

/// Output type for [`PostalAddress`] — a human-readable multi-line string.

/// A validated postal address.
///
/// All text fields (`street`, `city`, `zip`) must be non-empty after trimming.
/// The `country` field is a validated [`CountryCode`]. On construction the text
/// fields are trimmed; no further normalisation is applied.
///
/// The `Display` / `value()` output is a multi-line string in the format:
/// ```text
/// <street>
/// <zip> <city>
/// <COUNTRY>
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use arvo::contact::{CountryCode, PostalAddress, PostalAddressInput};
/// use arvo::traits::ValueObject;
///
/// let addr = PostalAddress::new(PostalAddressInput {
///     street:  "Václavské náměstí 1".into(),
///     city:    "Prague".into(),
///     zip:     "110 00".into(),
///     country: CountryCode::new("CZ".into()).unwrap(),
/// }).unwrap();
///
/// assert_eq!(addr.street(), "Václavské náměstí 1");
/// assert_eq!(addr.city(), "Prague");
/// assert_eq!(addr.zip(), "110 00");
/// assert_eq!(addr.country().value(), "CZ");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "PostalAddressInput", into = "PostalAddressInput"))]
pub struct PostalAddress {
    street: String,
    city: String,
    zip: String,
    country: CountryCode,
    formatted: String,
}

impl ValueObject for PostalAddress {
    type Input = PostalAddressInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let street = value.street.trim().to_owned();
        let city = value.city.trim().to_owned();
        let zip = value.zip.trim().to_owned();

        if street.is_empty() {
            return Err(ValidationError::empty("PostalAddress.street"));
        }
        if city.is_empty() {
            return Err(ValidationError::empty("PostalAddress.city"));
        }
        if zip.is_empty() {
            return Err(ValidationError::empty("PostalAddress.zip"));
        }

        let formatted = format!("{}\n{} {}\n{}", street, zip, city, value.country.value());

        Ok(Self {
            street,
            city,
            zip,
            country: value.country,
            formatted,
        })
    }

    fn into_inner(self) -> Self::Input {
        PostalAddressInput {
            street: self.street,
            city: self.city,
            zip: self.zip,
            country: self.country,
        }
    }
}

impl PostalAddress {
    pub fn value(&self) -> &str {
        &self.formatted
    }

    /// Returns the street field, e.g. `"Václavské náměstí 1"`.
    pub fn street(&self) -> &str {
        &self.street
    }

    /// Returns the city field, e.g. `"Prague"`.
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns the ZIP/postal code field, e.g. `"110 00"`.
    pub fn zip(&self) -> &str {
        &self.zip
    }

    /// Returns the country code, e.g. `CountryCode("CZ")`.
    pub fn country(&self) -> &CountryCode {
        &self.country
    }
}

impl TryFrom<PostalAddressInput> for PostalAddress {
    type Error = ValidationError;
    fn try_from(input: PostalAddressInput) -> Result<Self, Self::Error> {
        Self::new(input)
    }
}

#[cfg(feature = "serde")]
impl From<PostalAddress> for PostalAddressInput {
    fn from(a: PostalAddress) -> PostalAddressInput {
        a.into_inner()
    }
}

/// Displays the address in a human-readable multi-line format.
impl std::fmt::Display for PostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PrimitiveValue, ValueObject};

    fn cz() -> CountryCode {
        CountryCode::new("CZ".into()).unwrap()
    }

    fn valid_input() -> PostalAddressInput {
        PostalAddressInput {
            street: "Václavské náměstí 1".into(),
            city: "Prague".into(),
            zip: "110 00".into(),
            country: cz(),
        }
    }

    #[test]
    fn constructs_valid_address() {
        let addr = PostalAddress::new(valid_input()).unwrap();
        assert_eq!(addr.street(), "Václavské náměstí 1");
        assert_eq!(addr.city(), "Prague");
        assert_eq!(addr.zip(), "110 00");
        assert_eq!(addr.country().value(), "CZ");
    }

    #[test]
    fn value_is_formatted_string() {
        let addr = PostalAddress::new(valid_input()).unwrap();
        assert_eq!(addr.value(), "Václavské náměstí 1\n110 00 Prague\nCZ");
    }

    #[test]
    fn display_matches_value() {
        let addr = PostalAddress::new(valid_input()).unwrap();
        assert_eq!(addr.to_string(), addr.value().to_owned());
    }

    #[test]
    fn trims_whitespace_from_fields() {
        let addr = PostalAddress::new(PostalAddressInput {
            street: "  Main St 1  ".into(),
            city: "  Berlin  ".into(),
            zip: "  10115  ".into(),
            country: CountryCode::new("DE".into()).unwrap(),
        })
        .unwrap();
        assert_eq!(addr.street(), "Main St 1");
        assert_eq!(addr.city(), "Berlin");
        assert_eq!(addr.zip(), "10115");
    }

    #[test]
    fn rejects_empty_street() {
        let mut input = valid_input();
        input.street = String::new();
        assert!(PostalAddress::new(input).is_err());
    }

    #[test]
    fn rejects_whitespace_only_street() {
        let mut input = valid_input();
        input.street = "   ".into();
        assert!(PostalAddress::new(input).is_err());
    }

    #[test]
    fn rejects_empty_city() {
        let mut input = valid_input();
        input.city = String::new();
        assert!(PostalAddress::new(input).is_err());
    }

    #[test]
    fn rejects_empty_zip() {
        let mut input = valid_input();
        input.zip = String::new();
        assert!(PostalAddress::new(input).is_err());
    }

    #[test]
    fn into_inner_returns_original_fields() {
        let addr = PostalAddress::new(valid_input()).unwrap();
        let inner = addr.into_inner();
        assert_eq!(inner.street, "Václavské náměstí 1");
        assert_eq!(inner.city, "Prague");
        assert_eq!(inner.zip, "110 00");
        assert_eq!(inner.country.value(), "CZ");
    }

    #[test]
    fn equal_addresses_are_equal() {
        let a = PostalAddress::new(valid_input()).unwrap();
        let b = PostalAddress::new(valid_input()).unwrap();
        assert_eq!(a, b);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let addr = PostalAddress::new(valid_input()).unwrap();
        let json = serde_json::to_string(&addr).unwrap();
        let back: PostalAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(addr.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<PostalAddress, _> = serde_json::from_str(r#"{"street":"","city":"Prague","zip":"110 00","country":"CZ"}"#);
        assert!(result.is_err());
    }
}
