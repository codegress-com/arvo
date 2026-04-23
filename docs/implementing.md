# Implementing custom value objects

You can implement the `ValueObject` and `PrimitiveValue` traits for your own domain types. Use the existing types in `src/` as reference implementations.

## Simple value object

A simple VO wraps one raw primitive. Implement both `ValueObject` (construction + deconstruction) and `PrimitiveValue` (typed accessor).

```rust,ignore
use arvo::errors::ValidationError;
use arvo::traits::{PrimitiveValue, ValueObject};

pub type PercentageInput = f64;

pub struct Percentage(f64);

impl ValueObject for Percentage {
    type Input = f64;
    type Error = ValidationError;

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

    fn into_inner(self) -> f64 { self.0 }
}

impl PrimitiveValue for Percentage {
    type Primitive = f64;
    fn value(&self) -> &f64 { &self.0 }
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

A composite VO accepts multiple typed inputs. Implement only `ValueObject`. Expose data through dedicated accessor methods and `Display`. Provide `value()` as an inherent method returning the canonical string if useful.

```rust,ignore
use arvo::errors::ValidationError;
use arvo::traits::ValueObject;

pub struct CoordinateInput {
    pub latitude:  f64,
    pub longitude: f64,
}

pub struct Coordinate {
    input:     CoordinateInput,
    canonical: String,
}

impl ValueObject for Coordinate {
    type Input = CoordinateInput;
    type Error = ValidationError;

    fn new(value: CoordinateInput) -> Result<Self, ValidationError> {
        if !(-90.0..=90.0).contains(&value.latitude) {
            return Err(ValidationError::invalid("Coordinate.latitude", &value.latitude.to_string()));
        }
        if !(-180.0..=180.0).contains(&value.longitude) {
            return Err(ValidationError::invalid("Coordinate.longitude", &value.longitude.to_string()));
        }
        let canonical = format!("{}, {}", value.latitude, value.longitude);
        Ok(Self { input: value, canonical })
    }

    fn into_inner(self) -> CoordinateInput { self.input }
}

impl Coordinate {
    pub fn value(&self) -> &str      { &self.canonical }
    pub fn latitude(&self)  -> f64   { self.input.latitude }
    pub fn longitude(&self) -> f64   { self.input.longitude }
}

impl TryFrom<&str> for Coordinate {
    type Error = ValidationError;
    fn try_from(s: &str) -> Result<Self, Self::Error> { /* parse canonical */ todo!() }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}
```

## Checklist for every new type

- [ ] `type Input` type alias defined and exported
- [ ] `#[derive(Debug, Clone, PartialEq, Eq, Hash)]` on the struct
- [ ] Serde: `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`
      + `serde(try_from = "T", into = "T")` so deserialisation validates via `new()`
      + `impl TryFrom<T>` delegating to `new()` and `#[cfg(feature = "serde")] impl From<Type> for T`
- [ ] `impl ValueObject` with `new` and `into_inner`
- [ ] For simple types: `impl PrimitiveValue` with `type Primitive` and `value()`
- [ ] For composite types: inherent `pub fn value(&self) -> &str` (if canonical string exists)
- [ ] `impl TryFrom<&str>` (for string-input types and composite types with reversible canonical format)
- [ ] `impl Display`
- [ ] Extra accessors for composite types
- [ ] Unit tests: valid input, empty/invalid input, normalisation, `try_from`, `serde_roundtrip`, `serde_deserialize_validates`
- [ ] Doc comment with `# Example` block
- [ ] Registered in `mod.rs` and `prelude`
- [ ] Status updated in `ROADMAP.md`

## ORM / database integration

arvo does not bundle database integration — this keeps dependencies minimal and lets you use any framework. Integrate using the accessors arvo already provides:

**Raw sqlx:**
```rust,ignore
// Bind — extract the primitive
query.bind(email.value())
query.bind(addr.street())

// Read back — construct via new()
let s: String = row.get("email");
let email = EmailAddress::new(s)?;
```

**SeaORM / Diesel — composite types as multiple columns:**
```rust,ignore
// Define your own entity with individual columns
impl From<PostalAddress> for AddressModel {
    fn from(addr: PostalAddress) -> Self {
        let i = addr.into_inner();
        AddressModel { street: i.street, city: i.city, zip: i.zip,
                       country: i.country.into_inner() }
    }
}
```

See [CONTRIBUTING.md](../CONTRIBUTING.md) for the full development workflow.
