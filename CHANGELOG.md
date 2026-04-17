# Changelog

All notable changes to this project will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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
