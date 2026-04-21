use chrono::{DateTime, Duration, Utc};

use crate::errors::ValidationError;
use crate::traits::ValueObject;

/// Input type for [`TimeRange`] construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeRangeInput {
    /// Start of the range (inclusive).
    pub start: DateTime<Utc>,
    /// End of the range (exclusive).
    pub end: DateTime<Utc>,
}

/// Output type for [`TimeRange`] — canonical `"<start> / <end>"` string.
pub type TimeRangeOutput = String;

/// A validated time range with a start strictly before its end.
///
/// Both `start` and `end` are `chrono::DateTime<Utc>`. The canonical output
/// is formatted as `"<start> / <end>"` in RFC 3339 format.
///
/// # Example
///
/// ```rust,ignore
/// use arvo::temporal::{TimeRange, TimeRangeInput};
/// use arvo::traits::ValueObject;
/// use chrono::{TimeZone, Utc};
///
/// let range = TimeRange::new(TimeRangeInput {
///     start: Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
///     end:   Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap(),
/// }).unwrap();
///
/// assert_eq!(range.value(), "2025-01-01 10:00:00 UTC / 2025-01-01 12:00:00 UTC");
/// assert_eq!(range.duration().num_hours(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    #[cfg_attr(feature = "serde", serde(skip))]
    canonical: String,
}

impl ValueObject for TimeRange {
    type Input = TimeRangeInput;
    type Output = TimeRangeOutput;
    type Error = ValidationError;

    fn new(value: Self::Input) -> Result<Self, Self::Error> {
        if value.start >= value.end {
            return Err(ValidationError::invalid(
                "TimeRange",
                &format!("{} / {}", value.start, value.end),
            ));
        }

        let canonical = format!("{} / {}", value.start, value.end);
        Ok(Self {
            start: value.start,
            end: value.end,
            canonical,
        })
    }

    fn value(&self) -> &Self::Output {
        &self.canonical
    }

    fn into_inner(self) -> Self::Input {
        TimeRangeInput {
            start: self.start,
            end: self.end,
        }
    }
}

impl TimeRange {
    /// Returns the start of the range.
    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }

    /// Returns the end of the range.
    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }

    /// Returns the duration of the range (`end - start`).
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    /// Returns `true` if `dt` falls within `[start, end)`.
    pub fn contains(&self, dt: &DateTime<Utc>) -> bool {
        dt >= &self.start && dt < &self.end
    }

    /// Returns `true` if this range overlaps with `other` (shares at least one instant).
    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start < other.end && other.start < self.end
    }
}

impl TryFrom<&str> for TimeRange {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = || ValidationError::invalid("TimeRange", value);
        let (start_str, end_str) = value.trim().split_once(" / ").ok_or_else(err)?;
        let start: chrono::DateTime<chrono::Utc> = start_str.trim().parse().map_err(|_| err())?;
        let end: chrono::DateTime<chrono::Utc> = end_str.trim().parse().map_err(|_| err())?;
        Self::new(TimeRangeInput { start, end })
    }
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn start() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap()
    }

    fn end() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap()
    }

    #[test]
    fn accepts_valid_range() {
        assert!(
            TimeRange::new(TimeRangeInput {
                start: start(),
                end: end()
            })
            .is_ok()
        );
    }

    #[test]
    fn start_accessor() {
        let r = TimeRange::new(TimeRangeInput {
            start: start(),
            end: end(),
        })
        .unwrap();
        assert_eq!(r.start(), &start());
    }

    #[test]
    fn end_accessor() {
        let r = TimeRange::new(TimeRangeInput {
            start: start(),
            end: end(),
        })
        .unwrap();
        assert_eq!(r.end(), &end());
    }

    #[test]
    fn duration_is_two_hours() {
        let r = TimeRange::new(TimeRangeInput {
            start: start(),
            end: end(),
        })
        .unwrap();
        assert_eq!(r.duration().num_hours(), 2);
    }

    #[test]
    fn rejects_equal_start_end() {
        assert!(
            TimeRange::new(TimeRangeInput {
                start: start(),
                end: start()
            })
            .is_err()
        );
    }

    #[test]
    fn rejects_start_after_end() {
        assert!(
            TimeRange::new(TimeRangeInput {
                start: end(),
                end: start()
            })
            .is_err()
        );
    }

    #[test]
    fn display_matches_value() {
        let r = TimeRange::new(TimeRangeInput {
            start: start(),
            end: end(),
        })
        .unwrap();
        assert_eq!(r.to_string(), r.value().to_owned());
    }

    #[test]
    fn contains_inside() {
        let r = TimeRange::new(TimeRangeInput { start: start(), end: end() }).unwrap();
        let mid = Utc.with_ymd_and_hms(2025, 1, 1, 11, 0, 0).unwrap();
        assert!(r.contains(&mid));
    }

    #[test]
    fn contains_at_start_inclusive() {
        let r = TimeRange::new(TimeRangeInput { start: start(), end: end() }).unwrap();
        assert!(r.contains(&start()));
    }

    #[test]
    fn contains_at_end_exclusive() {
        let r = TimeRange::new(TimeRangeInput { start: start(), end: end() }).unwrap();
        assert!(!r.contains(&end()));
    }

    #[test]
    fn contains_outside() {
        let r = TimeRange::new(TimeRangeInput { start: start(), end: end() }).unwrap();
        let before = Utc.with_ymd_and_hms(2025, 1, 1, 9, 0, 0).unwrap();
        assert!(!r.contains(&before));
    }

    #[test]
    fn overlaps_true() {
        let r1 = TimeRange::new(TimeRangeInput { start: start(), end: end() }).unwrap();
        let overlap_start = Utc.with_ymd_and_hms(2025, 1, 1, 11, 0, 0).unwrap();
        let overlap_end = Utc.with_ymd_and_hms(2025, 1, 1, 13, 0, 0).unwrap();
        let r2 = TimeRange::new(TimeRangeInput { start: overlap_start, end: overlap_end }).unwrap();
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn overlaps_adjacent_no_overlap() {
        let r1 = TimeRange::new(TimeRangeInput { start: start(), end: end() }).unwrap();
        let after_end = Utc.with_ymd_and_hms(2025, 1, 1, 13, 0, 0).unwrap();
        let r2 = TimeRange::new(TimeRangeInput { start: end(), end: after_end }).unwrap();
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn into_inner_roundtrip() {
        let input = TimeRangeInput {
            start: start(),
            end: end(),
        };
        let r = TimeRange::new(input.clone()).unwrap();
        assert_eq!(r.into_inner(), input);
    }
}
