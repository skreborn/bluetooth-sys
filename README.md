# bluetooth-sys

`bluetooth-sys` provides unsafe bindings to Linux's C bluetooth API.

## Requirements

This package generates bindings using [`rust-bindgen`](https://github.com/rust-lang/rust-bindgen) at build time. It needs, as a result, the
appropriate development libraries installed.

For Debian based systems, execute the following:

```sh
apt install bluetooth bluez libbluetooth-dev libudev-dev
```

**See also [`rust-bindgen`'s requirements](https://github.com/rust-lang/rust-bindgen/blob/master/book/src/requirements.md).

## Notes

This package currently does the **bare minimum**. It uses `rust-bindgen` to generate the bindings,
and not much else. An important shortcoming of `rust-bindgen` is its inability to parse and
reimplement inline functions and macros on its own.

These functions and macros are **not** currently implemented, but might be at any given point in the
future. If you need them for a project, please raise an issue - or better yet, submit a PR!
