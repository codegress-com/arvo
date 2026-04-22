use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};
use std::net::Ipv4Addr;

/// Input type for [`IpV4Address`].
pub type IpV4AddressInput = String;

/// A validated IPv4 address (e.g. `"192.168.1.1"`).
///
/// **Normalisation:** trimmed. Leading zeros in octets are rejected
/// (e.g. `"192.168.001.001"` is invalid).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::IpV4Address;
/// use arvo::traits::ValueObject;
///
/// let ip = IpV4Address::new("192.168.1.1".into())?;
/// assert_eq!(ip.value(), "192.168.1.1");
///
/// let ip: IpV4Address = "10.0.0.1".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct IpV4Address(String);

impl ValueObject for IpV4Address {
    type Input = IpV4AddressInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("IpV4Address"));
        }

        // Reject leading zeros in any octet
        for octet in trimmed.split('.') {
            if octet.len() > 1 && octet.starts_with('0') {
                return Err(ValidationError::invalid("IpV4Address", trimmed));
            }
        }

        trimmed
            .parse::<Ipv4Addr>()
            .map(|ip| Self(ip.to_string()))
            .map_err(|_| ValidationError::invalid("IpV4Address", trimmed))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for IpV4Address {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

impl IpV4Address {
    /// Returns `true` for loopback addresses (`127.0.0.0/8`).
    pub fn is_loopback(&self) -> bool {
        self.0
            .parse::<std::net::Ipv4Addr>()
            .map(|ip| ip.is_loopback())
            .unwrap_or(false)
    }

    /// Returns `true` for private addresses (10/8, 172.16/12, 192.168/16).
    pub fn is_private(&self) -> bool {
        self.0
            .parse::<std::net::Ipv4Addr>()
            .map(|ip| ip.is_private())
            .unwrap_or(false)
    }
}

impl TryFrom<String> for IpV4Address {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<IpV4Address> for String {
    fn from(v: IpV4Address) -> String {
        v.0
    }
}
impl TryFrom<&str> for IpV4Address {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for IpV4Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_ipv4() {
        let ip = IpV4Address::new("192.168.1.1".into()).unwrap();
        assert_eq!(ip.value(), "192.168.1.1");
    }

    #[test]
    fn accepts_loopback() {
        assert!(IpV4Address::new("127.0.0.1".into()).is_ok());
    }

    #[test]
    fn accepts_all_zeros() {
        assert!(IpV4Address::new("0.0.0.0".into()).is_ok());
    }

    #[test]
    fn accepts_broadcast() {
        assert!(IpV4Address::new("255.255.255.255".into()).is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(IpV4Address::new(String::new()).is_err());
    }

    #[test]
    fn rejects_out_of_range_octet() {
        assert!(IpV4Address::new("256.0.0.1".into()).is_err());
    }

    #[test]
    fn rejects_too_few_octets() {
        assert!(IpV4Address::new("192.168.1".into()).is_err());
    }

    #[test]
    fn rejects_leading_zeros() {
        assert!(IpV4Address::new("192.168.001.001".into()).is_err());
    }

    #[test]
    fn rejects_ipv6() {
        assert!(IpV4Address::new("::1".into()).is_err());
    }

    #[test]
    fn is_loopback() {
        assert!(IpV4Address::new("127.0.0.1".into()).unwrap().is_loopback());
        assert!(
            !IpV4Address::new("192.168.1.1".into())
                .unwrap()
                .is_loopback()
        );
    }

    #[test]
    fn is_private() {
        assert!(IpV4Address::new("10.0.0.1".into()).unwrap().is_private());
        assert!(IpV4Address::new("172.16.0.1".into()).unwrap().is_private());
        assert!(IpV4Address::new("192.168.1.1".into()).unwrap().is_private());
        assert!(!IpV4Address::new("8.8.8.8".into()).unwrap().is_private());
    }

    #[test]
    fn try_from_str() {
        let ip: IpV4Address = "10.0.0.1".try_into().unwrap();
        assert_eq!(ip.value(), "10.0.0.1");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = IpV4Address::try_from("192.168.1.1").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: IpV4Address = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<IpV4Address, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
