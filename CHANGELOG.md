# Changelog

All notable changes to this project will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [1.0.0] — 2026-04-23

### Breaking

- `ValueObject` trait split into two traits: `ValueObject` (construction + destructuring) and `PrimitiveValue` (single-primitive accessor via `.value()`). Types backed by a primitive implement both; composite types implement only `ValueObject`.
- Removed the `sql` feature and all SQLx integration. Database mapping is now the responsibility of the application layer (see the ORM guide in `docs/`).

### Added

- `PrimitiveValue` trait with a `.value() -> &Primitive` accessor for types whose canonical form is a single primitive.
- `TryFrom<String>` and `TryFrom<&str>` implemented for all value objects — ergonomic construction without calling `::new()` explicitly.
- Serde deserialization now runs the full validation pipeline; deserializing an invalid value returns an error instead of producing an invalid object.
- Minimum `serde` version pinned to `1.0.116` (required for the `try_from` container attribute).

### Fixed

- Replaced `once_cell::sync::Lazy` with `std::sync::LazyLock` (stable since Rust 1.80); `once_cell` dependency removed.
- Stale `test-sql` CI job removed; `once_cell` removed from the `contact` feature declaration.

---

## [0.1.1] — 2026-04-17

### Added

- `CountryCode` value object — ISO 3166-1 alpha-2, normalised to uppercase
- `PhoneNumber` composite value object — `CountryCode` + local number → canonical E.164; ITU calling codes for all 249 ISO 3166-1 countries
- `EmailAddress::domain()` and `EmailAddress::local_part()` accessors
- `docs/` folder with reference documentation: `value-objects.md`, `implementing.md`, `contact.md`

### Changed

- `ValueObject` trait: `Raw` split into `Input` (accepted by `new()`) and `Output` (returned by `value()`), enabling composite types with structured input and canonical string output
- `EmailAddress` and `CountryCode` updated to use type aliases (`EmailAddressInput`/`Output`, `CountryCodeInput`/`Output`)
- Release workflow redesigned — version bump goes through PR, workflow only tags and publishes

---

## [0.1.0] — 2026-04-17

### Added

- `ValueObject` trait — core interface for all validated value types
- `ValidationError` enum with variants: `InvalidFormat`, `OutOfRange`, `Empty`, `Custom`
- `contact` feature with `EmailAddress` — validates, trims, and lowercases on construction
- `serde` feature — transparent `Serialize`/`Deserialize` for all types
- `full` meta-feature
- `prelude` module with convenience re-exports

[Unreleased]: https://github.com/codegress-com/arvo/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/codegress-com/arvo/compare/v0.1.1...v1.0.0
[0.1.1]: https://github.com/codegress-com/arvo/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/codegress-com/arvo/releases/tag/v0.1.0
