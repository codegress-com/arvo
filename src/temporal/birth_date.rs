use chrono::{Datelike, Local, NaiveDate};

use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`BirthDate`].
pub type BirthDateInput = NaiveDate;

/// Output type for [`BirthDate`].
pub type BirthDateOutput = NaiveDate;

/// A validated date of birth.
///
/// The date must be strictly in the past and no more than 150 years before
/// today at construction time.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::temporal::BirthDate;
/// use arvo::traits::ValueObject;
/// use chrono::NaiveDate;
///
/// let dob = BirthDate::new(NaiveDate::from_ymd_opt(1990, 6, 15).unwrap()).unwrap();
/// assert_eq!(dob.value().year(), 1990);
/// assert!(dob.age_years() > 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct BirthDate(NaiveDate);

impl ValueObject for BirthDate {
    type Input = BirthDateInput;
    type Output = BirthDateOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let today = Local::now().date_naive();

        if value >= today {
            return Err(ValidationError::invalid("BirthDate", &value.to_string()));
        }

        let min_date = today
            .with_year(today.year() - 150)
            .unwrap_or(NaiveDate::from_ymd_opt(today.year() - 150, today.month(), 1).unwrap());

        if value < min_date {
            return Err(ValidationError::invalid("BirthDate", &value.to_string()));
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

impl BirthDate {
    /// Returns the person's age in full completed years as of today.
    pub fn age_years(&self) -> u32 {
        let today = Local::now().date_naive();
        let years = today.year() - self.0.year();
        let had_birthday = (today.month(), today.day()) >= (self.0.month(), self.0.day());
        if had_birthday {
            years as u32
        } else {
            (years - 1) as u32
        }
    }

    /// Returns `true` if the person is under 18 years old as of today.
    pub fn is_minor(&self) -> bool {
        self.age_years() < 18
    }
}

impl TryFrom<&str> for BirthDate {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = chrono::NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d").map_err(|_| ValidationError::invalid("BirthDate", value))?;
        Self::new(parsed)
    }
}

impl std::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn past_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(1990, 6, 15).unwrap()
    }

    #[test]
    fn accepts_past_date() {
        assert!(BirthDate::new(past_date()).is_ok());
    }

    #[test]
    fn value_returns_date() {
        let d = BirthDate::new(past_date()).unwrap();
        assert_eq!(*d.value(), past_date());
    }

    #[test]
    fn rejects_today() {
        let today = Local::now().date_naive();
        assert!(BirthDate::new(today).is_err());
    }

    #[test]
    fn rejects_future() {
        let future = Local::now().date_naive() + chrono::Duration::days(1);
        assert!(BirthDate::new(future).is_err());
    }

    #[test]
    fn rejects_too_old() {
        let ancient = NaiveDate::from_ymd_opt(1800, 1, 1).unwrap();
        assert!(BirthDate::new(ancient).is_err());
    }

    #[test]
    fn age_years_positive() {
        let d = BirthDate::new(past_date()).unwrap();
        assert!(d.age_years() > 0);
    }

    #[test]
    fn display_is_iso_date() {
        let d = BirthDate::new(past_date()).unwrap();
        assert_eq!(d.to_string(), "1990-06-15");
    }

    #[test]
    fn into_inner_roundtrip() {
        let d = BirthDate::new(past_date()).unwrap();
        assert_eq!(d.into_inner(), past_date());
    }

    #[test]
    fn try_from_parses_valid() {
        let d = BirthDate::try_from("1990-06-15").unwrap();
        assert_eq!(d.value().to_string(), "1990-06-15");
    }

    #[test]
    fn try_from_rejects_invalid_format() {
        assert!(BirthDate::try_from("15-06-1990").is_err());
    }

    #[test]
    fn try_from_rejects_future_date() {
        assert!(BirthDate::try_from("2099-01-01").is_err());
    }
}
