use chrono::{Datelike, Local};

use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`CardExpiryDate`] — accepts `"MM/YY"` or `"MM/YYYY"`.
pub type CardExpiryDateInput = String;

/// Output type for [`CardExpiryDate`] — normalised `"MM/YY"` string.
pub type CardExpiryDateOutput = String;

/// A validated credit/debit card expiry date.
///
/// Accepts `"MM/YY"` or `"MM/YYYY"` format. The month must be 01–12 and the
/// expiry must not have already passed — a card is valid through the entire
/// month of its expiry date. Output is normalised to `"MM/YY"`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::finance::CardExpiryDate;
/// use arvo::traits::ValueObject;
///
/// let exp = CardExpiryDate::new("12/28".into()).unwrap();
/// assert_eq!(exp.value(), "12/28");
/// assert_eq!(exp.month(), 12);
/// assert_eq!(exp.year(), 2028);
///
/// // A date in the past is rejected
/// assert!(CardExpiryDate::new("01/20".into()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct CardExpiryDate(String);

impl ValueObject for CardExpiryDate {
    type Input = CardExpiryDateInput;
    type Output = CardExpiryDateOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::empty("CardExpiryDate"));
        }

        let parts: Vec<&str> = trimmed.split('/').collect();
        if parts.len() != 2 {
            return Err(ValidationError::invalid("CardExpiryDate", trimmed));
        }

        let month: u8 = parts[0]
            .parse()
            .map_err(|_| ValidationError::invalid("CardExpiryDate", trimmed))?;

        if !(1..=12).contains(&month) {
            return Err(ValidationError::invalid("CardExpiryDate", trimmed));
        }

        let year_str = parts[1];
        let full_year: u16 = match year_str.len() {
            2 => {
                let yy: u16 = year_str
                    .parse()
                    .map_err(|_| ValidationError::invalid("CardExpiryDate", trimmed))?;
                2000 + yy
            }
            4 => year_str
                .parse()
                .map_err(|_| ValidationError::invalid("CardExpiryDate", trimmed))?,
            _ => return Err(ValidationError::invalid("CardExpiryDate", trimmed)),
        };

        let now = Local::now();
        let current_year = now.year() as u16;
        let current_month = now.month() as u8;

        if full_year < current_year || (full_year == current_year && month < current_month) {
            return Err(ValidationError::invalid("CardExpiryDate", trimmed));
        }

        let canonical = format!("{:02}/{:02}", month, full_year % 100);
        Ok(Self(canonical))
    }

    fn value(&self) -> &Self::Output {
        &self.0
    }

    fn into_inner(self) -> Self::Input {
        self.0
    }
}

impl CardExpiryDate {
    /// Returns the expiry month as a number (1–12).
    pub fn month(&self) -> u8 {
        self.0[..2].parse().unwrap()
    }

    /// Returns the 4-digit expiry year, e.g. `2028`.
    pub fn year(&self) -> u16 {
        let yy: u16 = self.0[3..].parse().unwrap();
        2000 + yy
    }

    /// Returns the number of full months from the current month until expiry.
    pub fn months_until(&self) -> u32 {
        let now = Local::now();
        let current_year = now.year() as u16;
        let current_month = now.month() as u8;
        let expiry_months = self.year() * 12 + self.month() as u16;
        let current_months = current_year * 12 + current_month as u16;
        expiry_months.saturating_sub(current_months) as u32
    }
}


impl TryFrom<String> for CardExpiryDate {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl From<CardExpiryDate> for String {
    fn from(v: CardExpiryDate) -> String {
        v.0
    }
}
impl TryFrom<&str> for CardExpiryDate {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl std::fmt::Display for CardExpiryDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_future_mm_yy() {
        let e = CardExpiryDate::new("12/28".into()).unwrap();
        assert_eq!(e.value(), "12/28");
    }

    #[test]
    fn accepts_future_mm_yyyy() {
        let e = CardExpiryDate::new("06/2030".into()).unwrap();
        assert_eq!(e.value(), "06/30");
    }

    #[test]
    fn accepts_current_month() {
        // April 2026 — current month, still valid
        let e = CardExpiryDate::new("04/26".into()).unwrap();
        assert_eq!(e.value(), "04/26");
    }

    #[test]
    fn month_accessor() {
        let e = CardExpiryDate::new("12/28".into()).unwrap();
        assert_eq!(e.month(), 12);
    }

    #[test]
    fn year_accessor() {
        let e = CardExpiryDate::new("12/28".into()).unwrap();
        assert_eq!(e.year(), 2028);
    }

    #[test]
    fn rejects_past_date() {
        assert!(CardExpiryDate::new("01/25".into()).is_err());
    }

    #[test]
    fn rejects_previous_month() {
        assert!(CardExpiryDate::new("03/26".into()).is_err());
    }

    #[test]
    fn rejects_empty() {
        assert!(CardExpiryDate::new(String::new()).is_err());
    }

    #[test]
    fn rejects_invalid_month() {
        assert!(CardExpiryDate::new("13/28".into()).is_err());
        assert!(CardExpiryDate::new("00/28".into()).is_err());
    }

    #[test]
    fn rejects_invalid_format() {
        assert!(CardExpiryDate::new("12-28".into()).is_err());
        assert!(CardExpiryDate::new("1228".into()).is_err());
    }

    #[test]
    fn try_from_str() {
        let e: CardExpiryDate = "12/28".try_into().unwrap();
        assert_eq!(e.value(), "12/28");
    }
}
