use chrono::{DateTime, TimeZone, Utc};

use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`UnixTimestamp`].
pub type UnixTimestampInput = i64;

/// Output type for [`UnixTimestamp`].
pub type UnixTimestampOutput = i64;

/// A validated Unix timestamp — non-negative seconds since the Unix epoch.
///
/// Negative values (pre-1970) are rejected.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::temporal::UnixTimestamp;
/// use arvo::traits::ValueObject;
///
/// let ts = UnixTimestamp::new(1_700_000_000).unwrap();
/// assert_eq!(*ts.value(), 1_700_000_000);
///
/// assert!(UnixTimestamp::new(-1).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct UnixTimestamp(i64);

impl ValueObject for UnixTimestamp {
    type Input = UnixTimestampInput;
    type Output = UnixTimestampOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(ValidationError::invalid(
                "UnixTimestamp",
                &value.to_string(),
            ));
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

impl UnixTimestamp {
    /// Converts to a `DateTime<Utc>`.
    pub fn as_datetime(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.0, 0).single().expect("valid timestamp")
    }
}

impl TryFrom<i64> for UnixTimestamp {
    type Error = ValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for UnixTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_zero() {
        let ts = UnixTimestamp::new(0).unwrap();
        assert_eq!(*ts.value(), 0);
    }

    #[test]
    fn accepts_positive() {
        let ts = UnixTimestamp::new(1_700_000_000).unwrap();
        assert_eq!(*ts.value(), 1_700_000_000);
    }

    #[test]
    fn rejects_negative() {
        assert!(UnixTimestamp::new(-1).is_err());
    }

    #[test]
    fn into_inner_roundtrip() {
        let ts = UnixTimestamp::new(42).unwrap();
        assert_eq!(ts.into_inner(), 42);
    }

    #[test]
    fn as_datetime_epoch() {
        let ts = UnixTimestamp::new(0).unwrap();
        assert_eq!(ts.as_datetime().timestamp(), 0);
    }

    #[test]
    fn as_datetime_nonzero() {
        let ts = UnixTimestamp::new(1_700_000_000).unwrap();
        assert_eq!(ts.as_datetime().timestamp(), 1_700_000_000);
    }

    #[test]
    fn display() {
        let ts = UnixTimestamp::new(1_000).unwrap();
        assert_eq!(ts.to_string(), "1000");
    }
}
