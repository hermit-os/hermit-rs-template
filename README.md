# rusty-demo

Small *Hello World* demo based on the unikernel [RustyHermit](https://github.com/hermitcore/libhermit-rs).
Please install the latest Rust compiler and use following commands to build and to test the demo.

```sh
$ cargo install uhyve
$ cargo build -Z build-std=std,core,alloc --target x86_64-unknown-hermit
$ cargo run -Z build-std=std,core,alloc --target x86_64-unknown-hermit
```

Please read the README of [RustyHermit](https://github.com/hermitcore/libhermit-rs) for more information.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
