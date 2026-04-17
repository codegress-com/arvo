# arvo — roadmap

> Legend: ✅ done · 🚧 in progress · ⬜ planned

---

## `contact` feature

| Type | Status | Notes |
|---|---|---|
| `EmailAddress` | ✅ | normalised to lowercase, regex validated |
| `PhoneNumber` | ⬜ | E.164, strips spaces/dashes |
| `PersonName` | ⬜ | trimmed, 1–100 chars |
| `FullName` | ⬜ | composite: first `PersonName` + last `PersonName` |
| `CompanyName` | ⬜ | trimmed, 1–200 chars |
| `StreetAddress` | ⬜ | street + house number, trimmed |
| `City` | ⬜ | trimmed, non-empty |
| `ZipCode` | ⬜ | strips whitespace, max 10 chars |
| `CountryCode` | ⬜ | ISO 3166-1 alpha-2, normalised to uppercase |
| `PostalAddress` | ⬜ | composite: `StreetAddress` + `City` + `ZipCode` + `CountryCode` |
| `Website` | ⬜ | valid URL, https preferred |

---

## `identifiers` feature

| Type | Status | Notes |
|---|---|---|
| `Uuid` | ⬜ | RFC 4122, wraps `uuid` crate |
| `Ulid` | ⬜ | lexicographically sortable, wraps `ulid` crate |
| `Slug` | ⬜ | lowercase, alphanumeric + hyphens, no leading/trailing hyphens |
| `Username` | ⬜ | alphanumeric + underscore, 3–32 chars |
| `Sku` | ⬜ | Stock Keeping Unit, non-empty, normalised to uppercase |
| `Ean13` | ⬜ | EAN-13 barcode with checksum validation |
| `Ean8` | ⬜ | EAN-8 barcode with checksum validation |
| `Isbn13` | ⬜ | ISBN-13 with check digit |
| `Isbn10` | ⬜ | ISBN-10 with check digit |
| `Issn` | ⬜ | ISSN with check digit |
| `Vin` | ⬜ | Vehicle Identification Number, 17 chars, checksum validated |
| `CorrelationId` | ⬜ | non-empty string for request tracing; accepts UUID or opaque string |

---

## `finance` feature

| Type | Status | Notes |
|---|---|---|
| `Money` | ⬜ | `Decimal` amount + `CurrencyCode`; immutable arithmetic helpers |
| `CurrencyCode` | ⬜ | ISO 4217 alpha-3 (EUR, USD, CZK…) |
| `Iban` | ⬜ | IBAN with mod-97 checksum |
| `Bic` | ⬜ | BIC/SWIFT, 8 or 11 chars |
| `VatNumber` | ⬜ | EU VAT number with country-prefix + format validation |
| `Percentage` | ⬜ | `Decimal` in range 0–100 |
| `TaxRate` | ⬜ | `Decimal` in range 0–100; semantic alias for `Percentage` |
| `ExchangeRate` | ⬜ | positive `Decimal`, from/to currency pair |
| `CreditCardNumber` | ⬜ | Luhn algorithm validation; masked `Display` (shows only last 4 digits) |
| `CardExpiryDate` | ⬜ | MM/YY; rejected if in the past at construction time |

---

## `temporal` feature

| Type | Status | Notes |
|---|---|---|
| `Date` | ⬜ | calendar date without timezone, wraps `chrono::NaiveDate` |
| `Time` | ⬜ | time of day without timezone, wraps `chrono::NaiveTime` |
| `DateTime` | ⬜ | date + time with timezone, wraps `chrono::DateTime<Utc>` |
| `UnixTimestamp` | ⬜ | non-negative `i64` seconds since epoch |
| `Duration` | ⬜ | positive duration, wraps `chrono::Duration` |
| `TimeRange` | ⬜ | composite: start `DateTime` + end `DateTime`; start < end enforced |
| `BirthDate` | ⬜ | `Date` in the past, not more than 150 years ago |
| `ExpiryDate` | ⬜ | `Date` strictly in the future |
| `Age` | ⬜ | `u8` in range 0–150 |
| `Year` | ⬜ | `i32` in reasonable range (1000–9999) |
| `MonthOfYear` | ⬜ | `u8` in range 1–12 |
| `DayOfMonth` | ⬜ | `u8` in range 1–31 |
| `BusinessHours` | ⬜ | composite: weekday + open `Time` + close `Time`; open < close |

---

## `geo` feature

