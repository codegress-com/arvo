# arvo

[![Crates.io](https://img.shields.io/crates/v/arvo.svg)](https://crates.io/crates/arvo)
[![docs.rs](https://img.shields.io/docsrs/arvo)](https://docs.rs/arvo)
[![CI](https://github.com/codegress-com/arvo/actions/workflows/ci.yml/badge.svg)](https://github.com/codegress-com/arvo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MSRV: 1.85](https://img.shields.io/badge/MSRV-1.85-orange.svg)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)

**arvo** (Finnish: *value*) — validated, immutable value objects for common domain types.

Each type guarantees: **if it exists, it is valid.** Construction always goes through `::new()` returning `Result`, making invalid states unrepresentable at the type level.

## Features

| Feature | Types | Activates |
|---|---|---|
| `contact` | `EmailAddress` | `once_cell`, `regex` |
| `serde` | `Serialize`/`Deserialize` on all types | `serde` |
| `sql` | sqlx `Type`/`Encode`/`Decode` for Postgres | `sqlx` |
| `full` | all domain modules | `contact` |

Enable only what you need — zero unused dependencies pulled in.

## Quick start

```toml
[dependencies]
arvo = { version = "0.1", features = ["contact", "serde"] }
```

```rust,ignore
use arvo::prelude::*;

// Construction validates and normalises
let email = EmailAddress::new("User@Example.COM".into())?;
assert_eq!(email.value(), "user@example.com");

// Try-from for ergonomic use
let email: EmailAddress = "hello@example.com".try_into()?;

// Display prints the normalised value
println!("{email}"); // hello@example.com
```

## The `ValueObject` trait

All types implement the same core trait:

```rust,ignore
pub trait ValueObject: Sized + Clone + PartialEq {
    type Raw;
    type Error: std::error::Error;

    fn new(value: Self::Raw) -> Result<Self, Self::Error>;
    fn value(&self) -> &Self::Raw;
    fn into_inner(self) -> Self::Raw;
}
```

Implement it for your own domain types or use the provided implementations as a reference.

## Errors

All validation errors are variants of `ValidationError`:

```rust,ignore
use arvo::errors::ValidationError;

match EmailAddress::new("bad".into()) {
    Err(ValidationError::InvalidFormat { type_name, value }) => { /* … */ }
    Err(ValidationError::Empty { type_name }) => { /* … */ }
    _ => {}
}
```

## Serde

Enable the `serde` feature — all types serialize as their raw value (transparent newtype):

```rust,ignore
let json = serde_json::to_string(&email)?;           // "user@example.com"
let email: EmailAddress = serde_json::from_str(r#""user@example.com""#)?;
```

Deserialization runs through `::new()`, so invalid values are rejected at parse time.

## Roadmap

- [ ] `finance` — `Money`, `Currency`, `Percentage`
- [ ] `identifiers` — `Uuid`, `Ulid`
- [ ] `net` — `Url`, `IpAddress`
- [ ] `temporal` — `DateOfBirth`, `FutureDate`
- [ ] `geo` — `Coordinates`, `PostalCode`
- [ ] `primitives` — `NonEmptyString`, `BoundedInt`

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
