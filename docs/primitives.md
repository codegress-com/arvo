# primitives module

Feature flag: `primitives`

```toml
[dependencies]
arvo = { version = "0.4", features = ["primitives"] }
```

---

## NonEmptyString

A non-empty, trimmed string.

**Normalisation:** surrounding whitespace trimmed.  
**Validation:** must not be empty after trimming.

```rust,ignore
use arvo::primitives::NonEmptyString;
use arvo::traits::ValueObject;

let s = NonEmptyString::new("  hello  ".into())?;
assert_eq!(s.value(), "hello");

let s: NonEmptyString = "world".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"hello"` |
| `into_inner()` | `String` | `"hello"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"   "` | `ValidationError::Empty` |

---

## BoundedString

A string whose length (in Unicode characters) is constrained at the type level.

**Normalisation:** surrounding whitespace trimmed.  
**Validation:** character count (not byte count) must be `>= MIN` and `<= MAX`.

```rust,ignore
use arvo::primitives::BoundedString;
use arvo::traits::ValueObject;

type Username = BoundedString<3, 32>;

let name = Username::new("Alice".into())?;
assert_eq!(name.value(), "Alice");

assert!(Username::new("Al".into()).is_err()); // too short
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"Alice"` |
| `into_inner()` | `String` | `"Alice"` |

### Errors

| Input | Error |
|---|---|
| value shorter than `MIN` | `ValidationError::OutOfRange` |
| value longer than `MAX` | `ValidationError::OutOfRange` |

---

## PositiveInt

A strictly positive integer (`i64 > 0`).

**Normalisation:** none.  
**Validation:** value must be `> 0`. Zero and negative values are rejected.

```rust,ignore
use arvo::primitives::PositiveInt;
use arvo::traits::ValueObject;

let n = PositiveInt::new(42)?;
assert_eq!(*n.value(), 42);

assert!(PositiveInt::new(0).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&i64` | `42` |
| `into_inner()` | `i64` | `42` |

### Errors

| Input | Error |
|---|---|
| `0` | `ValidationError::OutOfRange` |
| `-1` | `ValidationError::OutOfRange` |

---

## NonNegativeInt

A non-negative integer (`i64 >= 0`).

**Normalisation:** none.  
**Validation:** value must be `>= 0`. Negative values are rejected.

```rust,ignore
use arvo::primitives::NonNegativeInt;
use arvo::traits::ValueObject;

let n = NonNegativeInt::new(0)?;
assert_eq!(*n.value(), 0);

assert!(NonNegativeInt::new(-1).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&i64` | `0` |
| `into_inner()` | `i64` | `0` |

### Errors

| Input | Error |
|---|---|
| `-1` | `ValidationError::OutOfRange` |

---

## PositiveDecimal

A strictly positive decimal (`rust_decimal::Decimal > 0`).

**Normalisation:** none.  
**Validation:** value must be `> Decimal::ZERO`.

```rust,ignore
use arvo::primitives::PositiveDecimal;
use arvo::traits::ValueObject;
use rust_decimal::Decimal;
use std::str::FromStr;

let price = PositiveDecimal::new(Decimal::from_str("9.99").unwrap())?;
assert_eq!(price.value(), &Decimal::from_str("9.99").unwrap());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&Decimal` | `9.99` |
| `into_inner()` | `Decimal` | `9.99` |

### Errors

| Input | Error |
|---|---|
| `Decimal::ZERO` | `ValidationError::OutOfRange` |
| negative value | `ValidationError::OutOfRange` |

---

## NonNegativeDecimal

A non-negative decimal (`rust_decimal::Decimal >= 0`).

**Normalisation:** none.  
**Validation:** value must be `>= Decimal::ZERO`.

```rust,ignore
use arvo::primitives::NonNegativeDecimal;
use arvo::traits::ValueObject;
use rust_decimal::Decimal;

let amount = NonNegativeDecimal::new(Decimal::ZERO)?;
assert_eq!(amount.value(), &Decimal::ZERO);
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&Decimal` | `0` |
| `into_inner()` | `Decimal` | `0` |

