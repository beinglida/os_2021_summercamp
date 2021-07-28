get riscv64 tool chain

```shell-session
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain.git
cd riscv-gnu-toolchain
git submodule update --init --recursive
./configure --prefix=/opt/riscv64_1
sudo make linux
```

