use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};
use std::net::Ipv6Addr;

/// Input type for [`IpV6Address`].
pub type IpV6AddressInput = String;

/// Output type for [`IpV6Address`].

/// A validated IPv6 address (e.g. `"2001:db8::1"`).
///
/// **Normalisation:** trimmed; the address is stored in the canonical
/// compressed lowercase form produced by Rust's standard library
/// (e.g. `"2001:0db8:0000:0000:0000:0000:0000:0001"` → `"2001:db8::1"`).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::IpV6Address;
/// use arvo::traits::ValueObject;
///
/// let ip = IpV6Address::new("2001:0db8::0001".into())?;
/// assert_eq!(ip.value(), "2001:db8::1");
///
/// let ip: IpV6Address = "::1".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct IpV6Address(String);

impl ValueObject for IpV6Address {
    type Input = IpV6AddressInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("IpV6Address"));
        }

        trimmed
            .parse::<Ipv6Addr>()
            .map(|ip| Self(ip.to_string()))
            .map_err(|_| ValidationError::invalid("IpV6Address", trimmed))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for IpV6Address {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for IpV6Address {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<IpV6Address> for String {
    fn from(v: IpV6Address) -> String {
        v.0
    }
}
impl TryFrom<&str> for IpV6Address {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for IpV6Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_loopback() {
        let ip = IpV6Address::new("::1".into()).unwrap();
        assert_eq!(ip.value(), "::1");
    }

    #[test]
    fn normalises_to_compressed_form() {
        let ip = IpV6Address::new("2001:0db8:0000:0000:0000:0000:0000:0001".into()).unwrap();
        assert_eq!(ip.value(), "2001:db8::1");
    }

    #[test]
    fn accepts_full_address() {
        assert!(IpV6Address::new("2001:db8:85a3::8a2e:370:7334".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(IpV6Address::new(String::new()).is_err());
    }

    #[test]
    fn rejects_ipv4() {
        assert!(IpV6Address::new("192.168.1.1".into()).is_err());
    }

    #[test]
    fn rejects_invalid() {
        assert!(IpV6Address::new("not-an-ip".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let ip: IpV6Address = "::1".try_into().unwrap();
        assert_eq!(ip.value(), "::1");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = IpV6Address::try_from("::1").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: IpV6Address = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<IpV6Address, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
