<div align="center">

# arvo

**Validated, immutable value objects for common domain types**

*arvo* — Finnish for *value*

[![Crates.io](https://img.shields.io/crates/v/arvo?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/arvo)
[![docs.rs](https://img.shields.io/docsrs/arvo?style=flat-square&logo=docs.rs)](https://docs.rs/arvo)
[![CI](https://img.shields.io/github/actions/workflow/status/codegress-com/arvo/ci.yml?branch=main&style=flat-square&logo=github&label=CI)](https://github.com/codegress-com/arvo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)
[![MSRV: 1.85](https://img.shields.io/badge/MSRV-1.85-purple?style=flat-square&logo=rust)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)

</div>

---

Each type in **arvo** carries a single guarantee: **if it exists, it is valid.**

Construction always goes through `::new()` returning `Result` — invalid states become unrepresentable at the type level. No more stringly-typed domain values, no runtime surprises.

```rust,ignore
// This compiles. This is guaranteed valid. Forever.
let email: EmailAddress = "user@example.com".try_into()?;
```

---

## Contents

- [Installation](#installation)
- [Feature flags](#feature-flags)
- [Quick start](#quick-start)
- [The `ValueObject` trait](#the-valueobject-trait)
- [Error handling](#error-handling)
- [Serde support](#serde-support)
- [SQL support](#sql-support)
- [Roadmap](#roadmap)
- [Contributing](#contributing)

## Documentation

| Document | Description |
|---|---|
| [docs/value-objects.md](docs/value-objects.md) | What value objects are, simple vs composite, normalisation |
| [docs/implementing.md](docs/implementing.md) | How to implement the `ValueObject` trait for custom types |
| [docs/contact.md](docs/contact.md) | Reference for all `contact` module types |
| [docs/finance.md](docs/finance.md) | Reference for all `finance` module types |
| [docs/geo.md](docs/geo.md) | Reference for all `geo` module types |
| [docs/measurement.md](docs/measurement.md) | Reference for all `measurement` module types |
| [docs/net.md](docs/net.md) | Reference for all `net` module types |
| [docs/identifiers.md](docs/identifiers.md) | Reference for all `identifiers` module types |
| [docs/primitives.md](docs/primitives.md) | Reference for all `primitives` module types |
| [docs/temporal.md](docs/temporal.md) | Reference for all `temporal` module types |

---

## Installation

```toml
[dependencies]
arvo = { version = "0.9", features = ["contact", "serde"] }
```

Enable only the modules you need — unused features add zero dependencies.

---

## Feature flags

| Feature | What you get | Extra deps |
|:---|:---|:---|
| `contact` | `EmailAddress`, `CountryCode`, `PhoneNumber`, `PostalAddress`, `Website` | `once_cell`, `regex`, `url` |
| `finance` | `Money`, `CurrencyCode`, `Iban`, `Bic`, `VatNumber`, `Percentage`, `ExchangeRate`, `CreditCardNumber`, `CardExpiryDate` | `rust_decimal`, `chrono` |
| `geo` | `Latitude`, `Longitude`, `Coordinate`, `BoundingBox`, `TimeZone`, `CountryRegion` | — |
| `measurement` | `Length`, `Weight`, `Temperature`, `Volume`, `Area`, `Speed`, `Pressure`, `Energy`, `Power`, `Frequency` | — |
| `net` | `Url`, `Domain`, `IpV4Address`, `IpV6Address`, `IpAddress`, `Port`, `MacAddress`, `MimeType`, `HttpStatusCode`, `ApiKey` | `url` |
| `identifiers` | `Slug`, `Ean13`, `Ean8`, `Isbn13`, `Isbn10`, `Issn`, `Vin` | — |
| `primitives` | `NonEmptyString`, `BoundedString`, `PositiveInt`, `NonNegativeInt`, `PositiveDecimal`, `NonNegativeDecimal`, `Probability`, `HexColor`, `Locale`, `Base64String` | `rust_decimal`, `base64` |
| `temporal` | `UnixTimestamp`, `BirthDate`, `ExpiryDate`, `TimeRange`, `BusinessHours` | `chrono` |
| `serde` | `Serialize` / `Deserialize` on all types | `serde` |
| `sql` | `sqlx::Type` + `Encode` + `Decode` for PostgreSQL on all types | `sqlx` |
| `full` | All domain modules | all of the above |

> **Tip:** `serde` and `full` are orthogonal — combine them freely:
> `features = ["full", "serde"]`

---

## Quick start

```rust,ignore
use arvo::contact::{CountryCode, PhoneNumber, PhoneNumberInput};
use arvo::prelude::*;

// Construct via new() — validates and normalises on construction
let email = EmailAddress::new("User@Example.COM".into())?;
assert_eq!(email.value(), "user@example.com");  // always lowercase
assert_eq!(email.domain(), "example.com");

// Ergonomic try_into from &str
let email: EmailAddress = "hello@example.com".try_into()?;

// Country code — normalised to uppercase, ISO 3166-1 alpha-2
let country = CountryCode::new("cz".into())?;
assert_eq!(country.value(), "CZ");

// Composite value object — structured input, canonical E.164 output
let phone = PhoneNumber::new(PhoneNumberInput {
    country_code: CountryCode::new("CZ".into())?,
    number: "123 456 789".into(),   // formatting stripped automatically
})?;
assert_eq!(phone.value(), "+420123456789");
assert_eq!(phone.calling_code(), "+420");

// Invalid input → descriptive error, not a panic
let err = EmailAddress::new("not-an-email".into()).unwrap_err();
println!("{err}");  // 'not-an-email' is not a valid EmailAddress
```

---

## The `ValueObject` trait

Every type in arvo implements the same core interface:

```rust,ignore
pub trait ValueObject: Sized + Clone + PartialEq {
    /// What `new()` accepts — raw primitive for simple types,
    /// a dedicated input struct for composites.
    type Input;

    /// What `value()` returns — same as `Input` for simple types,
    /// canonical representation (e.g. E.164 string) for composites.
    type Output: ?Sized;

    type Error: std::error::Error;

    /// Only way to construct — validates and normalises the input.
    fn new(value: Self::Input) -> Result<Self, Self::Error>;

    /// Returns the validated output value.
    fn value(&self) -> &Self::Output;

    /// Consumes and returns the original input.
    fn into_inner(self) -> Self::Input;
}
```

**Simple type** — `Input` and `Output` are the same (`String`):
```rust,ignore
let email = EmailAddress::new("user@example.com".into())?;
email.value()       // &String → "user@example.com"
email.into_inner()  // String  → "user@example.com"
```

**Composite type** — `Input` is a struct, `Output` is canonical string:
```rust,ignore
let phone = PhoneNumber::new(PhoneNumberInput { country_code, number })?;
phone.value()       // &String → "+420123456789"  (E.164)
phone.into_inner()  // PhoneNumberInput { country_code, number }
```

You can implement it for your own domain types using the provided implementations as a reference.

---

## Error handling

All validation errors are variants of `ValidationError`:

```rust,ignore
use arvo::errors::ValidationError;

match EmailAddress::new("bad".into()) {
    Ok(email)  => println!("valid: {email}"),
    Err(ValidationError::InvalidFormat { type_name, value }) => {
        eprintln!("'{value}' is not a valid {type_name}");
    }
    Err(ValidationError::Empty { type_name }) => {
        eprintln!("{type_name} must not be empty");
    }
    Err(e) => eprintln!("{e}"),
}
```

---

## Parsing from strings

Every type implements `TryFrom<&str>` (and therefore `.try_into()`) that parses the canonical string representation and validates in one step:

```rust,ignore
// Simple types parse their primitive value
let lat: Latitude       = "48.8588".try_into()?;
let port: Port          = "8080".try_into()?;
let ts: UnixTimestamp   = "1700000000".try_into()?;
let dob: BirthDate      = "1990-06-15".try_into()?;

// Composite types parse their canonical string format
let money: Money        = "10.50 EUR".try_into()?;
let rate: ExchangeRate  = "EUR/USD 1.0850".try_into()?;
let coord: Coordinate   = "48.858844, 2.294351".try_into()?;
let len: Length         = "1.5 km".try_into()?;
let range: TimeRange    = "2025-01-01 10:00:00 UTC / 2025-01-01 12:00:00 UTC".try_into()?;
let hours: BusinessHours = "Mon 09:00–17:00".try_into()?;
```

Parsing errors return `ValidationError` just like `::new()`.

> **Note:** `PhoneNumber` and `PostalAddress` do not implement `TryFrom<&str>` — their
> canonical strings are not unambiguously reversible back to a structured input.

---

## Serde support

Enable the `serde` feature. All types serialize as their raw primitive and **deserialisation validates** — invalid values are rejected at parse time, not after:

```rust,ignore
use arvo::contact::EmailAddress;

let email = EmailAddress::new("user@example.com".into())?;

let json = serde_json::to_string(&email)?;
// → "\"user@example.com\""

// Deserialisation goes through new() — domain validation is enforced
let parsed: EmailAddress = serde_json::from_str(r#""hello@example.com""#)?;

// Invalid values are rejected at parse time
let err: Result<EmailAddress, _> = serde_json::from_str(r#""not-an-email""#);
assert!(err.is_err());
```

Composite types (`PostalAddress`) serialise as their structured `Input` type (JSON object).  
`PhoneNumber` and `PostalAddress` do not support `TryFrom<&str>`, so no canonical-string round-trip is attempted.

---

## SQL support

Enable the `sql` feature. All types implement `sqlx::Type`, `sqlx::Encode`, and `sqlx::Decode` for PostgreSQL:

```toml
arvo = { version = "0.9", features = ["finance", "sql"] }
```

```rust,ignore
// Use arvo types directly in sqlx queries
let money: Money = sqlx::query_scalar("SELECT price FROM products WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await?;

// Bind arvo types as query parameters
sqlx::query("INSERT INTO orders (amount) VALUES ($1)")
    .bind(Money::new(MoneyInput { amount: "9.99".parse()?, currency })?)
    .execute(&pool)
    .await?;
```

**Storage mapping:**

| Type category | PostgreSQL type |
|:---|:---|
| `String`-based newtypes (`EmailAddress`, `Iban`, `Slug`, …) | `TEXT` |
| `f64` newtypes (`Latitude`, `Longitude`, `Percentage`, …) | `FLOAT8` |
| `i64` newtypes (`PositiveInt`, `UnixTimestamp`, …) | `INT8` |
| `Decimal` newtypes (`PositiveDecimal`, `NonNegativeDecimal`) | `NUMERIC` |
| `NaiveDate` newtypes (`BirthDate`, `ExpiryDate`) | `DATE` |
| `Port`, `HttpStatusCode` (u16) | `INT4` |
| Composite types (`Money`, `Coordinate`, `Length`, …) | `TEXT` (canonical string) |

> **Note:** `PhoneNumber` and `PostalAddress` do not implement sqlx traits —
> their canonical strings cannot be unambiguously decoded back to a structured value.

---

## Roadmap

62 value object types planned across 8 domain modules. Types are only added when they bring validation, normalisation, or domain semantics that existing crates don't already provide.

| Feature | Highlights | Types | Status |
|:---|:---|:---:|:---:|
| `contact` | `EmailAddress`, `PhoneNumber`, `CountryCode`, `PostalAddress`, `Website` | 5 | 5 / 5 ✅ |
| `identifiers` | `Slug`, `Ean13`, `Isbn13`, `Vin` | 7 | 7 / 7 ✅ |
| `finance` | `Money`, `CurrencyCode`, `Iban`, `Bic`, `VatNumber`, `Percentage`, `ExchangeRate`, `CreditCardNumber`, `CardExpiryDate` | 9 | 9 / 9 ✅ |
| `temporal` | `UnixTimestamp`, `BirthDate`, `ExpiryDate`, `TimeRange`, `BusinessHours` | 5 | 5 / 5 ✅ |
| `geo` | `Latitude`, `Longitude`, `Coordinate`, `BoundingBox`, `TimeZone`, `CountryRegion` | 6 | 6 / 6 ✅ |
| `net` | `Url`, `Domain`, `IpV4Address`, `IpV6Address`, `IpAddress`, `Port`, `MacAddress`, `MimeType`, `HttpStatusCode`, `ApiKey` | 10 | 10 / 10 ✅ |
| `measurement` | `Length`, `Weight`, `Temperature`, `Volume`, `Area`, `Speed`, `Pressure`, `Energy`, `Power`, `Frequency` | 10 | 10 / 10 ✅ |
| `primitives` | `NonEmptyString`, `BoundedString`, `Locale`, `HexColor` | 10 | 10 / 10 ✅ |

→ Full details and design rationale in [ROADMAP.md](ROADMAP.md)

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

- **Bug?** → [open a bug report](https://github.com/codegress-com/arvo/issues/new?template=bug_report.yml)
- **Feature idea?** → [open a feature request](https://github.com/codegress-com/arvo/issues/new?template=feature_request.yml)
- **Security issue?** → see [SECURITY.md](SECURITY.md) — do **not** open a public issue

---

<div align="center">

MIT License — © [Codegress](https://github.com/codegress-com)

</div>
