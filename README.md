https://github.com/rcore-os/rCore-Tutorial-v3

### Compile

```
cd os
```
```
cargo build --release
```

Optional:
```
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os
```

### Run and wait for GBD

```
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os,addr=0x80200000 \
    -s -S
```

Remove `-s S` to run without gdb.

### GDB

```
riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

### User applications

To run user application on qemu-riscv64 simulator:

```
cd user
make build
cd target/riscv64gc-unknown-none-elf/release
qemu-riscv64 ./00hello_world
```
