# temporal module

Feature flag: `temporal`

```toml
[dependencies]
arvo = { version = "0.6", features = ["temporal"] }
```

---

## UnixTimestamp

A validated Unix timestamp — non-negative seconds since the Unix epoch (1970-01-01T00:00:00Z).

**Normalisation:** none.  
**Validation:** must be `>= 0`.

```rust,ignore
use arvo::temporal::UnixTimestamp;
use arvo::traits::ValueObject;

let ts = UnixTimestamp::new(1_700_000_000).unwrap();
assert_eq!(*ts.value(), 1_700_000_000);

assert!(UnixTimestamp::new(-1).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&i64` | `1700000000` |
| `into_inner()` | `i64` | `1700000000` |

---

## BirthDate

A validated date of birth — strictly in the past and no more than 150 years ago.

**Normalisation:** none.  
**Input / Output:** `chrono::NaiveDate`.

```rust,ignore
use arvo::temporal::BirthDate;
use arvo::traits::ValueObject;
use chrono::NaiveDate;

let dob = BirthDate::new(NaiveDate::from_ymd_opt(1990, 6, 15).unwrap()).unwrap();
assert_eq!(dob.value().year(), 1990);
assert!(dob.age_years() > 0);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&NaiveDate` | `1990-06-15` |
| `age_years()` | `u32` | `35` |
| `into_inner()` | `NaiveDate` | — |

### Errors

| Condition | Error |
|---|---|
| date is today or in the future | `ValidationError::InvalidFormat` |
| date is more than 150 years ago | `ValidationError::InvalidFormat` |

---

## ExpiryDate

A validated expiry date — strictly in the future at construction time.

**Normalisation:** none.  
**Input / Output:** `chrono::NaiveDate`.

```rust,ignore
use arvo::temporal::ExpiryDate;
use arvo::traits::ValueObject;
use chrono::NaiveDate;

let exp = ExpiryDate::new(NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()).unwrap();
assert!(exp.days_until() > 0);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&NaiveDate` | `2030-12-31` |
| `days_until()` | `i64` | `1715` |
| `into_inner()` | `NaiveDate` | — |

### Errors

| Condition | Error |
|---|---|
| date is today or in the past | `ValidationError::InvalidFormat` |

---

## TimeRange

A validated time range — `start` must be strictly before `end`.

**Normalisation:** none.  
**Input:** `TimeRangeInput { start: DateTime<Utc>, end: DateTime<Utc> }`.  
**Output:** canonical `"<start> / <end>"` string.

```rust,ignore
use arvo::temporal::{TimeRange, TimeRangeInput};
use arvo::traits::ValueObject;
use chrono::{TimeZone, Utc};

let range = TimeRange::new(TimeRangeInput {
    start: Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap(),
    end:   Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap(),
})?;
assert_eq!(range.duration().num_hours(), 2);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"2025-01-01 10:00:00 UTC / 2025-01-01 12:00:00 UTC"` |
| `start()` | `&DateTime<Utc>` | — |
| `end()` | `&DateTime<Utc>` | — |
| `duration()` | `Duration` | `2h` |
| `into_inner()` | `TimeRangeInput` | — |

### Errors

| Condition | Error |
|---|---|
| `start >= end` | `ValidationError::InvalidFormat` |

---

## BusinessHours

Validated business hours for a single weekday — `open` must be strictly before `close`.

**Normalisation:** none.  
**Input:** `BusinessHoursInput { weekday: Weekday, open: NaiveTime, close: NaiveTime }`.  
**Output:** canonical `"<Day> HH:MM–HH:MM"` string.

```rust,ignore
use arvo::temporal::{BusinessHours, BusinessHoursInput};
use arvo::traits::ValueObject;
use chrono::{NaiveTime, Weekday};

let hours = BusinessHours::new(BusinessHoursInput {
    weekday: Weekday::Mon,
    open:    NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
    close:   NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
})?;
assert_eq!(hours.value(), "Mon 09:00–17:00");
assert_eq!(hours.duration().num_hours(), 8);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"Mon 09:00–17:00"` |
| `weekday()` | `Weekday` | `Weekday::Mon` |
| `open()` | `&NaiveTime` | `09:00` |
| `close()` | `&NaiveTime` | `17:00` |
| `duration()` | `Duration` | `8h` |
| `into_inner()` | `BusinessHoursInput` | — |

### Errors

| Condition | Error |
|---|---|
| `open >= close` | `ValidationError::InvalidFormat` |
