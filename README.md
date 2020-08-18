# selectel-mks-rust: Rust SDK for the Selectel Managed Kubernetes Service

[![crates.io](https://img.shields.io/crates/v/selectel-mks.svg)](https://crates.io/crates/selectel-mks)
[![Documentation](https://docs.rs/selectel-mks/badge.svg)](https://docs.rs/selectel-mks)
![CI](https://github.com/ozerovandrei/selectel-mks-rust/workflows/CI/badge.svg?branch=master)

Pure Rust bindings to the Selectel MKS V1 API.

## Getting Started

Add `selectel-mks` to the `Cargo.toml`:

```toml
[dependencies]
selectel-mks = "0.2.0"
```

Prepare a new `Client` instance and use methods to work with the MKS API.

You can check `./examples` directory and also `./test` directory to see how `Client` methods are used to work with the MKS API.

## TLS

`selectel-mks` supports [rustls] and [rust-native-tls] for TLS connectivity.
`rustls` is used by default, but one can toggle support with Cargo features:

```toml
[dependencies.selectel-mks]
version = "0.2.0"
default-features = false
features = ["rust-native-tls"]
```

[rustls]: https://github.com/ctz/rustls
[rust-native-tls]: https://github.com/sfackler/rust-native-tls

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.