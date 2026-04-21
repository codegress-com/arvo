use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`Port`].
pub type PortInput = u16;

/// Output type for [`Port`].
pub type PortOutput = u16;

/// A validated network port number in the range `1..=65535`.
///
/// Port 0 is reserved and rejected.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::net::Port;
/// use arvo::traits::ValueObject;
///
/// let port = Port::new(8080)?;
/// assert_eq!(*port.value(), 8080);
///
/// assert!(Port::new(0).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Port(u16);

impl ValueObject for Port {
    type Input = PortInput;
    type Output = PortOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(ValidationError::invalid("Port", &value.to_string()));
        }
        Ok(Self(value))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_port() {
        let port = Port::new(8080).unwrap();
        assert_eq!(*port.value(), 8080);
    }

    #[test]
    fn accepts_port_1() {
        assert!(Port::new(1).is_ok());
    }

    #[test]
    fn accepts_port_65535() {
        assert!(Port::new(65535).is_ok());
    }

    #[test]
    fn accepts_well_known_ports() {
        assert!(Port::new(80).is_ok());
        assert!(Port::new(443).is_ok());
        assert!(Port::new(22).is_ok());
    }

    #[test]
    fn rejects_zero() {
        assert!(Port::new(0).is_err());
    }

    #[test]
    fn display() {
        let port = Port::new(443).unwrap();
        assert_eq!(port.to_string(), "443");
    }

    #[test]
    fn into_inner_roundtrip() {
        let port = Port::new(3000).unwrap();
        assert_eq!(port.into_inner(), 3000);
    }
}
