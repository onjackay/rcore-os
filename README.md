https://github.com/rcore-os/rCore-Tutorial-v3

### Compile

```
cargo build --release
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os1 -O binary target/riscv64gc-unknown-none-elf/release/os.bin
```

### Run and wait for GBD

```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
```

### GDB

```
riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```
