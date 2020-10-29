# Install toolchain
```
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
cd riscv-gnu-toolchain
./configure --prefix=/opt/riscv --with-arch=rv64i --with-abi=lp64
make
```

# Build examples
`make`
