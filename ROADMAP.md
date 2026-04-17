# arvo — roadmap

> Legend: ✅ done · 🚧 in progress · ⬜ planned

**Design rule:** arvo only implements a type when it adds validation, normalisation, or domain semantics that the raw type or an existing well-maintained crate does not already provide.

---

## `contact` feature

| Type | Status | Notes |
|---|---|---|
| `EmailAddress` | ✅ | normalised to lowercase, regex validated |
| `PhoneNumber` | ⬜ | E.164, strips spaces/dashes |
| `CountryCode` | ⬜ | ISO 3166-1 alpha-2, normalised to uppercase |
| `PostalAddress` | ⬜ | composite: street + city + zip + `CountryCode`; fields validated as non-empty |
| `Website` | ⬜ | valid URL, https preferred |

---

## `identifiers` feature

*`Uuid` and `Ulid` are intentionally omitted — use the `uuid` and `ulid` crates directly.*

| Type | Status | Notes |
|---|---|---|
| `Slug` | ⬜ | lowercase, alphanumeric + hyphens, no leading/trailing hyphens |
| `Ean13` | ⬜ | EAN-13 barcode with checksum validation |
| `Ean8` | ⬜ | EAN-8 barcode with checksum validation |
| `Isbn13` | ⬜ | ISBN-13 with check digit |
| `Isbn10` | ⬜ | ISBN-10 with check digit |
| `Issn` | ⬜ | ISSN with check digit |
| `Vin` | ⬜ | Vehicle Identification Number, 17 chars, checksum validated |

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
| `ExchangeRate` | ⬜ | positive `Decimal`, from/to `CurrencyCode` pair |
| `CreditCardNumber` | ⬜ | Luhn algorithm validation; masked `Display` (shows only last 4 digits) |
| `CardExpiryDate` | ⬜ | MM/YY; rejected if in the past at construction time |

---

## `temporal` feature

*`Date`, `Time`, `DateTime`, and `Duration` are intentionally omitted — `chrono` and the `time` crate cover these. arvo adds types with domain-level semantics on top.*

| Type | Status | Notes |
|---|---|---|
| `UnixTimestamp` | ⬜ | non-negative `i64` seconds since epoch |
| `BirthDate` | ⬜ | date in the past, not more than 150 years ago |
| `ExpiryDate` | ⬜ | date strictly in the future |
| `TimeRange` | ⬜ | start + end `DateTime`; `start < end` enforced |
| `BusinessHours` | ⬜ | composite: weekday + open time + close time; open < close |

---

## `geo` feature

| Type | Status | Notes |
|---|---|---|
| `Latitude` | ⬜ | `f64` in range −90.0..=90.0 |
| `Longitude` | ⬜ | `f64` in range −180.0..=180.0 |
| `Coordinate` | ⬜ | composite: `Latitude` + `Longitude` |
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

> ⚠️ **Design required before implementation.** Each type carries a unit — but without unit conversion (e.g. `km → m`, `°F → °C`) the unit is just a label. The API design for conversions must be settled first. Tracked in issue [#TODO].

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
| `BoundedString` | ⬜ | `BoundedString<const MIN: usize, const MAX: usize>` via const generics |
| `PositiveInt` | ⬜ | `i64 > 0` |
| `NonNegativeInt` | ⬜ | `i64 >= 0` |
| `PositiveDecimal` | ⬜ | `Decimal > 0` |
| `NonNegativeDecimal` | ⬜ | `Decimal >= 0` |
| `Probability` | ⬜ | `f64` in range 0.0..=1.0 |
| `HexColor` | ⬜ | `#RRGGBB` or `#RGB`, normalised to uppercase |
| `Locale` | ⬜ | BCP 47 language tag (e.g. `en-US`, `cs-CZ`) |
| `Base64String` | ⬜ | valid base64-encoded string |

---

## Summary

| Feature | Total | Done | Remaining |
|---|---|---|---|
| `contact` | 5 | 1 | 4 |
| `identifiers` | 7 | 0 | 7 |
| `finance` | 9 | 0 | 9 |
| `temporal` | 5 | 0 | 5 |
| `geo` | 6 | 0 | 6 |
| `net` | 10 | 0 | 10 |
| `measurement` | 10 | 0 | 10 |
| `primitives` | 10 | 0 | 10 |
| **Total** | **62** | **1** | **61** |
