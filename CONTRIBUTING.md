# Contributing to arvo

Thank you for your interest in contributing! This document covers everything you need to get started.

## Table of contents

- [Code of conduct](#code-of-conduct)
- [Ways to contribute](#ways-to-contribute)
- [Development setup](#development-setup)
- [Submitting a pull request](#submitting-a-pull-request)
- [Coding conventions](#coding-conventions)
- [Adding a new value object](#adding-a-new-value-object)
- [Release process](#release-process)

---

## Code of conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to uphold it. Report unacceptable behaviour to **hello@codegress.com**.

---

## Ways to contribute

| Type | Where |
|---|---|
| Bug report | [GitHub Issues — bug report template](.github/ISSUE_TEMPLATE/bug_report.yml) |
| Feature request | [GitHub Issues — feature request template](.github/ISSUE_TEMPLATE/feature_request.yml) |
| Documentation fix | Open a PR directly |
| New value object | Discuss in an issue first, then PR |
| Security vulnerability | **Do not open a public issue** — see [SECURITY.md](SECURITY.md) |

---

## Development setup

**Requirements:** Rust stable ≥ 1.85, Git.

```bash
git clone https://github.com/codegress-com/arvo.git
cd arvo

# Run all tests
cargo test --features full,serde

# Lint
cargo clippy --features full,serde -- -Dclippy::all

# Format
cargo fmt

# Build docs locally
cargo doc --open --features full,serde
```

For the `sql` feature you also need a local Postgres instance:

```bash
export DATABASE_URL=postgres://arvo:arvo@localhost:5432/arvo_test
cargo test --features sql
```

---

## Submitting a pull request

1. **Open an issue first** for non-trivial changes so we can discuss the approach.
2. Fork the repo and create a branch: `git checkout -b feat/my-change`.
3. Write your change and add tests.
4. Run the full local check:
   ```bash
   cargo fmt --check && cargo clippy --features full,serde -- -Dclippy::all && cargo test --features full,serde
   ```
5. Open a PR against `main`. Fill in the PR template.
6. A maintainer will review within a few business days.

**PR requirements:**

- All CI checks must be green.
- New public API must have doc comments with at least one `# Example` block.
- New value objects must include unit tests covering: valid input, empty input, invalid format, and normalisation (if applicable).

---

## Coding conventions

- **No comments that explain *what* the code does** — the code does that. Comments explain *why* when the reason is non-obvious.
- Every new public type must implement `ValueObject` and derive `Debug`, `Clone`, `PartialEq`, `Eq` (where sensible).
- `serde` and `sql` support is added via `cfg_attr` — never a hard dependency.
- `#[non_exhaustive]` on enums and structs that may grow.
- Keep feature flags additive — enabling a feature must never break downstream code.

---

## Adding a new value object

1. Create `src/<module>/<type_name>.rs`.
2. Implement `ValueObject` for the type.
3. Gate the module behind a feature flag in `Cargo.toml` and `src/lib.rs`.
4. Export the type from `src/<module>/mod.rs` and `src/lib.rs` prelude.
5. Add the feature to the `full` meta-feature.
6. Write unit tests in the same file.
7. Update the feature table in `README.md`.

---

## Release process

Releases are made by maintainers only:

1. Bump `version` in `Cargo.toml`.
2. Commit: `git commit -m "chore: release vX.Y.Z"`.
3. Tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
4. The `release` CI workflow publishes to crates.io and creates a GitHub Release automatically.
