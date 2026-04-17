## Summary

<!-- What does this PR do and why? Link the related issue: "Closes #123" -->

## Type of change

- [ ] Bug fix
- [ ] New value object / feature
- [ ] Documentation
- [ ] Refactor / internal improvement
- [ ] CI / tooling

## Checklist

- [ ] `cargo fmt` — code is formatted
- [ ] `cargo clippy --features full,serde -- -Dclippy::all` — no warnings
- [ ] `cargo test --features full,serde` — all tests pass
- [ ] New public API has doc comments with an `# Example` block
- [ ] New value objects have tests for: valid input, empty input, invalid format, normalisation
- [ ] README feature table updated (if a new feature was added)