### Errors

| Input | Error |
|---|---|
| negative value | `ValidationError::OutOfRange` |

---

## Probability

A probability value in the closed interval `[0.0, 1.0]`.

**Normalisation:** none.  
**Validation:** must be finite and in `0.0..=1.0`. NaN and infinity are rejected.

```rust,ignore
use arvo::primitives::Probability;
use arvo::traits::ValueObject;

let p = Probability::new(0.75)?;
assert_eq!(*p.value(), 0.75);

assert!(Probability::new(1.5).is_err());
assert!(Probability::new(f64::NAN).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&f64` | `0.75` |
| `into_inner()` | `f64` | `0.75` |

### Errors

| Input | Error |
|---|---|
| `1.5` | `ValidationError::OutOfRange` |
| `-0.1` | `ValidationError::OutOfRange` |
| `NaN` | `ValidationError::OutOfRange` |
| `∞` | `ValidationError::OutOfRange` |

---

## HexColor

A CSS hex color in canonical `#RRGGBB` form.

**Normalisation:** trimmed; uppercased; 3-digit shorthand expanded to 6-digit form (`#F0A` → `#FF00AA`).  
**Validation:** must start with `#`; remaining chars must be exactly 3 or 6 hexadecimal digits.

```rust,ignore
use arvo::primitives::HexColor;
use arvo::traits::ValueObject;

let red = HexColor::new("#f00".into())?;
assert_eq!(red.value(), "#FF0000");
assert_eq!(red.r(), 255);
assert_eq!(red.g(), 0);
assert_eq!(red.b(), 0);

let c: HexColor = "#1A2B3C".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"#FF0000"` |
| `r()` | `u8` | `255` |
| `g()` | `u8` | `0` |
| `b()` | `u8` | `0` |
| `into_inner()` | `String` | `"#FF0000"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"FF0000"` | `ValidationError::InvalidFormat` (missing `#`) |
| `"#GGGGGG"` | `ValidationError::InvalidFormat` |
| `"#FFFF"` | `ValidationError::InvalidFormat` (wrong length) |

---

## Locale

A BCP 47 language tag (e.g. `"en-US"`, `"cs-CZ"`, `"fr"`).

**Normalisation:** trimmed; `_` separator normalised to `-`; language subtag lowercased; region subtag uppercased.  
**Validation (MVP):** language subtag must be 2–3 ASCII letters; optional region subtag must be 2 ASCII letters or 3 digits.

```rust,ignore
use arvo::primitives::Locale;
use arvo::traits::ValueObject;

let locale = Locale::new("en_us".into())?;
assert_eq!(locale.value(), "en-US");

let fr: Locale = "fr".try_into()?;
assert_eq!(fr.value(), "fr");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"en-US"` |
| `into_inner()` | `String` | `"en-US"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"e"` | `ValidationError::InvalidFormat` (language too short) |
| `"engl"` | `ValidationError::InvalidFormat` (language too long) |
| `"en-X1"` | `ValidationError::InvalidFormat` (invalid region) |

---

## Base64String

A validated standard Base64-encoded string.

**Normalisation:** surrounding whitespace trimmed.  
**Validation:** must decode successfully using the standard Base64 alphabet (`A–Z`, `a–z`, `0–9`, `+`, `/`) with correct `=` padding.

```rust,ignore
use arvo::primitives::Base64String;
use arvo::traits::ValueObject;

let b = Base64String::new("aGVsbG8=".into())?;
assert_eq!(b.decode(), b"hello");

let b: Base64String = "aGVsbG8=".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"aGVsbG8="` |
| `decode()` | `Vec<u8>` | `[104, 101, 108, 108, 111]` |
| `into_inner()` | `String` | `"aGVsbG8="` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"not!!valid"` | `ValidationError::InvalidFormat` |
| `"aGVsbG8"` | `ValidationError::InvalidFormat` (invalid padding) |
