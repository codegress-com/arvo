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
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct BusinessHours {
    weekday: Weekday,
    open: NaiveTime,
    close: NaiveTime,
    canonical: String,
}

impl ValueObject for BusinessHours {
    type Input = BusinessHoursInput;
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

    fn into_inner(self) -> Self::Input {
        BusinessHoursInput {
            weekday: self.weekday,
            open: self.open,
            close: self.close,
        }
    }
}

impl BusinessHours {
    pub fn value(&self) -> &str {
        &self.canonical
    }

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

    /// Returns `true` if `time` falls within `[open, close)`.
    pub fn is_open_at(&self, time: NaiveTime) -> bool {
        time >= self.open && time < self.close
    }
}

impl TryFrom<&str> for BusinessHours {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("BusinessHours", value);
        let (day_str, times_str) = value.trim().split_once(' ').ok_or_else(err)?;
        let weekday = match day_str {
            "Mon" => chrono::Weekday::Mon,
            "Tue" => chrono::Weekday::Tue,
            "Wed" => chrono::Weekday::Wed,
            "Thu" => chrono::Weekday::Thu,
            "Fri" => chrono::Weekday::Fri,
            "Sat" => chrono::Weekday::Sat,
            "Sun" => chrono::Weekday::Sun,
            _ => return Err(err()),
        };
        let (open_str, close_str) = times_str.split_once('\u{2013}').ok_or_else(err)?;
        let open =
            chrono::NaiveTime::parse_from_str(open_str.trim(), "%H:%M").map_err(|_| err())?;
        let close =
            chrono::NaiveTime::parse_from_str(close_str.trim(), "%H:%M").map_err(|_| err())?;
        Self::new(BusinessHoursInput {
            weekday,
            open,
            close,
        })
    }
}

#[cfg(feature = "serde")]
impl From<BusinessHours> for String {
    fn from(v: BusinessHours) -> String {
        v.canonical
    }
}

impl TryFrom<String> for BusinessHours {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
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
    fn is_open_at_during_hours() {
        let h = BusinessHours::new(valid_input()).unwrap();
        let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert!(h.is_open_at(noon));
    }

    #[test]
    fn is_open_at_open_time_inclusive() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert!(h.is_open_at(open()));
    }

    #[test]
    fn is_open_at_close_time_exclusive() {
        let h = BusinessHours::new(valid_input()).unwrap();
        assert!(!h.is_open_at(close()));
    }

    #[test]
    fn is_open_at_before_open() {
        let h = BusinessHours::new(valid_input()).unwrap();
        let early = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        assert!(!h.is_open_at(early));
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

    #[test]
    fn try_from_parses_valid() {
        let h = BusinessHours::try_from("Mon 09:00–17:00").unwrap();
        assert_eq!(h.value(), "Mon 09:00–17:00");
    }

    #[test]
    fn try_from_rejects_invalid_day() {
        assert!(BusinessHours::try_from("Xyz 09:00–17:00").is_err());
    }

    #[test]
    fn try_from_rejects_close_before_open() {
        assert!(BusinessHours::try_from("Mon 17:00–09:00").is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = BusinessHours::try_from("Mon 09:00–17:00").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        let back: BusinessHours = serde_json::from_str(&json).unwrap();
        assert_eq!(v.value(), back.value());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_serializes_as_canonical_string() {
        let v = BusinessHours::try_from("Mon 09:00–17:00").unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("Mon"));
    }
}
