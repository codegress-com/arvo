use crate::errors::ValidationError;
use crate::traits::ValueObject;

use super::{IpV4Address, IpV6Address};

/// Input for [`IpAddress`] — either a v4 or v6 address string.
pub type IpAddressInput = String;

/// Output type for [`IpAddress`].
pub type IpAddressOutput = String;

/// A validated IP address — either IPv4 or IPv6.
///
/// Tries IPv4 first, then IPv6. The canonical string is stored normalised.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::IpAddress;
/// use arvo::traits::ValueObject;
///
/// let ip = IpAddress::new("192.168.1.1".into())?;
/// assert!(ip.is_v4());
///
/// let ip = IpAddress::new("::1".into())?;
/// assert!(ip.is_v6());
///
/// let ip: IpAddress = "10.0.0.1".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct IpAddress(String);

impl ValueObject for IpAddress {
    type Input = IpAddressInput;
    type Output = IpAddressOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("IpAddress"));
        }

        if let Ok(v4) = IpV4Address::new(trimmed.to_owned()) {
            return Ok(Self(v4.into_inner()));
        }

        if let Ok(v6) = IpV6Address::new(trimmed.to_owned()) {
            return Ok(Self(v6.into_inner()));
        }

        Err(ValidationError::invalid("IpAddress", trimmed))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl IpAddress {
    /// Returns `true` if the address is IPv4.
    pub fn is_v4(&self) -> bool {
        self.0.contains('.')
    }

    /// Returns `true` if the address is IPv6.
    pub fn is_v6(&self) -> bool {
        self.0.contains(':')
    }
}

impl TryFrom<&str> for IpAddress {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ipv4() {
        let ip = IpAddress::new("192.168.1.1".into()).unwrap();
        assert_eq!(ip.value(), "192.168.1.1");
        assert!(ip.is_v4());
        assert!(!ip.is_v6());
    }

    #[test]
    fn accepts_ipv6() {
        let ip = IpAddress::new("::1".into()).unwrap();
        assert_eq!(ip.value(), "::1");
        assert!(ip.is_v6());
        assert!(!ip.is_v4());
    }

    #[test]
    fn normalises_ipv6() {
        let ip = IpAddress::new("2001:0db8::0001".into()).unwrap();
        assert_eq!(ip.value(), "2001:db8::1");
    }

    #[test]
    fn rejects_empty() {
        assert!(IpAddress::new(String::new()).is_err());
    }

    #[test]
    fn rejects_invalid() {
        assert!(IpAddress::new("not-an-ip".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let ip: IpAddress = "10.0.0.1".try_into().unwrap();
        assert!(ip.is_v4());
    }
}
