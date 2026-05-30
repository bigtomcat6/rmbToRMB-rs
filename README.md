# rmbToRMB-rs

Convert RMB/CNY amounts into Chinese uppercase financial text.

这个项目现在由两层组成：

1. **Pure Rust library**: 可直接被 Rust backend、CLI、WASM 或其他 binding 复用。
2. **N-API wrapper**: 保留原来的 Node.js 原生扩展能力。

## Rust usage

The recommended API is `to_rmb_upper_from_cents`, because it represents money as an integer number of cents and avoids floating-point precision issues.

```rust
use rmb_upper::to_rmb_upper_from_cents;

fn main() {
  let value = to_rmb_upper_from_cents(12_345).unwrap();
  assert_eq!(value, "壹佰贰拾叁元肆角伍分");
}
```

For decimal strings, use `to_rmb_upper_from_str`:

```rust
use rmb_upper::to_rmb_upper_from_str;

assert_eq!(to_rmb_upper_from_str("123.45").unwrap(), "壹佰贰拾叁元肆角伍分");
assert_eq!(to_rmb_upper_from_str("0.01").unwrap(), "零元壹分");
```

For cross-language bindings or platforms where large integers may lose precision, use `to_rmb_upper_from_cents_str`:

```rust
use rmb_upper::to_rmb_upper_from_cents_str;

assert_eq!(to_rmb_upper_from_cents_str("12345").unwrap(), "壹佰贰拾叁元肆角伍分");
```

The `f64` adapter exists for compatibility only:

```rust
use rmb_upper::to_rmb_upper_from_f64;

assert_eq!(to_rmb_upper_from_f64(123.45).unwrap(), "壹佰贰拾叁元肆角伍分");
```

Prefer the cents or string APIs for financial code.

## Supported range

The maximum supported integer part is:

```text
9999兆9999亿9999万9999
```

The maximum supported amount in cents is exposed as `MAX_CENTS`.

## Errors

The Rust APIs return `Result<String, RmbError>`. Possible error variants include:

- `NegativeAmount`
- `NonFiniteAmount`
- `InvalidFormat`
- `TooManyDecimalPlaces`
- `TooLarge`

## Node.js / N-API usage

The original `rmbToRmb(number)` API is still available for compatibility. It accepts a JavaScript number and rounds to the nearest cent.

```ts
import { rmbToRmb } from '@rs/rmb-to-rmb'

rmbToRmb(123.45) // "壹佰贰拾叁元肆角伍分"
```

For deterministic formatting, prefer string-based APIs:

```ts
import { rmbToRmbFromCents, rmbToRmbFromString } from '@rs/rmb-to-rmb'

rmbToRmbFromCents('12345') // "壹佰贰拾叁元肆角伍分"
rmbToRmbFromString('123.45') // "壹佰贰拾叁元肆角伍分"
```

## Building

### Rust library

```bash
cargo build
cargo test
```

### N-API wrapper

```bash
pnpm i
pnpm build
```

The N-API build enables the `napi` feature and compiles the Rust core into a Node.js native addon.

## Testing

```bash
cargo test
```

For JavaScript tests:

```bash
pnpm test:js
```

## Related projects

- [napi-rs](https://github.com/napi-rs/napi-rs)
