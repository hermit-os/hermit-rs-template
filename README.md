# hermit-rs-template

An application template for [Hermit for Rust](https://github.com/hermit-os/hermit-rs).


## Requirements

* [`rustup`](https://www.rust-lang.org/tools/install)
* [QEMU](https://www.qemu.org/) or [Uhyve](https://github.com/hermit-os/uhyve) for running the application


## Transform your application into a Hermit image:

(this is already done in this repo)

*   Add the `hermit` dependency for hermit targets in `Cargo.toml`.

    ```bash
    cargo add --target 'cfg(target_os = "hermit")' hermit
    ```

*   Make sure we link against hermit in `main.rs`:

    ```rust
    #[cfg(target_os = "hermit")]
    use hermit as _;
    ```


## Usage

### Install the Rust Standard Library for Hermit

See [rust-std-hermit](https://github.com/hermit-os/rust-std-hermit).

Note that rust-std-hermit is not available for the `stable` toolchain channel but for specific channels of stable Rust versions.
See [rust-toolchain.toml](rust-toolchain.toml).

### Build the Hermit Application

``` 
$ cargo build --target x86_64-unknown-hermit
```


### Run the Application in Uhyve

```
$ uhyve target/x86_64-unknown-hermit/debug/hermit-rs-template
```

For more details, see [uhyve's README](https://github.com/hermit-os/uhyve/blob/master/README.md).


### Run the Application in QEMU

Download the loader binary from its [releases page](https://github.com/hermit-os/loader/releases).

```
$ qemu-system-x86_64 \
    -cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
    -smp 1 -m 128M \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none -serial stdio \
    -kernel hermit-loader-x86_64 \
    -initrd target/x86_64-unknown-hermit/debug/hermit-rs-template
```

For more details, see the [loader's README](https://github.com/hermit-os/loader/blob/master/README.md).
