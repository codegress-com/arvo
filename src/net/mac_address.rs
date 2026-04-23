use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`MacAddress`].
pub type MacAddressInput = String;

/// A validated MAC address, normalised to lowercase colon-separated hex.
///
/// **Normalisation:** accepts colon-separated (`AA:BB:CC:DD:EE:FF`),
/// hyphen-separated (`AA-BB-CC-DD-EE-FF`), or dotted (`AABB.CCDD.EEFF`) formats.
/// Stored as lowercase colon-separated (e.g. `"aa:bb:cc:dd:ee:ff"`).
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::MacAddress;
/// use arvo::traits::ValueObject;
///
/// let mac = MacAddress::new("AA:BB:CC:DD:EE:FF".into())?;
/// assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");
///
/// let mac: MacAddress = "AA-BB-CC-DD-EE-FF".try_into()?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct MacAddress(String);

impl ValueObject for MacAddress {
    type Input = MacAddressInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("MacAddress"));
        }

        let bytes = parse_mac_bytes(trimmed)
            .ok_or_else(|| ValidationError::invalid("MacAddress", trimmed))?;

        let canonical = format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        );

        Ok(Self(canonical))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for MacAddress {
    type Primitive = String;
    fn value(&self) -> &String {
        &self.0
    }
}

fn parse_mac_bytes(s: &str) -> Option<[u8; 6]> {
    // colon or hyphen separated: XX:XX:XX:XX:XX:XX or XX-XX-XX-XX-XX-XX
    let sep = if s.contains(':') {
        Some(':')
    } else if s.contains('-') {
        Some('-')
    } else {
        None
    };

    if let Some(sep) = sep {
        let parts: Vec<&str> = s.split(sep).collect();
        if parts.len() != 6 {
            return None;
        }
        let mut bytes = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            if part.len() != 2 {
                return None;
            }
            bytes[i] = u8::from_str_radix(part, 16).ok()?;
        }
        return Some(bytes);
    }

    // Cisco dotted: XXXX.XXXX.XXXX
    if s.contains('.') {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let hex: String = parts.concat();
        if hex.len() != 12 {
            return None;
        }
        let mut bytes = [0u8; 6];
        for i in 0..6 {
            bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
        }
        return Some(bytes);
    }

    None
}

impl TryFrom<String> for MacAddress {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<MacAddress> for String {
    fn from(v: MacAddress) -> String {
        v.0
    }
}
impl TryFrom<&str> for MacAddress {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_colon_separated() {
        let mac = MacAddress::new("AA:BB:CC:DD:EE:FF".into()).unwrap();
        assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn accepts_hyphen_separated() {
        let mac = MacAddress::new("AA-BB-CC-DD-EE-FF".into()).unwrap();
        assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn accepts_dotted_format() {
        let mac = MacAddress::new("AABB.CCDD.EEFF".into()).unwrap();
        assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn normalises_to_lowercase() {
        let mac = MacAddress::new("AA:BB:CC:DD:EE:FF".into()).unwrap();
        assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn rejects_empty() {
        assert!(MacAddress::new(String::new()).is_err());
    }

    #[test]
    fn rejects_too_few_groups() {
        assert!(MacAddress::new("AA:BB:CC:DD:EE".into()).is_err());
    }

    #[test]
    fn rejects_invalid_hex() {
        assert!(MacAddress::new("GG:BB:CC:DD:EE:FF".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let mac: MacAddress = "aa:bb:cc:dd:ee:ff".try_into().unwrap();
        assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = MacAddress::try_from("00:1A:2B:3C:4D:5E").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: MacAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<MacAddress, _> = serde_json::from_str("\"__invalid__\"");
        assert!(result.is_err());
    }
}
