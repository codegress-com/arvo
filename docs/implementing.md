# Implementing custom value objects

You can implement the `ValueObject` trait for your own domain types. Use the existing types in `src/contact/` as reference implementations.

## Simple value object

A simple VO wraps one raw primitive. `Input` and `Output` are the same type.

```rust,ignore
use arvo::errors::ValidationError;
use arvo::traits::ValueObject;

pub type PercentageInput  = f64;
pub type PercentageOutput = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct Percentage(f64);

impl ValueObject for Percentage {
    type Input  = PercentageInput;
    type Output = PercentageOutput;
    type Error  = ValidationError;

    fn new(value: f64) -> Result<Self, ValidationError> {
        if !(0.0..=100.0).contains(&value) {
            return Err(ValidationError::OutOfRange {
                type_name: "Percentage",
                min:    "0".into(),
                max:    "100".into(),
                actual: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    fn value(&self) -> &f64    { &self.0 }
    fn into_inner(self) -> f64 { self.0 }
}

impl TryFrom<f64> for Percentage {
    type Error = ValidationError;
    fn try_from(v: f64) -> Result<Self, Self::Error> { Self::new(v) }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}
```

## Composite value object

A composite VO accepts multiple typed inputs and returns a canonical representation. `Input` is a dedicated struct; `Output` is typically `String`.

```rust,ignore
use arvo::errors::ValidationError;
use arvo::traits::ValueObject;

// Dedicated input struct — one field per component
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoordinateInput {
    pub latitude:  f64,
    pub longitude: f64,
}

pub type CoordinateOutput = String; // canonical: "48.8566,2.3522"

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    input:      CoordinateInput,
    canonical:  String,
}

impl ValueObject for Coordinate {
    type Input  = CoordinateInput;
    type Output = CoordinateOutput;
    type Error  = ValidationError;

    fn new(value: CoordinateInput) -> Result<Self, ValidationError> {
        if !(-90.0..=90.0).contains(&value.latitude) {
            return Err(ValidationError::invalid("Coordinate.latitude", &value.latitude.to_string()));
        }
        if !(-180.0..=180.0).contains(&value.longitude) {
            return Err(ValidationError::invalid("Coordinate.longitude", &value.longitude.to_string()));
        }
        let canonical = format!("{},{}", value.latitude, value.longitude);
        Ok(Self { input: value, canonical })
    }

    fn value(&self) -> &String          { &self.canonical }
    fn into_inner(self) -> CoordinateInput { self.input }
}

// Extra accessors beyond the trait
impl Coordinate {
    pub fn latitude(&self)  -> f64 { self.input.latitude }
    pub fn longitude(&self) -> f64 { self.input.longitude }
}
```

## Checklist for every new type

- [ ] `type Input` and `type Output` type aliases defined and exported
- [ ] `#[derive(Debug, Clone, PartialEq, Eq, Hash)]` on the struct
- [ ] Serde: `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
      + `serde(try_from = "T", into = "T")` so deserialisation validates via `new()`
      + `impl TryFrom<T>` delegating to `new()` and `#[cfg(feature = "serde")] impl From<Type> for T`
- [ ] SQLx: `#[cfg_attr(feature = "sql", derive(sqlx::Type))] #[cfg_attr(feature = "sql", sqlx(transparent))]`
      for simple newtypes; manual `Type + Encode + Decode` for composites (store as TEXT via `TryFrom<&str>`)
- [ ] `impl ValueObject` with `new`, `value`, `into_inner`
- [ ] `impl TryFrom<&str>` (for string-input types and all composite types)
- [ ] `impl Display`
- [ ] Extra accessors for composite types
- [ ] Unit tests: valid input, empty/invalid input, normalisation, `try_from`, `serde_roundtrip`, `serde_deserialize_validates`
- [ ] Doc comment with `# Example` block
- [ ] Registered in `mod.rs` and `prelude`
- [ ] Status updated in `ROADMAP.md`

See [CONTRIBUTING.md](../CONTRIBUTING.md) for the full development workflow.
