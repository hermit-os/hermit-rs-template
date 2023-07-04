# rusty-demo

An application template for [RustyHermit](https://github.com/hermitcore/rusty-hermit).


## Requirements

* [`rustup`](https://www.rust-lang.org/tools/install)
* [NASM](https://nasm.us/) (only for x86_64 with SMP)
* [QEMU](https://www.qemu.org/) or [Uhyve](https://github.com/hermitcore/uhyve) for running the application


## Transform your application into a RustyHermit image:

(this is already done in this repo)

*   Add the `hermit-sys` dependency for hermit targets in `Cargo.toml`.

    ```toml
    [target.'cfg(target_os = "hermit")'.dependencies]
    hermit-sys = "<version>"
    ```

*   Use the [exact Rust version] required by `hermit-sys` in `rust-toolchain.toml`.
    [exact Rust version]: rust-toolchain.toml#L2

    ```toml
    [toolchain]
    channel = "<version>"
    ```

*   Make sure we link against hermit in `main.rs`:

    ```rust
    #[cfg(target_os = "hermit")]
    use hermit_sys as _;
    ```


## Usage

### Install the Rust Standard Library for Rusty-Hermit

See [rust-std-hermit](https://github.com/hermitcore/rust-std-hermit).

### Build the Hermit Application

``` 
$ cargo build --target x86_64-unknown-hermit
```


### Run the Application in Uhyve

```
$ uhyve target/x86_64-unknown-hermit/release/hello_world
```

For more details, see [uhyve's README](https://github.com/hermitcore/uhyve/blob/master/README.md).


### Run the Application in QEMU

Download the rusty-loader binary from its [releases page](https://github.com/hermitcore/rusty-loader/releases).

```
$ qemu-system-x86_64 \
    -cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
    -smp 1 -m 64M \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none -serial stdio \
    -kernel rusty-loader-x86_64 \
    -initrd target/x86_64-unknown-hermit/release/hello_world
```

For more details, see the [loader's README](https://github.com/hermitcore/rusty-loader/blob/master/README.md).
