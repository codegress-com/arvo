# net module

Feature flag: `net`

```toml
[dependencies]
arvo = { version = "0.9", features = ["net"] }
```

---

## Url

A validated URL. Accepts `http`, `https`, `ftp`, `ftps`, `ws`, and `wss` schemes. Scheme and host are normalised to lowercase.

**Validation:** must be a valid URL with an allowed scheme and a host.

```rust,ignore
use arvo::net::Url;
use arvo::traits::ValueObject;

let url = Url::new("HTTPS://Example.COM/path".into())?;
assert_eq!(url.value(), "https://example.com/path");
assert_eq!(url.scheme(), "https");
assert_eq!(url.host(), "example.com");

let url: Url = "https://example.com".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"https://example.com/path"` |
| `scheme()` | `&str` | `"https"` |
| `host()` | `&str` | `"example.com"` |
| `into_inner()` | `String` | `"https://example.com/path"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"not-a-url"` | `ValidationError::InvalidFormat` |
| `"mailto:user@example.com"` | `ValidationError::InvalidFormat` (scheme not allowed) |

---

## Domain

A validated domain name without a scheme (e.g. `"example.com"`).

**Normalisation:** trimmed, lowercased.
**Validation:** at least two labels, each 1–63 alphanumeric/hyphen characters, not starting or ending with a hyphen.

```rust,ignore
use arvo::net::Domain;
use arvo::traits::ValueObject;

let domain = Domain::new("Example.COM".into())?;
assert_eq!(domain.value(), "example.com");

let domain: Domain = "api.example.com".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"example.com"` |
| `into_inner()` | `String` | `"example.com"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"localhost"` | `ValidationError::InvalidFormat` (single label) |
| `"-example.com"` | `ValidationError::InvalidFormat` (leading hyphen) |

---

## IpV4Address

A validated IPv4 address. Leading zeros in octets are rejected.

```rust,ignore
use arvo::net::IpV4Address;
use arvo::traits::ValueObject;

let ip = IpV4Address::new("192.168.1.1".into())?;
assert_eq!(ip.value(), "192.168.1.1");

let ip: IpV4Address = "10.0.0.1".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"192.168.1.1"` |
| `into_inner()` | `String` | `"192.168.1.1"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"256.0.0.1"` | `ValidationError::InvalidFormat` |
| `"192.168.001.001"` | `ValidationError::InvalidFormat` (leading zeros) |

---

## IpV6Address

A validated IPv6 address, normalised to canonical compressed lowercase form.

```rust,ignore
use arvo::net::IpV6Address;
use arvo::traits::ValueObject;

let ip = IpV6Address::new("2001:0db8::0001".into())?;
assert_eq!(ip.value(), "2001:db8::1");

let ip: IpV6Address = "::1".try_into()?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"2001:db8::1"` |
| `into_inner()` | `String` | `"2001:db8::1"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"not-an-ip"` | `ValidationError::InvalidFormat` |

---

## IpAddress

A validated IP address — IPv4 or IPv6. Tries IPv4 first, then IPv6.

```rust,ignore
use arvo::net::IpAddress;
use arvo::traits::ValueObject;

let ip = IpAddress::new("192.168.1.1".into())?;
assert!(ip.is_v4());

let ip = IpAddress::new("::1".into())?;
assert!(ip.is_v6());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"192.168.1.1"` |
| `is_v4()` | `bool` | `true` |
| `is_v6()` | `bool` | `false` |
| `into_inner()` | `String` | `"192.168.1.1"` |

---

## Port

A validated network port number in the range `1..=65535`. Port 0 is reserved and rejected.

```rust,ignore
use arvo::net::Port;
use arvo::traits::ValueObject;

let port = Port::new(8080)?;
assert_eq!(*port.value(), 8080);

assert!(Port::new(0).is_err());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&u16` | `8080` |
| `into_inner()` | `u16` | `8080` |

### Errors

| Input | Error |
|---|---|
| `0` | `ValidationError::InvalidFormat` |

---

## MacAddress

A validated MAC address, normalised to lowercase colon-separated hex. Accepts colon-separated, hyphen-separated, or Cisco dotted formats.

```rust,ignore
use arvo::net::MacAddress;
use arvo::traits::ValueObject;

let mac = MacAddress::new("AA:BB:CC:DD:EE:FF".into())?;
assert_eq!(mac.value(), "aa:bb:cc:dd:ee:ff");

// Also accepts hyphen and dotted formats
let mac = MacAddress::new("AA-BB-CC-DD-EE-FF".into())?;
let mac = MacAddress::new("AABB.CCDD.EEFF".into())?;
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"aa:bb:cc:dd:ee:ff"` |
| `into_inner()` | `String` | `"aa:bb:cc:dd:ee:ff"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"AA:BB:CC:DD:EE"` | `ValidationError::InvalidFormat` (5 groups) |
| `"GG:BB:CC:DD:EE:FF"` | `ValidationError::InvalidFormat` (invalid hex) |

---

## MimeType

A validated MIME type. Trimmed and lowercased. Parameters (e.g. `; charset=utf-8`) are accepted and preserved.

```rust,ignore
use arvo::net::MimeType;
use arvo::traits::ValueObject;

let mime = MimeType::new("image/png".into())?;
assert_eq!(mime.value(), "image/png");
assert_eq!(mime.type_part(), "image");
assert_eq!(mime.subtype(), "png");

let mime = MimeType::new("text/html; charset=utf-8".into())?;
assert_eq!(mime.subtype(), "html");
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"image/png"` |
| `type_part()` | `&str` | `"image"` |
| `subtype()` | `&str` | `"png"` |
| `into_inner()` | `String` | `"image/png"` |

### Errors

| Input | Error |
|---|---|
| `""` | `ValidationError::Empty` |
| `"imagepng"` | `ValidationError::InvalidFormat` (no slash) |
| `"image/"` | `ValidationError::InvalidFormat` (empty subtype) |

---

## HttpStatusCode

A validated HTTP status code in the range `100..=599`.

```rust,ignore
use arvo::net::HttpStatusCode;
use arvo::traits::ValueObject;

let code = HttpStatusCode::new(200)?;
assert!(code.is_success());

let code = HttpStatusCode::new(404)?;
assert!(code.is_client_error());
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&u16` | `200` |
| `is_informational()` | `bool` | `false` |
| `is_success()` | `bool` | `true` |
| `is_redirection()` | `bool` | `false` |
| `is_client_error()` | `bool` | `false` |
| `is_server_error()` | `bool` | `false` |
| `into_inner()` | `u16` | `200` |

### Errors

| Input | Error |
|---|---|
| `< 100` or `> 599` | `ValidationError::InvalidFormat` |

---

## ApiKey

A validated API key — non-empty string. `Display` shows a masked form with only the last 4 characters visible.

```rust,ignore
use arvo::net::ApiKey;
use arvo::traits::ValueObject;

let key = ApiKey::new("sk-1234567890abcd".into())?;
assert_eq!(key.value(), "sk-1234567890abcd");  // full key
assert_eq!(key.last_four(), "abcd");
println!("{key}");  // *************abcd
```

### Accessors

| Method | Returns | Example |
|---|---|---|
| `value()` | `&String` | `"sk-1234567890abcd"` (full) |
| `last_four()` | `&str` | `"abcd"` |
| `masked()` | `String` | `"*************abcd"` |
| `into_inner()` | `String` | `"sk-1234567890abcd"` |

### Errors

| Input | Error |
|---|---|
| `""` or whitespace only | `ValidationError::Empty` |
