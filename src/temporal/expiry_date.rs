use chrono::{Local, NaiveDate};

use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`ExpiryDate`].
pub type ExpiryDateInput = NaiveDate;

/// Output type for [`ExpiryDate`].
pub type ExpiryDateOutput = NaiveDate;

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
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ExpiryDate(NaiveDate);

impl ValueObject for ExpiryDate {
    type Input = ExpiryDateInput;
    type Output = ExpiryDateOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let today = Local::now().date_naive();

        if value <= today {
            return Err(ValidationError::invalid("ExpiryDate", &value.to_string()));
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

impl ExpiryDate {
    /// Returns the number of days from today until the expiry date.
    pub fn days_until(&self) -> i64 {
        let today = Local::now().date_naive();
        (self.0 - today).num_days()
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
}
