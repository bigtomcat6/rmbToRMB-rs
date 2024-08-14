# rmbToRMB-rs
`Napi` to convert numbers into Chinese format based on `Rust`

这是一个基于`Rust`编写的数字转人民币大写形式的`N-API`库。

## Building

1. Using `pnpm` to install dependencies.

```bash
pnpm i
```

```bash
pnpm add --save-dev @types/node
```

2. And Using `pnpm build` to build the project.

```bash
pnpm build
```

The base code from `src/rmb_to_rmb.rs` will be compiled into `RS-rmbToRMB.SYSTEM_NAME.node`

So far, the `RS-rmbToRMB` has only been tested on `macOS`.


## Testing

Because the test code depends on `ts-jest`, you need to use `pnpm` to configure `ts-jest` first.

```bash
pnpm add --save-dev @types/node @types/jest jest ts-jest nzh
```


```bash
pnpm ts-jest config:init
```

Then, you can run the test with `pnpm test`.

```bash
pnpm test
```

## Related projects

- [napi-rs](https://github.com/napi-rs/napi-rs)

