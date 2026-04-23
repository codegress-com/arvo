# geo module

Feature flag: `geo`

```toml
[dependencies]
arvo = { version = "1.0", features = ["geo"] }
```

---

## Latitude

A validated geographic latitude in decimal degrees.

**Validation:** must be finite, in the inclusive range `−90.0..=90.0`.

```rust,ignore
use arvo::geo::Latitude;
use arvo::traits::{PrimitiveValue, ValueObject};

let lat = Latitude::new(48.8588)?;
assert_eq!(*lat.value(), 48.8588);

assert!(Latitude::new(91.0).is_err());
assert!(Latitude::new(f64::NAN).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&f64` | `48.858844` |
| `into_inner()` | `f64` | `48.858844` |

### Errors

| Input | Error |
|---|---|
| `> 90.0` or `< -90.0` | `ValidationError::InvalidFormat` |
| `NaN` / `Infinity` | `ValidationError::InvalidFormat` |

---

## Longitude

A validated geographic longitude in decimal degrees.

**Validation:** must be finite, in the inclusive range `−180.0..=180.0`.

```rust,ignore
use arvo::geo::Longitude;
use arvo::traits::{PrimitiveValue, ValueObject};

let lng = Longitude::new(14.4208)?;
assert_eq!(*lng.value(), 14.4208);

assert!(Longitude::new(181.0).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&f64` | `14.420800` |
| `into_inner()` | `f64` | `14.420800` |

### Errors

| Input | Error |
|---|---|
| `> 180.0` or `< -180.0` | `ValidationError::InvalidFormat` |
| `NaN` / `Infinity` | `ValidationError::InvalidFormat` |

---

## Coordinate

A geographic coordinate (latitude + longitude pair).

**Normalisation:** canonical string `"lat, lng"` with six decimal places.

```rust,ignore
use arvo::geo::{Coordinate, CoordinateInput, Latitude, Longitude};
use arvo::traits::{PrimitiveValue, ValueObject};

let coord = Coordinate::new(CoordinateInput {
    lat: Latitude::new(48.858844)?,
    lng: Longitude::new(2.294351)?,
})?;

assert_eq!(coord.value(), "48.858844, 2.294351");
assert_eq!(*coord.lat().value(), 48.858844);
assert_eq!(*coord.lng().value(), 2.294351);
```

### Input struct

```rust,ignore
pub struct CoordinateInput {
    pub lat: Latitude,
    pub lng: Longitude,
}
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&str` | `"48.858844, 2.294351"` |
| `lat()` | `&Latitude` | `Latitude(48.858844)` |
| `lng()` | `&Longitude` | `Longitude(2.294351)` |
| `into_inner()` | `CoordinateInput` | original input |

---

## BoundingBox

A geographic bounding box defined by a south-west and a north-east [`Coordinate`].

**Validation:** `sw.lat ≤ ne.lat` and `sw.lng ≤ ne.lng`.

```rust,ignore
use arvo::geo::{BoundingBox, BoundingBoxInput, Coordinate, CoordinateInput, Latitude, Longitude};
use arvo::traits::{PrimitiveValue, ValueObject};

let sw = Coordinate::new(CoordinateInput {
    lat: Latitude::new(48.0)?,
    lng: Longitude::new(14.0)?,
})?;
let ne = Coordinate::new(CoordinateInput {
    lat: Latitude::new(51.0)?,
    lng: Longitude::new(18.0)?,
})?;

let bbox = BoundingBox::new(BoundingBoxInput { sw, ne })?;
assert_eq!(bbox.value(), "SW: 48.000000, 14.000000 / NE: 51.000000, 18.000000");
assert_eq!(*bbox.sw().lat().value(), 48.0);
```

### Input struct

```rust,ignore
pub struct BoundingBoxInput {
    pub sw: Coordinate,
    pub ne: Coordinate,
}
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&str` | `"SW: 48.0, 14.0 / NE: 51.0, 18.0"` |
| `sw()` | `&Coordinate` | south-west corner |
| `ne()` | `&Coordinate` | north-east corner |
| `contains(&Coordinate)` | `bool` | inclusive on all four edges |
| `into_inner()` | `BoundingBoxInput` | original input |

### Errors

| Condition | Error |
|---|---|
| `sw.lat > ne.lat` or `sw.lng > ne.lng` | `ValidationError::InvalidFormat` |

---

## TimeZone

A validated IANA timezone name.

**Validation:** must be present in the built-in list of canonical IANA timezone names. The name is trimmed but **case-sensitive** — IANA names are case-sensitive by specification.

```rust,ignore
use arvo::geo::TimeZone;
use arvo::traits::{PrimitiveValue, ValueObject};

let tz = TimeZone::new("Europe/Prague".into())?;
assert_eq!(tz.value(), "Europe/Prague");

let tz: TimeZone = "UTC".try_into()?;

assert!(TimeZone::new("europe/prague".into()).is_err());  // wrong case
assert!(TimeZone::new("Fake/Zone".into()).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"Europe/Prague"` |
| `into_inner()` | `String` | `"Europe/Prague"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| unknown or wrong-case name | `ValidationError::InvalidFormat` |

---

## CountryRegion

A validated ISO 3166-2 subdivision code.

**Format:** two uppercase ASCII letters (country code), hyphen, one to eight uppercase alphanumeric characters (subdivision code). Examples: `"CZ-PR"`, `"US-CA"`, `"GB-ENG"`.

**Normalisation:** trimmed and uppercased.

```rust,ignore
use arvo::geo::CountryRegion;
use arvo::traits::{PrimitiveValue, ValueObject};

let region = CountryRegion::new("cz-pr".into())?;
assert_eq!(region.value(), "CZ-PR");
assert_eq!(region.country_code(), "CZ");
assert_eq!(region.subdivision_code(), "PR");

let region: CountryRegion = "US-CA".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"CZ-PR"` |
| `country_code()` | `&str` | `"CZ"` |
| `subdivision_code()` | `&str` | `"PR"` |
| `into_inner()` | `String` | `"CZ-PR"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| missing `-` | `ValidationError::InvalidFormat` |
| country code ≠ 2 letters | `ValidationError::InvalidFormat` |
| subdivision empty or > 8 chars | `ValidationError::InvalidFormat` |
