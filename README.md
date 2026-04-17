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
- [Roadmap](#roadmap)
- [Contributing](#contributing)

---

## Installation

```toml
[dependencies]
arvo = { version = "0.1", features = ["contact", "serde"] }
```

Enable only the modules you need — unused features add zero dependencies.

---

## Feature flags

| Feature | What you get | Extra deps |
|:---|:---|:---|
| `contact` | `EmailAddress` | `once_cell`, `regex` |
| `serde` | `Serialize` / `Deserialize` on all types | `serde` |
| `full` | All domain modules | all of the above |

> **Tip:** `serde` and `full` are orthogonal — combine them freely:
> `features = ["full", "serde"]`

---

## Quick start

```rust,ignore
use arvo::prelude::*;

// Validates and normalises on construction
let email = EmailAddress::new("User@Example.COM".into())?;
assert_eq!(email.value(), "user@example.com");    // always lowercase

// Ergonomic try_into from &str
let email: EmailAddress = "hello@example.com".try_into()?;

// Display shows the normalised value
println!("{email}");  // hello@example.com

// Invalid input → descriptive error, not a panic
let err = EmailAddress::new("not-an-email".into()).unwrap_err();
println!("{err}");    // 'not-an-email' is not a valid EmailAddress
```

---

## The `ValueObject` trait

Every type in arvo implements the same core interface:

```rust,ignore
pub trait ValueObject: Sized + Clone + PartialEq {
    type Raw;
    type Error: std::error::Error;

    /// Only way to construct — validates the raw value.
    fn new(value: Self::Raw) -> Result<Self, Self::Error>;

    /// Borrow the validated inner value.
    fn value(&self) -> &Self::Raw;

    /// Consume and unwrap.
    fn into_inner(self) -> Self::Raw;
}
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

## Serde support

Enable the `serde` feature. All types serialize as their raw primitive (transparent newtype):

```rust,ignore
use arvo::contact::EmailAddress;

let email = EmailAddress::new("user@example.com".into())?;

let json = serde_json::to_string(&email)?;
// → "\"user@example.com\""

// Deserialization validates — invalid JSON values are rejected at parse time
let parsed: EmailAddress = serde_json::from_str(r#""hello@example.com""#)?;
```

---

## Roadmap

62 value object types planned across 8 domain modules. Types are only added when they bring validation, normalisation, or domain semantics that existing crates don't already provide.

| Feature | Highlights | Types | Status |
|:---|:---|:---:|:---:|
| `contact` | `EmailAddress`, `PhoneNumber`, `CountryCode`, `PostalAddress` | 5 | 1 / 5 |
| `identifiers` | `Slug`, `Ean13`, `Isbn13`, `Vin` | 7 | 0 / 7 |
| `finance` | `Money`, `Iban`, `Bic`, `VatNumber`, `CreditCardNumber` | 9 | 0 / 9 |
| `temporal` | `BirthDate`, `ExpiryDate`, `TimeRange`, `BusinessHours` | 5 | 0 / 5 |
| `geo` | `Latitude`, `Longitude`, `Coordinate`, `BoundingBox`, `TimeZone` | 6 | 0 / 6 |
| `net` | `Url`, `IpAddress`, `MacAddress`, `ApiKey`, `Port` | 10 | 0 / 10 |
| `measurement` | `Length`, `Weight`, `Temperature`, `Speed` ⚠️ needs unit conversion design | 10 | 0 / 10 |
| `primitives` | `NonEmptyString`, `BoundedString`, `Locale`, `HexColor` | 10 | 0 / 10 |

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
