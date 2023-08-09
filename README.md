# OVBOS
one very bad os (true)

# Building
## Requirements
- `rust (nightly)`
- `cargo`

## Building a boot image
```bash
cargo bootimage --target x86_64-ovbos.json
```

This will generate a .bin file in `target/x86_64-ovbos/debug/bootimage-ovbos.bin` which can be booted by something such as QEMU reading it as a raw boot disk.

# Running
After building, you can run 
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ovbos/debug/bootimage-ovbos.bin
```
to boot the OS in QEMU.