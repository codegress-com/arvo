use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`CurrencyCode`].
pub type CurrencyCodeInput = String;

/// Active ISO 4217 alphabetic currency codes, sorted for binary search.
static ISO_4217: &[&str] = &[
    "AED", "AFN", "ALL", "AMD", "ANG", "AOA", "ARS", "AUD", "AWG", "AZN", "BAM", "BBD", "BDT",
    "BGN", "BHD", "BMD", "BND", "BOB", "BOV", "BRL", "BSD", "BTN", "BWP", "BYN", "BZD", "CAD",
    "CDF", "CHE", "CHF", "CHW", "CLF", "CLP", "CNY", "COP", "COU", "CRC", "CUP", "CVE", "CZK",
    "DJF", "DKK", "DOP", "DZD", "EGP", "ERN", "ETB", "EUR", "FJD", "FKP", "GBP", "GEL", "GHS",
    "GIP", "GMD", "GNF", "GTQ", "GYD", "HKD", "HNL", "HTG", "HUF", "IDR", "ILS", "INR", "IQD",
    "IRR", "ISK", "JMD", "JOD", "JPY", "KES", "KGS", "KHR", "KMF", "KPW", "KRW", "KWD", "KYD",
    "KZT", "LAK", "LBP", "LKR", "LRD", "LSL", "LYD", "MAD", "MDL", "MGA", "MKD", "MMK", "MNT",
    "MOP", "MRU", "MUR", "MVR", "MWK", "MXN", "MXV", "MYR", "MZN", "NAD", "NGN", "NIO", "NOK",
    "NPR", "NZD", "OMR", "PAB", "PEN", "PGK", "PHP", "PKR", "PLN", "PYG", "QAR", "RON", "RSD",
    "RUB", "RWF", "SAR", "SBD", "SCR", "SDG", "SEK", "SGD", "SHP", "SLE", "SLL", "SOS", "SRD",
    "SSP", "STN", "SVC", "SYP", "SZL", "THB", "TJS", "TMT", "TND", "TOP", "TRY", "TTD", "TWD",
    "TZS", "UAH", "UGX", "USD", "USN", "UYI", "UYU", "UYW", "UZS", "VED", "VES", "VND", "VUV",
    "WST", "XAF", "XAG", "XAU", "XBA", "XBB", "XBC", "XBD", "XCD", "XDR", "XOF", "XPD", "XPF",
    "XPT", "XSU", "XTS", "XUA", "XXX", "YER", "ZAR", "ZMW", "ZWG", "ZWL",
];

/// A validated ISO 4217 alphabetic currency code.
///
/// On construction the input is trimmed and uppercased. Only active ISO 4217
/// codes (e.g. `"EUR"`, `"USD"`, `"CZK"`) are accepted.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::CurrencyCode;
/// use arvo::traits::ValueObject;
///
/// let code = CurrencyCode::new("eur".into()).unwrap();
/// assert_eq!(code.value(), "EUR");
///
/// assert!(CurrencyCode::new("XYZ".into()).is_err());
/// assert!(CurrencyCode::new("US".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct CurrencyCode(String);

impl ValueObject for CurrencyCode {
    type Input = CurrencyCodeInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let upper = value.trim().to_uppercase();

        if upper.is_empty() {
            return Err(ValidationError::empty("CurrencyCode"));
        }

        if upper.len() != 3 || !upper.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ValidationError::invalid("CurrencyCode", &upper));
        }

        if ISO_4217.binary_search(&upper.as_str()).is_err() {
            return Err(ValidationError::invalid("CurrencyCode", &upper));
        }

        Ok(Self(upper))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for CurrencyCode {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for CurrencyCode {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<CurrencyCode> for String {
    fn from(v: CurrencyCode) -> String {
        v.0
    }
}
impl TryFrom<&str> for CurrencyCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_code() {
        let c = CurrencyCode::new("EUR".into()).unwrap();
        assert_eq!(c.value(), "EUR");
    }

    #[test]
    fn normalises_to_uppercase() {
        let c = CurrencyCode::new("eur".into()).unwrap();
        assert_eq!(c.value(), "EUR");
    }

    #[test]
    fn trims_whitespace() {
        let c = CurrencyCode::new("  USD  ".into()).unwrap();
        assert_eq!(c.value(), "USD");
    }

    #[test]
    fn accepts_czk() {
        assert!(CurrencyCode::new("CZK".into()).is_ok());
    }

    #[test]
    fn accepts_jpy() {
        assert!(CurrencyCode::new("JPY".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(CurrencyCode::new(String::new()).is_err());
    }

    #[test]
    fn rejects_unknown_code() {
        assert!(CurrencyCode::new("XYZ".into()).is_err());
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(CurrencyCode::new("US".into()).is_err());
        assert!(CurrencyCode::new("USDX".into()).is_err());
    }

    #[test]
    fn rejects_digits() {
        assert!(CurrencyCode::new("U5D".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let c: CurrencyCode = "GBP".try_into().unwrap();
        assert_eq!(c.value(), "GBP");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = CurrencyCode::try_from("EUR").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: CurrencyCode = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<CurrencyCode, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
