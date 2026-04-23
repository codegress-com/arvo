# finance module

Feature flag: `finance`

```toml
[dependencies]
arvo = { version = "1.0", features = ["finance"] }
```

---

## CurrencyCode

A validated ISO 4217 alphabetic currency code.

**Normalisation:** trimmed, uppercased.  
**Validation:** exactly 3 ASCII letters; must be a known active ISO 4217 code.

```rust,ignore
use arvo::finance::CurrencyCode;
use arvo::traits::{PrimitiveValue, ValueObject};

let code = CurrencyCode::new("eur".into())?;
assert_eq!(code.value(), "EUR");

assert!(CurrencyCode::new("XYZ".into()).is_err());

let c: CurrencyCode = "CZK".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"EUR"` |
| `into_inner()` | `String` | `"EUR"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"US"` | `ValidationError::InvalidFormat` |
| `"XYZ"` | `ValidationError::InvalidFormat` |

---

## Money

A validated monetary amount with an associated currency.

**Normalisation:** none (amount stored as-is; output formatted with 2 decimal places).  
**Validation:** `amount` may be any finite `Decimal`; `currency` must be a valid `CurrencyCode`.

```rust,ignore
use arvo::finance::{CurrencyCode, Money, MoneyInput};
use arvo::traits::{PrimitiveValue, ValueObject};

let money = Money::new(MoneyInput {
    amount: "10.50".parse()?,
    currency: CurrencyCode::new("EUR".into())?,
})?;
assert_eq!(money.value(), "10.50 EUR");
assert_eq!(money.currency().value(), "EUR");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"10.50 EUR"` |
| `amount()` | `&Decimal` | `10.50` |
| `currency()` | `&CurrencyCode` | `CurrencyCode("EUR")` |
| `into_inner()` | `MoneyInput` | — |

### Arithmetic helpers

All operations are immutable — they return a new `Money` or `Result<Money>`.

| Method | Returns | Notes |
|---|---|---|
| `add(&Money)` | `Result<Money>` | Fails if currencies differ |
| `sub(&Money)` | `Result<Money>` | Fails if currencies differ; result may be negative |
| `neg()` | `Money` | Negates the amount; always succeeds |

---

## Iban

A validated IBAN (International Bank Account Number) using the mod-97 algorithm.

**Normalisation:** spaces stripped, uppercased.  
**Validation:** 15–34 characters; starts with 2-letter country code and 2 check digits; all remaining characters alphanumeric; mod-97 checksum equals 1.

```rust,ignore
use arvo::finance::Iban;
use arvo::traits::{PrimitiveValue, ValueObject};

let iban = Iban::new("GB82 WEST 1234 5698 7654 32".into())?;
assert_eq!(iban.value(), "GB82WEST12345698765432");
assert_eq!(iban.country_code(), "GB");
assert_eq!(iban.bban(), "WEST12345698765432");

let i: Iban = "DE89370400440532013000".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"GB82WEST12345698765432"` |
| `country_code()` | `&str` | `"GB"` |
| `check_digits()` | `&str` | `"82"` |
| `bban()` | `&str` | `"WEST12345698765432"` |
| `into_inner()` | `String` | — |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| too short / too long | `ValidationError::InvalidFormat` |
| wrong checksum | `ValidationError::InvalidFormat` |

---

## Bic

A validated BIC (Bank Identifier Code / SWIFT code).

**Normalisation:** trimmed, uppercased.  
**Validation:** 8 or 11 alphanumeric characters; positions 1–4 are letters (bank code); positions 5–6 are letters (country code); positions 7–8 are alphanumeric (location code); optional positions 9–11 are alphanumeric (branch code).

```rust,ignore
use arvo::finance::Bic;
use arvo::traits::{PrimitiveValue, ValueObject};

let bic = Bic::new("DEUTDEDB".into())?;
assert_eq!(bic.bank_code(), "DEUT");
assert_eq!(bic.country_code(), "DE");
assert_eq!(bic.branch_code(), None);

let bic11 = Bic::new("DEUTDEDBBER".into())?;
assert_eq!(bic11.branch_code(), Some("BER"));
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"DEUTDEDB"` |
| `bank_code()` | `&str` | `"DEUT"` |
| `country_code()` | `&str` | `"DE"` |
| `location_code()` | `&str` | `"DB"` |
| `branch_code()` | `Option<&str>` | `None` / `Some("BER")` |
| `into_inner()` | `String` | — |

---

## VatNumber

A validated EU VAT number.

