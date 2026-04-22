# Value objects

## What is a value object?

A **value object** is a small, immutable wrapper around a raw primitive that carries a domain guarantee: *if it exists, it is valid.*

Instead of passing a raw `String` and hoping it contains a valid email address, you pass an `EmailAddress` — and you know it was validated at the moment it was created. There is no way to construct an invalid instance.

```rust,ignore
// Without value objects — validation is the caller's responsibility
fn send_email(address: String) { /* is this actually valid? */ }

// With value objects — validity is guaranteed by the type
fn send_email(address: EmailAddress) { /* always valid */ }
```

## The two kinds of value objects in arvo

### Simple value objects

A simple VO wraps a single raw primitive. Implements both `ValueObject` and `PrimitiveValue`.

```
"User@Example.COM"  →  EmailAddress("user@example.com")
       ↑                          ↑
     Input                  PrimitiveValue::value() → &String
```

Examples: `EmailAddress`, `CountryCode`, `Latitude`, `Port`.

### Composite value objects

A composite VO is constructed from multiple typed inputs and exposes data through dedicated accessor methods. Implements only `ValueObject`.

```
PhoneNumberInput { country_code: "CZ", number: "123456789" }
       ↓
PhoneNumber { e164: "+420123456789" }
       ↓
value()          → &str → "+420123456789"  (inherent method)
calling_code()   → &str → "+420"
number()         → &str → "123456789"
country_code()   → &CountryCode
```

Examples: `PhoneNumber`, `Money`, `PostalAddress`.

## The trait hierarchy

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

Use `PrimitiveValue` as a bound when you need generic access to the inner value:

```rust,ignore
fn print_value<T: PrimitiveValue<Primitive = str>>(v: &T) {
    println!("{}", v.value());
}
```

For composite types, use the concrete type and its specific accessors.

## Why immutability matters

Once constructed, a value object never changes. There are no setters. If you need a different value, you construct a new instance. This means:

- The type system enforces validity — not documentation, not tests, not convention
- Value objects are safe to share across threads (`Clone` + no mutation)
- Equality is structural — two `EmailAddress("user@example.com")` instances are always equal

## Normalisation

Many value objects normalise their input on construction:

| Type | Normalisation |
|---|---|
| `EmailAddress` | trimmed, lowercased |
| `CountryCode` | trimmed, uppercased |
| `PhoneNumber` | non-digit characters stripped, stored as E.164 |

This means `EmailAddress::new("User@Example.COM")` and `EmailAddress::new("user@example.com")` produce equal instances.

## Errors

All validation errors are variants of `ValidationError`. See [error handling](../README.md#error-handling) in the README.
