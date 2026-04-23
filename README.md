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
- [The trait hierarchy](#the-trait-hierarchy)
- [Error handling](#error-handling)
- [Parsing from strings](#parsing-from-strings)
- [Serde support](#serde-support)
- [Database / ORM integration](#database--orm-integration)
- [Roadmap](#roadmap)
- [Contributing](#contributing)

## Documentation

| Document | Description |
|---|---|
| [docs/value-objects.md](docs/value-objects.md) | What value objects are, simple vs composite, normalisation |
| [docs/implementing.md](docs/implementing.md) | How to implement the traits for custom types |
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
arvo = { version = "1.0", features = ["contact", "serde"] }
```

Enable only the modules you need — unused features add zero dependencies.

---

## Feature flags

| Feature | What you get | Extra deps |
|:---|:---|:---|
| `contact` | `EmailAddress`, `CountryCode`, `PhoneNumber`, `PostalAddress`, `Website` | `regex`, `url` |
| `finance` | `Money`, `CurrencyCode`, `Iban`, `Bic`, `VatNumber`, `Percentage`, `ExchangeRate`, `CreditCardNumber`, `CardExpiryDate` | `rust_decimal`, `chrono` |
| `geo` | `Latitude`, `Longitude`, `Coordinate`, `BoundingBox`, `TimeZone`, `CountryRegion` | — |
| `measurement` | `Length`, `Weight`, `Temperature`, `Volume`, `Area`, `Speed`, `Pressure`, `Energy`, `Power`, `Frequency` | — |
| `net` | `Url`, `Domain`, `IpV4Address`, `IpV6Address`, `IpAddress`, `Port`, `MacAddress`, `MimeType`, `HttpStatusCode`, `ApiKey` | `url` |
| `identifiers` | `Slug`, `Ean13`, `Ean8`, `Isbn13`, `Isbn10`, `Issn`, `Vin` | — |
| `primitives` | `NonEmptyString`, `BoundedString`, `PositiveInt`, `NonNegativeInt`, `PositiveDecimal`, `NonNegativeDecimal`, `Probability`, `HexColor`, `Locale`, `Base64String` | `rust_decimal`, `base64` |
| `temporal` | `UnixTimestamp`, `BirthDate`, `ExpiryDate`, `TimeRange`, `BusinessHours` | `chrono` |
| `serde` | `Serialize` / `Deserialize` on all types — deserialisation validates | `serde` |
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

// Composite value object — structured input, multiple accessors
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

## The trait hierarchy

arvo uses two traits:

```rust,ignore
// Base trait — all value objects
pub trait ValueObject: Sized + Clone + PartialEq {
    type Input;
    type Error: std::error::Error;

    fn new(value: Self::Input) -> Result<Self, Self::Error>;
    fn into_inner(self) -> Self::Input;
}

// Subtrait — simple single-primitive newtypes only
pub trait PrimitiveValue: ValueObject {
    type Primitive: ?Sized;
    fn value(&self) -> &Self::Primitive;
}
```

**Simple types** implement both — `value()` returns the inner primitive:
```rust,ignore
let email = EmailAddress::new("user@example.com".into())?;
email.value()       // &String → "user@example.com"
email.into_inner()  // String  → "user@example.com"
```

**Composite types** implement only `ValueObject` — data is accessed through dedicated methods:
```rust,ignore
let phone = PhoneNumber::new(PhoneNumberInput { country_code, number })?;
phone.value()        // &str → "+420123456789"  (inherent method, not trait)
phone.calling_code() // &str → "+420"
phone.into_inner()   // PhoneNumberInput { country_code, number }
```

Use `PrimitiveValue` as a generic bound when you need access to the inner value:
```rust,ignore
fn print_value<T: PrimitiveValue<Primitive = str>>(v: &T) {
    println!("{}", v.value());
}
```

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

Enable the `serde` feature. All types serialize as their raw primitive and **deserialisation validates** — invalid values are rejected at parse time:

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

---

## Database / ORM integration

arvo intentionally has no database dependency. Integrate using the accessors arvo provides — this works with any ORM and enables multi-column storage for composite types:

**Raw sqlx — simple types:**
```rust,ignore
// Bind — extract the primitive
query.bind(email.value())
query.bind(country.value())

// Read back — construct via new()
let s: String = row.get("email");
let email = EmailAddress::new(s)?;
```

**SeaORM / Diesel — composite types as multiple columns:**
```rust,ignore
// Define your own entity with individual columns
#[derive(DeriveEntityModel)]
pub struct Model {
    pub street: String, pub city: String,
    pub zip: String,    pub country: String,
}

// Convert via into_inner()
impl From<PostalAddress> for Model {
    fn from(addr: PostalAddress) -> Self {
        let i = addr.into_inner();
        Model { street: i.street, city: i.city,
                zip: i.zip, country: i.country.into_inner() }
    }
}
```

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

## Migration from 0.x to 1.0

| What changed | Migration |
|---|---|
| `ValueObject::value()` moved to `PrimitiveValue` | Change `T: ValueObject` to `T: PrimitiveValue` if you call `.value()` generically |
| `type Output` removed from `ValueObject` | Replace `<T as ValueObject>::Output` with the concrete type |
| `XxxOutput` type aliases removed | Replace `EmailAddressOutput` with `String`, `PortOutput` with `u16`, etc. |
| `sql` feature removed | Use `.value()` / `.into_inner()` to bind primitives; implement sqlx traits yourself if needed |

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