**Normalisation:** trimmed, uppercased, internal spaces stripped.  
**Validation:** starts with a known EU country prefix (AT, BE, BG, CY, CZ, DE, DK, EE, EL, ES, FI, FR, HR, HU, IE, IT, LT, LU, LV, MT, NL, PL, PT, RO, SE, SI, SK, XI); followed by 2–13 alphanumeric characters.

```rust,ignore
use arvo::finance::VatNumber;
use arvo::traits::{PrimitiveValue, ValueObject};

let vat = VatNumber::new("CZ 1234 5678".into())?;
assert_eq!(vat.value(), "CZ12345678");
assert_eq!(vat.country_prefix(), "CZ");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"CZ12345678"` |
| `country_prefix()` | `&str` | `"CZ"` |
| `into_inner()` | `String` | — |

---

## Percentage

A validated percentage in the range `0.0..=100.0`.

**Normalisation:** none.  
**Validation:** finite (not NaN, not infinite); in range `0.0..=100.0` inclusive.

```rust,ignore
use arvo::finance::Percentage;
use arvo::traits::{PrimitiveValue, ValueObject};

let p = Percentage::new(42.5)?;
assert_eq!(*p.value(), 42.5);

assert!(Percentage::new(-1.0).is_err());
assert!(Percentage::new(f64::NAN).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&f64` | `42.5` |
| `as_fraction()` | `f64` | `0.425` (divides by 100) |
| `into_inner()` | `f64` | `42.5` |

---

## ExchangeRate

A validated currency exchange rate between two different currencies.

**Normalisation:** none.  
**Validation:** `rate` must be strictly positive (> 0); `from` and `to` must be different currencies.

```rust,ignore
use arvo::finance::{CurrencyCode, ExchangeRate, ExchangeRateInput};
use arvo::traits::{PrimitiveValue, ValueObject};

let rate = ExchangeRate::new(ExchangeRateInput {
    from: CurrencyCode::new("EUR".into())?,
    to:   CurrencyCode::new("USD".into())?,
    rate: "1.0850".parse()?,
})?;
assert_eq!(rate.value(), "EUR/USD 1.0850");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"EUR/USD 1.0850"` |
| `from()` | `&CurrencyCode` | `CurrencyCode("EUR")` |
| `to()` | `&CurrencyCode` | `CurrencyCode("USD")` |
| `rate()` | `&Decimal` | `1.0850` |
| `into_inner()` | `ExchangeRateInput` | — |

### Errors

| Condition | Error |
|---|---|
| `rate <= 0` | `ValidationError::InvalidFormat` |
| `from == to` | `ValidationError::InvalidFormat` |

---

## CreditCardNumber

A validated credit card number using the Luhn algorithm.

**Normalisation:** spaces and hyphens stripped; only digits kept.  
**Validation:** 13–19 digits after stripping; must pass the Luhn checksum.  
**Display:** masked — only last 4 digits visible, e.g. `"**** **** **** 0366"`.

```rust,ignore
use arvo::finance::CreditCardNumber;
use arvo::traits::{PrimitiveValue, ValueObject};

let card = CreditCardNumber::new("4532 0151 1283 0366".into())?;
assert_eq!(card.last_four(), "0366");
assert_eq!(card.masked(), "**** **** **** 0366");
// value() returns the full digit string — treat as sensitive
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"4532015112830366"` (sensitive) |
| `last_four()` | `&str` | `"0366"` |
| `masked()` | `String` | `"**** **** **** 0366"` |
| `into_inner()` | `String` | — |

---

## CardExpiryDate

A validated credit/debit card expiry date that is not in the past.

**Normalisation:** parsed and stored as `"MM/YY"`.  
**Input:** accepts `"MM/YY"` or `"MM/YYYY"`.  
**Validation:** month 01–12; expiry month/year must be ≥ current month/year (card valid through entire expiry month).

```rust,ignore
use arvo::finance::CardExpiryDate;
use arvo::traits::{PrimitiveValue, ValueObject};

let exp = CardExpiryDate::new("12/28".into())?;
assert_eq!(exp.value(), "12/28");
assert_eq!(exp.month(), 12);
assert_eq!(exp.year(), 2028);

assert!(CardExpiryDate::new("01/20".into()).is_err()); // past
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"12/28"` |
| `month()` | `u8` | `12` |
| `year()` | `u16` | `2028` |
| `months_until()` | `u32` | full months remaining until expiry |
| `into_inner()` | `String` | — |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| invalid month | `ValidationError::InvalidFormat` |
| expired | `ValidationError::InvalidFormat` |
