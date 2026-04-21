use chrono::{Duration, NaiveTime, Timelike, Weekday};

use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`BusinessHours`] construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BusinessHoursInput {
    /// Day of the week.
    pub weekday: Weekday,
    /// Opening time — must be strictly before `close`.
    pub open: NaiveTime,
    /// Closing time — must be strictly after `open`.
    pub close: NaiveTime,
}

/// Output type for [`BusinessHours`] — canonical `"<Day> HH:MM–HH:MM"` string.
pub type BusinessHoursOutput = String;

/// Validated business hours for a single weekday.
///
/// `open` must be strictly before `close`. The canonical output is formatted
/// as `"<Day> HH:MM–HH:MM"`, e.g. `"Mon 09:00–17:00"`.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::temporal::{BusinessHours, BusinessHoursInput};
/// use arvo::traits::ValueObject;
/// use chrono::{NaiveTime, Weekday};
///
/// let hours = BusinessHours::new(BusinessHoursInput {
///     weekday: Weekday::Mon,
///     open:    NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
///     close:   NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
/// }).unwrap();
///
/// assert_eq!(hours.value(), "Mon 09:00–17:00");
/// assert_eq!(hours.duration().num_hours(), 8);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BusinessHours {
    weekday: Weekday,
    open: NaiveTime,
    close: NaiveTime,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for BusinessHours {
    type Input = BusinessHoursInput;
    type Output = BusinessHoursOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value.open >= value.close {
            return Err(ValidationError::invalid(
                "BusinessHours",
                &format!(
                    "{} {}–{}",
                    weekday_abbr(value.weekday),
                    value.open,
                    value.close
                ),
            ));
        }

        let canonical = format!(
            "{} {:02}:{:02}–{:02}:{:02}",
            weekday_abbr(value.weekday),
            value.open.hour(),
            value.open.minute(),
            value.close.hour(),
            value.close.minute(),
        );

        Ok(Self {
            weekday: value.weekday,
            open: value.open,
            close: value.close,
            canonical,
        })
    }

    fn value(&self) -> &Self::Output {
        &self.canonical
    }

    fn into_inner(self) -> Self::Input {
        BusinessHoursInput {
            weekday: self.weekday,
            open: self.open,
            close: self.close,
        }
    }
}

impl BusinessHours {
    /// Returns the weekday.
    pub fn weekday(&self) -> Weekday {
        self.weekday
    }

    /// Returns the opening time.
    pub fn open(&self) -> &NaiveTime {
        &self.open
    }

    /// Returns the closing time.
    pub fn close(&self) -> &NaiveTime {
        &self.close
    }

    /// Returns the duration of the business day (`close - open`).
    pub fn duration(&self) -> Duration {
        self.close - self.open
    }
}

impl std::fmt::Display for BusinessHours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

fn weekday_abbr(wd: Weekday) -> &'static str {
    match wd {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open() -> NaiveTime {
        NaiveTime::from_hms_opt(9, 0, 0).unwrap()
    }

    fn close() -> NaiveTime {
        NaiveTime::from_hms_opt(17, 0, 0).unwrap()
    }

    fn valid_input() -> BusinessHoursInput {
        BusinessHoursInput {
            weekday: Weekday::Mon,
            open: open(),
            close: close(),
        }
    }

    #[test]
    fn accepts_valid_hours() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert_eq!(h.value(), "Mon 09:00–17:00");
    }

    #[test]
    fn weekday_accessor() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert_eq!(h.weekday(), Weekday::Mon);
    }

    #[test]
    fn open_accessor() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert_eq!(h.open(), &open());
    }

    #[test]
    fn close_accessor() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert_eq!(h.close(), &close());
    }

    #[test]
    fn duration_is_eight_hours() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert_eq!(h.duration().num_hours(), 8);
    }

    #[test]
    fn rejects_equal_open_close() {
        assert!(
            BusinessHours::new(BusinessHoursInput {
                weekday: Weekday::Mon,
                open: open(),
                close: open(),
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_open_after_close() {
        assert!(
            BusinessHours::new(BusinessHoursInput {
                weekday: Weekday::Mon,
                open: close(),
                close: open(),
            })
            .is_err()
        );
    }

    #[test]
    fn formats_all_weekdays() {
        for (wd, abbr) in [
            (Weekday::Mon, "Mon"),
            (Weekday::Tue, "Tue"),
            (Weekday::Wed, "Wed"),
            (Weekday::Thu, "Thu"),
            (Weekday::Fri, "Fri"),
            (Weekday::Sat, "Sat"),
            (Weekday::Sun, "Sun"),
        ] {
            let h = BusinessHours::new(BusinessHoursInput {
                weekday: wd,
                open: open(),
                close: close(),
            })
            .unwrap();
            assert!(h.value().starts_with(abbr));
        }
    }

    #[test]
    fn display_matches_value() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert_eq!(h.to_string(), h.value().to_owned());
    }

    #[test]
    fn into_inner_roundtrip() {
        let input = valid_input();
        let h = BusinessHours::new(input.clone()).unwrap();
        assert_eq!(h.into_inner(), input);
    }
}
