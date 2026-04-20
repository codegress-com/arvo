# identifiers module

Feature flag: `identifiers`

```toml
[dependencies]
arvo = { version = "0.3", features = ["identifiers"] }
```

---

## Slug

A URL-safe slug: lowercase alphanumeric characters and hyphens only.

**Normalisation:** trimmed, lowercased.  
**Validation:** non-empty; only `[a-z0-9-]`; no leading, trailing, or consecutive hyphens.

```rust,ignore
use arvo::identifiers::Slug;
use arvo::traits::ValueObject;

let slug = Slug::new("Hello-World".into())?;
assert_eq!(slug.value(), "hello-world");

assert!(Slug::new("-bad".into()).is_err());
assert!(Slug::new("has--double".into()).is_err());

let s: Slug = "my-slug".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"hello-world"` |
| `into_inner()` | `String` | `"hello-world"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"-bad"` | `ValidationError::InvalidFormat` |
| `"has--double"` | `ValidationError::InvalidFormat` |
| `"has space"` | `ValidationError::InvalidFormat` |

---

## Ean13

A validated EAN-13 barcode number.

**Normalisation:** spaces and hyphens stripped; only digits retained.  
**Validation:** exactly 13 digits; check digit valid per GS1 algorithm (alternating weights from right, total mod 10 == 0).

```rust,ignore
use arvo::identifiers::Ean13;
use arvo::traits::ValueObject;

let ean = Ean13::new("4006381333931".into())?;
assert_eq!(ean.value(), "4006381333931");
assert_eq!(ean.check_digit(), 1);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"4006381333931"` |
| `check_digit()` | `u8` | `1` |
| `into_inner()` | `String` | `"4006381333931"` |

### Errors

| Input | Error |
|---|---|
| wrong digit count | `ValidationError::InvalidFormat` |
| invalid checksum | `ValidationError::InvalidFormat` |

---

## Ean8

A validated EAN-8 barcode number.

**Normalisation:** spaces and hyphens stripped; only digits retained.  
**Validation:** exactly 8 digits; check digit valid per GS1 algorithm.

```rust,ignore
use arvo::identifiers::Ean8;
use arvo::traits::ValueObject;

let ean = Ean8::new("73513537".into())?;
assert_eq!(ean.value(), "73513537");
assert_eq!(ean.check_digit(), 7);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"73513537"` |
| `check_digit()` | `u8` | `7` |
| `into_inner()` | `String` | `"73513537"` |

### Errors

| Input | Error |
|---|---|
| wrong digit count | `ValidationError::InvalidFormat` |
| invalid checksum | `ValidationError::InvalidFormat` |

---

## Isbn13

A validated ISBN-13 number.

**Normalisation:** hyphens and spaces stripped; output is 13 bare digits.  
**Validation:** exactly 13 digits; must start with `978` or `979`; check digit valid per EAN-13 algorithm.

```rust,ignore
use arvo::identifiers::Isbn13;
use arvo::traits::ValueObject;

let isbn = Isbn13::new("978-0-306-40615-7".into())?;
assert_eq!(isbn.value(), "9780306406157");
assert_eq!(isbn.prefix(), "978");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"9780306406157"` |
| `prefix()` | `&str` | `"978"` or `"979"` |
| `into_inner()` | `String` | `"9780306406157"` |

### Errors

| Input | Error |
|---|---|
| wrong digit count | `ValidationError::InvalidFormat` |
| wrong prefix | `ValidationError::InvalidFormat` |
| invalid checksum | `ValidationError::InvalidFormat` |

---

## Isbn10

A validated ISBN-10 number.

**Normalisation:** hyphens and spaces stripped; trailing `x` uppercased to `X`; output is 10 bare characters.  
**Validation:** exactly 10 characters (9 digits + check `0–9` or `X`); validated using ISBN-10 weighted sum (weights 10 down to 2, total mod 11 == 0; `X` = 10).

```rust,ignore
use arvo::identifiers::Isbn10;
use arvo::traits::ValueObject;

let isbn = Isbn10::new("0-306-40615-2".into())?;
assert_eq!(isbn.value(), "0306406152");

let isbn_x = Isbn10::new("047191536x".into())?;
assert_eq!(isbn_x.value(), "047191536X");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"0306406152"` |
| `into_inner()` | `String` | `"0306406152"` |

### Errors

| Input | Error |
|---|---|
| wrong length | `ValidationError::InvalidFormat` |
| invalid checksum | `ValidationError::InvalidFormat` |

---

## Issn

A validated ISSN (International Standard Serial Number).

**Normalisation:** spaces and hyphens stripped; trailing `x` uppercased; output formatted as `XXXX-XXXX`.  
**Validation:** exactly 8 characters (7 digits + check `0–9` or `X`); validated using ISSN weighted sum (weights 8 down to 2, total mod 11 == 0; `X` = 10).

```rust,ignore
use arvo::identifiers::Issn;
use arvo::traits::ValueObject;

let issn = Issn::new("0317-8471".into())?;
assert_eq!(issn.value(), "0317-8471");

let issn_x = Issn::new("0000006x".into())?;
assert_eq!(issn_x.value(), "0000-006X");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"0317-8471"` |
| `into_inner()` | `String` | `"0317-8471"` |

### Errors

| Input | Error |
|---|---|
| wrong length | `ValidationError::InvalidFormat` |
| invalid checksum | `ValidationError::InvalidFormat` |

---

## Vin

A validated Vehicle Identification Number (VIN) per ISO 3779.

**Normalisation:** trimmed, uppercased.  
**Validation:** exactly 17 characters from the VIN alphabet (letters and digits; `I`, `O`, `Q` forbidden); check digit at position 9 (1-indexed) validated via the standard transliteration table and positional weights (mod 11; `X` = 10).

```rust,ignore
use arvo::identifiers::Vin;
use arvo::traits::ValueObject;

let vin = Vin::new("1HGBH41JXMN109186".into())?;
assert_eq!(vin.wmi(), "1HG");
assert_eq!(vin.vds(), "BH41JX");
assert_eq!(vin.vis(), "MN109186");
assert_eq!(vin.model_year(), 'M');
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"1HGBH41JXMN109186"` |
| `wmi()` | `&str` | `"1HG"` (World Manufacturer Identifier) |
| `vds()` | `&str` | `"BH41JX"` (Vehicle Descriptor Section) |
| `vis()` | `&str` | `"MN109186"` (Vehicle Identifier Section) |
| `model_year()` | `char` | `'M'` |
| `into_inner()` | `String` | `"1HGBH41JXMN109186"` |

### Errors

| Input | Error |
|---|---|
| wrong length | `ValidationError::InvalidFormat` |
| contains `I`, `O`, or `Q` | `ValidationError::InvalidFormat` |
| invalid check digit | `ValidationError::InvalidFormat` |
