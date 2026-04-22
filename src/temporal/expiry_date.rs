use chrono::{Local, NaiveDate};

use crate::errors::ValidationError;
use crate::traits::{PrimitiveValue, ValueObject};

/// Input type for [`ExpiryDate`].
pub type ExpiryDateInput = NaiveDate;

/// Output type for [`ExpiryDate`].

/// A validated expiry date that is strictly in the future.
///
/// The date must be after today at construction time.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::temporal::ExpiryDate;
/// use arvo::traits::ValueObject;
/// use chrono::NaiveDate;
///
/// let exp = ExpiryDate::new(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()).unwrap();
/// assert!(exp.days_until() > 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "chrono::NaiveDate", into = "chrono::NaiveDate"))]
pub struct ExpiryDate(NaiveDate);

impl ValueObject for ExpiryDate {
    type Input = ExpiryDateInput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let today = Local::now().date_naive();

        if value <= today {
            return Err(ValidationError::invalid("ExpiryDate", &value.to_string()));
        }

        Ok(Self(value))
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}
impl PrimitiveValue for ExpiryDate {
    type Primitive = chrono::NaiveDate;
    fn value(&self) -> &chrono::NaiveDate {
        &self.0
    }
}

impl ExpiryDate {
    /// Returns the number of days from today until the expiry date.
    pub fn days_until(&self) -> i64 {
        let today = Local::now().date_naive();
        (self.0 - today).num_days()
    }
}

impl TryFrom<chrono::NaiveDate> for ExpiryDate {
    type Error = ValidationError;
    fn try_from(v: chrono::NaiveDate) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

#[cfg(feature = "serde")]
impl From<ExpiryDate> for chrono::NaiveDate {
    fn from(v: ExpiryDate) -> chrono::NaiveDate {
        v.0
    }
}
impl TryFrom<&str> for ExpiryDate {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = chrono::NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d").map_err(|_| ValidationError::invalid("ExpiryDate", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for ExpiryDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn future_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()
    }

    #[test]
    fn accepts_future_date() {
        assert!(ExpiryDate::new(future_date()).is_ok());
    }

    #[test]
    fn value_returns_date() {
        let d = ExpiryDate::new(future_date()).unwrap();
        assert_eq!(*d.value(), future_date());
    }

    #[test]
    fn rejects_today() {
        let today = Local::now().date_naive();
        assert!(ExpiryDate::new(today).is_err());
    }

    #[test]
    fn rejects_past() {
        let past = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert!(ExpiryDate::new(past).is_err());
    }

    #[test]
    fn days_until_positive() {
        let d = ExpiryDate::new(future_date()).unwrap();
        assert!(d.days_until() > 0);
    }

    #[test]
    fn display_is_iso_date() {
        let d = ExpiryDate::new(future_date()).unwrap();
        assert_eq!(d.to_string(), "2030-12-31");
    }

    #[test]
    fn into_inner_roundtrip() {
        let d = ExpiryDate::new(future_date()).unwrap();
        assert_eq!(d.into_inner(), future_date());
    }

    #[test]
    fn try_from_parses_valid() {
        let d = ExpiryDate::try_from("2030-12-31").unwrap();
        assert_eq!(d.to_string(), "2030-12-31");
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(ExpiryDate::try_from("31-12-2030").is_err());
    }

    #[test]
    fn try_from_rejects_past_date() {
        assert!(ExpiryDate::try_from("2020-01-01").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = ExpiryDate::try_from("2030-12-31").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, "\"2030-12-31\"");
        let back: ExpiryDate = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_deserialize_validates() {
        let result: Result<ExpiryDate, _> = serde_json::from_str("\"2020-01-01\"");
        assert!(result.is_err());
    }
}
