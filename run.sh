#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

cargo +nightly build \
    --target x86_64-unknown-hermit \
    -Z build-std=std,panic_abort \
    --release

sudo qemu-system-x86_64 \
    -M microvm,x-option-roms=off,pit=off,pic=off,rtc=on,auto-kernel-cmdline=off,acpi=off \
    -accel kvm -cpu host \
    -smp 1 \
    -m 256M \
    -display none -serial stdio \
    -netdev user,id=net0,hostfwd=tcp::8080-:8080 \
    -device virtio-net-device,netdev=net0 \
    -global virtio-mmio.force-legacy=off \
    -kernel hermit-loader-x86_64 \
    -initrd target/x86_64-unknown-hermit/release/hermit-rs-template \
    -append "-freq 3250"
