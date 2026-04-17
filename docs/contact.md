# contact module

Feature flag: `contact`

```toml
[dependencies]
arvo = { version = "0.1", features = ["contact"] }
```

---

## EmailAddress

A validated, normalised email address.

**Normalisation:** trimmed, lowercased.  
**Validation:** must match `local@domain.tld` pattern (RFC 5322 lite — full compliance intentionally out of scope).

```rust,ignore
use arvo::contact::EmailAddress;
use arvo::traits::ValueObject;

let email = EmailAddress::new("User@Example.COM".into())?;
assert_eq!(email.value(), "user@example.com");
assert_eq!(email.local_part(), "user");
assert_eq!(email.domain(), "example.com");

// try_into from &str
let email: EmailAddress = "hello@example.com".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"user@example.com"` |
| `local_part()` | `&str` | `"user"` |
| `domain()` | `&str` | `"example.com"` |
| `into_inner()` | `String` | `"user@example.com"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"notanemail"` | `ValidationError::InvalidFormat` |

---

## CountryCode

A validated ISO 3166-1 alpha-2 country code.

**Normalisation:** trimmed, uppercased.  
**Validation:** exactly 2 ASCII letters.

```rust,ignore
use arvo::contact::CountryCode;
use arvo::traits::ValueObject;

let code = CountryCode::new("cz".into())?;
assert_eq!(code.value(), "CZ");

// try_into from &str
let code: CountryCode = "DE".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"CZ"` |
| `into_inner()` | `String` | `"CZ"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::InvalidFormat` |
| `"USA"` | `ValidationError::InvalidFormat` (3 letters) |
| `"C1"` | `ValidationError::InvalidFormat` (digit) |

---

## PhoneNumber

A validated phone number stored in canonical E.164 format.

**Normalisation:** non-digit characters stripped from the local number.  
**Validation:** local number must be 4–14 digits. Calling code is derived from `CountryCode`.

```rust,ignore
use arvo::contact::{CountryCode, PhoneNumber, PhoneNumberInput};
use arvo::traits::ValueObject;

let phone = PhoneNumber::new(PhoneNumberInput {
    country_code: CountryCode::new("CZ".into())?,
    number: "123 456 789".into(),   // spaces stripped automatically
})?;

assert_eq!(phone.value(), "+420123456789");
assert_eq!(phone.calling_code(), "+420");
assert_eq!(phone.number(), "123456789");
assert_eq!(phone.country_code().value(), "CZ");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"+420123456789"` |
| `calling_code()` | `&str` | `"+420"` |
| `number()` | `&str` | `"123456789"` |
| `country_code()` | `&CountryCode` | `CountryCode("CZ")` |
| `into_inner()` | `PhoneNumberInput` | `{ country_code: "CZ", number: "123456789" }` |

### Input struct

```rust,ignore
pub struct PhoneNumberInput {
    pub country_code: CountryCode,
    pub number: String,  // digits only; formatting characters are stripped
}
```

### Errors

| Input | Error |
|---|---|
| `number: ""` | `ValidationError::InvalidFormat` |
| `number: "123"` | `ValidationError::InvalidFormat` (too short, min 4 digits) |
| `number: "123456789012345"` | `ValidationError::InvalidFormat` (too long, max 14 digits) |

---

## Planned

| Type | Notes |
|---|---|
| `PostalAddress` | composite: street + city + zip + `CountryCode` |
| `Website` | valid URL, https preferred |

See [ROADMAP.md](../ROADMAP.md) for full details.