| Type | Status | Notes |
|---|---|---|
| `Latitude` | ⬜ | `f64` in range −90.0..=90.0 |
| `Longitude` | ⬜ | `f64` in range −180.0..=180.0 |
| `Coordinate` | ⬜ | composite: `Latitude` + `Longitude` |
| `Altitude` | ⬜ | metres above sea level, `f64` |
| `Angle` | ⬜ | `f64` in range 0.0..360.0 (degrees); useful for bearing/heading |
| `Distance` | ⬜ | non-negative `f64` with unit (m, km, mi) |
| `BoundingBox` | ⬜ | composite: SW `Coordinate` + NE `Coordinate` |
| `TimeZone` | ⬜ | IANA timezone name (e.g. `Europe/Prague`) |
| `CountryRegion` | ⬜ | ISO 3166-2 subdivision code (e.g. `CZ-PR`) |

---

## `net` feature

| Type | Status | Notes |
|---|---|---|
| `Url` | ⬜ | valid URL, wraps `url` crate |
| `Domain` | ⬜ | valid domain name without scheme |
| `IpV4Address` | ⬜ | valid IPv4 (e.g. `192.168.1.1`) |
| `IpV6Address` | ⬜ | valid IPv6 |
| `IpAddress` | ⬜ | enum: `V4(IpV4Address)` \| `V6(IpV6Address)` |
| `Port` | ⬜ | `u16` in range 1–65535 |
| `MacAddress` | ⬜ | 6-byte MAC, normalised to lowercase colon-separated hex |
| `MimeType` | ⬜ | valid MIME type (e.g. `image/png`) |
| `HttpStatusCode` | ⬜ | `u16` in range 100–599 |
| `ApiKey` | ⬜ | non-empty; masked `Display` shows only last 4 chars |

---

## `measurement` feature

| Type | Status | Notes |
|---|---|---|
| `Length` | ⬜ | non-negative `f64` with unit (m, cm, mm, in, ft) |
| `Weight` | ⬜ | non-negative `f64` with unit (kg, g, lb, oz) |
| `Temperature` | ⬜ | `f64` with unit (°C, °F, K); Kelvin must be ≥ 0 |
| `Volume` | ⬜ | non-negative `f64` with unit (l, ml, m³, fl oz) |
| `Area` | ⬜ | non-negative `f64` with unit (m², cm², ft²) |
| `Speed` | ⬜ | non-negative `f64` with unit (m/s, km/h, mph) |
| `Pressure` | ⬜ | non-negative `f64` with unit (Pa, bar, psi) |
| `Energy` | ⬜ | non-negative `f64` with unit (J, kWh, cal) |
| `Power` | ⬜ | non-negative `f64` with unit (W, kW, hp) |
| `Frequency` | ⬜ | positive `f64` with unit (Hz, kHz, MHz) |

---

## `primitives` feature

| Type | Status | Notes |
|---|---|---|
| `NonEmptyString` | ⬜ | trimmed, at least 1 non-whitespace char |
| `TrimmedString` | ⬜ | leading/trailing whitespace stripped |
| `BoundedString` | ⬜ | `BoundedString<const MIN: usize, const MAX: usize>` via const generics |
| `PositiveInt` | ⬜ | `i64 > 0` |
| `NonNegativeInt` | ⬜ | `i64 >= 0` |
| `PositiveDecimal` | ⬜ | `Decimal > 0` |
| `NonNegativeDecimal` | ⬜ | `Decimal >= 0` |
| `Probability` | ⬜ | `f64` in range 0.0..=1.0 |
| `SemVer` | ⬜ | semantic version (major.minor.patch), wraps `semver` crate |
| `HexColor` | ⬜ | `#RRGGBB` or `#RGB`, normalised to uppercase |
| `RgbColor` | ⬜ | composite: r, g, b each `u8` |
| `Locale` | ⬜ | BCP 47 language tag (e.g. `en-US`, `cs-CZ`) |
| `LanguageCode` | ⬜ | ISO 639-1 alpha-2 (e.g. `en`, `cs`) |
| `Base64String` | ⬜ | valid base64-encoded string |
| `FilePath` | ⬜ | non-empty, OS-agnostic path validation |
| `FileName` | ⬜ | no path separators, non-empty |

---

## Summary

| Feature | Total | Done | Remaining |
|---|---|---|---|
| `contact` | 11 | 1 | 10 |
| `identifiers` | 12 | 0 | 12 |
| `finance` | 10 | 0 | 10 |
| `temporal` | 13 | 0 | 13 |
| `geo` | 9 | 0 | 9 |
| `net` | 10 | 0 | 10 |
| `measurement` | 10 | 0 | 10 |
| `primitives` | 16 | 0 | 16 |
| **Total** | **91** | **1** | **90** |
