# rusty-demo

Small *Hello World* demo based on the unikernel [RustyHermit](https://github.com/hermitcore/libhermit-rs).

Please read the README of [RustyHermit](https://github.com/hermitcore/libhermit-rs) for more information.


## Requirements

* [`rustup`](https://www.rust-lang.org/tools/install)
* [NASM](https://nasm.us/) (only for x86_64 with SMP)
* [QEMU](https://www.qemu.org/) or [uhyve](https://github.com/hermitcore/uhyve) for running the application


## Usage



### Build the Hermit Application

``` 
$ cargo build \
    -Zbuild-std=core,alloc,std,panic_abort \
    --target x86_64-unknown-hermit \
    --release
```


### Run the Application in uhyve

```
$ uhyve target/x86_64-unknown-hermit/release/hello_world
```


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

Arguments can be provided like this:

```
$ qemu-system-x86_64 ... -append "kernel-args -- app-args"
```


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
