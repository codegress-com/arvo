# contact module

Feature flag: `contact`

```toml
[dependencies]
arvo = { version = "0.4", features = ["contact"] }
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

## Website

A validated website URL. Accepts `http` and `https` schemes only. Scheme and host are normalised to lowercase on construction.

**Normalisation:** scheme and host lowercased.  
**Validation:** must be a valid URL with `http` or `https` scheme and a host.

```rust,ignore
use arvo::contact::Website;
use arvo::traits::ValueObject;

let site = Website::new("https://EXAMPLE.COM/path".into())?;
assert_eq!(site.value(), "https://example.com/path");
assert!(site.is_https());
assert_eq!(site.host(), "example.com");

// try_into from &str
let site: Website = "https://example.com".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"https://example.com/"` |
| `is_https()` | `bool` | `true` |
| `host()` | `&str` | `"example.com"` |
| `into_inner()` | `String` | `"https://example.com/"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"not-a-url"` | `ValidationError::InvalidFormat` |
| `"ftp://example.com"` | `ValidationError::InvalidFormat` (scheme not allowed) |

---

## PostalAddress

A validated composite postal address. All text fields are trimmed; empty or whitespace-only values are rejected. The `country` field requires a valid [`CountryCode`].

**Normalisation:** `street`, `city`, and `zip` are trimmed.  
**Validation:** all fields must be non-empty after trimming.

```rust,ignore
use arvo::contact::{CountryCode, PostalAddress, PostalAddressInput};
use arvo::traits::ValueObject;

let addr = PostalAddress::new(PostalAddressInput {
    street:  "Václavské náměstí 1".into(),
    city:    "Prague".into(),
    zip:     "110 00".into(),
    country: CountryCode::new("CZ".into())?,
})?;

assert_eq!(addr.street(), "Václavské náměstí 1");
assert_eq!(addr.zip(), "110 00");
assert_eq!(addr.country().value(), "CZ");

// value() / Display — multi-line canonical form
assert_eq!(addr.value(), "Václavské náměstí 1\n110 00 Prague\nCZ");
```

### Input struct

```rust,ignore
pub struct PostalAddressInput {
    pub street:  String,
    pub city:    String,
    pub zip:     String,
    pub country: CountryCode,
}
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"Main St 1\n10115 Berlin\nDE"` |
| `street()` | `&str` | `"Main St 1"` |
| `city()` | `&str` | `"Berlin"` |
| `zip()` | `&str` | `"10115"` |
| `country()` | `&CountryCode` | `CountryCode("DE")` |
| `into_inner()` | `PostalAddressInput` | original input fields |

### Errors

| Field | Input | Error |
|---|---|---|
| `street` | `""` or whitespace | `ValidationError::Empty` |
| `city` | `""` or whitespace | `ValidationError::Empty` |
| `zip` | `""` or whitespace | `ValidationError::Empty` |
