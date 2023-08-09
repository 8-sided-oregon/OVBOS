# OVBOS
one very bad os (true)

# Building
## Requirements
- `rust (nightly)`
- `cargo`
- `bootimage`
- `llvm-tools-preview`
- `cargo-xbuild`

## Building a boot image
```bash
cargo bootimage
```

This will generate a .bin file in `target/x86_64-ovbos/debug/bootimage-ovbos.bin` which can be booted by something such as QEMU reading it as a raw boot disk.

# Running
Running
```bash
cargo run
```
will build the boot image and run it in QEMU.

# Tests
Running
```bash
cargo xtest
```