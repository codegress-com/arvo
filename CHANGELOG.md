# Changelog

All notable changes to this project will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- `CountryCode` value object — ISO 3166-1 alpha-2, normalised to uppercase
- `PhoneNumber` composite value object — `CountryCode` + local number → canonical E.164; ITU calling codes for all 249 ISO 3166-1 countries
- `EmailAddress::domain()` and `EmailAddress::local_part()` accessors

### Changed

- `ValueObject` trait: `Raw` split into `Input` (accepted by `new()`) and `Output` (returned by `value()`), enabling composite types with structured input and canonical string output
- `EmailAddress` and `CountryCode` updated to use `EmailAddressInput`/`Output` and `CountryCodeInput`/`Output` type aliases

---

## [0.1.0] — 2026-04-17

### Added

- `ValueObject` trait — core interface for all validated value types
- `ValidationError` enum with variants: `InvalidFormat`, `OutOfRange`, `Empty`, `Custom`
- `contact` feature with `EmailAddress` — validates, trims, and lowercases on construction
- `serde` feature — transparent `Serialize`/`Deserialize` for all types
- `full` meta-feature
- `prelude` module with convenience re-exports

[Unreleased]: https://github.com/codegress-com/arvo/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/codegress-com/arvo/releases/tag/v0.1.0
