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

A simple VO wraps a single raw primitive. `Input` and `Output` are the same type.

```
"User@Example.COM"  →  EmailAddress("user@example.com")
       ↑                          ↑
     Input                      Output (&String)
```

Examples: `EmailAddress`, `CountryCode`.

### Composite value objects

A composite VO is constructed from multiple typed inputs and returns a canonical representation.

```
PhoneNumberInput { country_code: "CZ", number: "123456789" }
       ↓
PhoneNumber { e164: "+420123456789" }
       ↓
value() → &String → "+420123456789"
```

Examples: `PhoneNumber`.

## The `ValueObject` trait

All types implement the same interface:

```rust,ignore
pub trait ValueObject: Sized + Clone + PartialEq {
    type Input;           // what new() accepts
    type Output: ?Sized;  // what value() returns
    type Error: std::error::Error;

    fn new(value: Self::Input) -> Result<Self, Self::Error>;
    fn value(&self) -> &Self::Output;
    fn into_inner(self) -> Self::Input;
}
```

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
