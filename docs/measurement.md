# measurement module

Feature flag: `measurement`

```toml
[dependencies]
arvo = { version = "1.0", features = ["measurement"] }
```

All measurement types share the same pattern: `XxxInput { value: f64, unit: XxxUnit }`.
`value()` returns a canonical string like `"75 kg"`. No unit conversion is provided — the unit is metadata.

---

## Length

**Validation:** finite, non-negative. **Units:** `Mm`, `Cm`, `M`, `Km`, `In`, `Ft`.

```rust,ignore
use arvo::measurement::{Length, LengthInput, LengthUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let len = Length::new(LengthInput { value: 1.80, unit: LengthUnit::M })?;
assert_eq!(len.value(), "1.8 m");
assert_eq!(len.amount(), 1.80);
```

| Method | Returns |
|---|---|
| `value()` | `&str` — e.g. `"1.8 m"` |
| `amount()` | `f64` |
| `unit()` | `&LengthUnit` |

---

## Weight

**Validation:** finite, non-negative. **Units:** `Mg`, `G`, `Kg`, `T`, `Oz`, `Lb`.

```rust,ignore
use arvo::measurement::{Weight, WeightInput, WeightUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let w = Weight::new(WeightInput { value: 75.0, unit: WeightUnit::Kg })?;
assert_eq!(w.value(), "75 kg");
```

---

## Temperature

**Validation:** finite; minimum depends on unit — Kelvin ≥ 0, Celsius ≥ −273.15, Fahrenheit ≥ −459.67.
**Units:** `Celsius`, `Fahrenheit`, `Kelvin`.

```rust,ignore
use arvo::measurement::{Temperature, TemperatureInput, TemperatureUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let t = Temperature::new(TemperatureInput { value: 100.0, unit: TemperatureUnit::Celsius })?;
assert_eq!(t.value(), "100 °C");

assert!(Temperature::new(TemperatureInput { value: -274.0, unit: TemperatureUnit::Celsius }).is_err());
```

---

## Volume

**Validation:** finite, non-negative. **Units:** `Ml`, `L`, `M3`, `FlOz`, `Gal`.

```rust,ignore
use arvo::measurement::{Volume, VolumeInput, VolumeUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let v = Volume::new(VolumeInput { value: 1.5, unit: VolumeUnit::L })?;
assert_eq!(v.value(), "1.5 l");
```

---

## Area

**Validation:** finite, non-negative. **Units:** `Mm2`, `Cm2`, `M2`, `Km2`, `In2`, `Ft2`, `Ha`.

```rust,ignore
use arvo::measurement::{Area, AreaInput, AreaUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let a = Area::new(AreaInput { value: 50.0, unit: AreaUnit::M2 })?;
assert_eq!(a.value(), "50 m²");
```

---

## Speed

**Validation:** finite, non-negative. **Units:** `Ms` (m/s), `Kmh` (km/h), `Mph`, `Kn` (knots).

```rust,ignore
use arvo::measurement::{Speed, SpeedInput, SpeedUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let s = Speed::new(SpeedInput { value: 120.0, unit: SpeedUnit::Kmh })?;
assert_eq!(s.value(), "120 km/h");
```

---

## Pressure

**Validation:** finite, non-negative. **Units:** `Pa`, `KPa`, `MPa`, `Bar`, `Psi`, `Atm`.

```rust,ignore
use arvo::measurement::{Pressure, PressureInput, PressureUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let p = Pressure::new(PressureInput { value: 101.325, unit: PressureUnit::KPa })?;
assert_eq!(p.value(), "101.325 kPa");
```

---

## Energy

**Validation:** finite, non-negative. **Units:** `J`, `KJ`, `MJ`, `KWh`, `Cal`, `Kcal`.

```rust,ignore
use arvo::measurement::{Energy, EnergyInput, EnergyUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let e = Energy::new(EnergyInput { value: 500.0, unit: EnergyUnit::Kcal })?;
assert_eq!(e.value(), "500 kcal");
```

---

## Power

**Validation:** finite, non-negative. **Units:** `W`, `KW`, `MW`, `Hp`.

```rust,ignore
use arvo::measurement::{Power, PowerInput, PowerUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let p = Power::new(PowerInput { value: 3.7, unit: PowerUnit::KW })?;
assert_eq!(p.value(), "3.7 kW");
```

---

## Frequency

**Validation:** finite, strictly positive (> 0). **Units:** `Hz`, `KHz`, `MHz`, `GHz`.

```rust,ignore
use arvo::measurement::{Frequency, FrequencyInput, FrequencyUnit};
use arvo::traits::{PrimitiveValue, ValueObject};

let f = Frequency::new(FrequencyInput { value: 2.4, unit: FrequencyUnit::GHz })?;
assert_eq!(f.value(), "2.4 GHz");

assert!(Frequency::new(FrequencyInput { value: 0.0, unit: FrequencyUnit::Hz }).is_err());
```
